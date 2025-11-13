use spacetimedb::{reducer, table, Identity, ReducerContext, Table};

use crate::{
    action::{effect::*, get_action_effects},
    character::*,
    player::*,
};

pub const NB_STEPS: u8 = 4;

#[table(name = game, public)]
#[derive(Clone)]
pub struct Game {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub players: Vec<Identity>,
    pub started: bool,
    pub round: u8,
    pub ending: bool,
}

#[reducer]
pub fn select_game(ctx: &ReducerContext, game_id: u64) {
    if let Some(game) = ctx.db.game().id().find(&game_id) {
        if !game.started && !game.players.contains(&ctx.sender) {
            let mut new_players = game.players.clone();
            new_players.push(ctx.sender);
            ctx.db.game().id().update(Game {
                players: new_players,
                ..game
            });

            let player = ctx.db.player().id().find(ctx.sender).unwrap();
            ctx.db.player().id().update(Player {
                game_id: Some(game.id),
                ..player
            });
        }
    }
}

#[reducer]
pub fn create_game(ctx: &ReducerContext) {
    let game = ctx.db.game().insert(Game {
        id: 0,
        players: vec![ctx.sender],
        started: false,
        round: 0,
        ending: false,
    });
    let player = ctx.db.player().id().find(ctx.sender).unwrap();
    ctx.db.player().id().update(Player {
        game_id: Some(game.id),
        ..player
    });
}

#[reducer]
pub fn ready(ctx: &ReducerContext, is_ready: bool) {
    let player = ctx.db.player().id().find(&ctx.sender).unwrap();
    if let Some(game_id) = player.game_id {
        let game = ctx.db.game().id().find(&game_id).unwrap();

        if game.started {
            // If in game, set player to ready
            ctx.db.player().id().update(Player {
                ready: is_ready,
                ..player
            });
        } else {
            // If in lobby check that character is selected
            if let Some(_) = player.character_class_id {
                ctx.db.player().id().update(Player {
                    ready: is_ready,
                    ..player
                });
            }
        }

        if is_ready {
            // check if everyone is ready
            let all_ready = game.players.iter().all(|player_id| {
                let p = ctx.db.player().id().find(player_id).unwrap();
                p.ready
            });

            if all_ready {
                // If game started finish round
                if game.started {
                    next_round(ctx, game);
                } else {
                    start_game(ctx, game);
                }
            }
        }
    }
}

pub fn start_game(ctx: &ReducerContext, game: Game) {
    for player_id in &game.players {
        let player = ctx.db.player().id().find(player_id).unwrap();
        ctx.db.player().id().update(Player {
            ready: false,
            eliminated: false,
            ..player
        });
    }

    ctx.db.game().id().update(Game {
        started: true,
        round: 1,
        ..game
    });
}

pub fn next_round(ctx: &ReducerContext, game: Game) {
    // Start round effects
    let mut effects: Vec<Vec<Effect>> = vec![];
    for player_id in &game.players {
        let player = ctx.db.player().id().find(player_id).unwrap();
        effects.push(vec![
            Effect::Restore(RestoreConfig {
                character_id: player.character_id.unwrap(),
                jauge_type: JaugeType::Stamina,
                amount: 100,
            }),
            Effect::Restore(RestoreConfig {
                character_id: player.character_id.unwrap(),
                jauge_type: JaugeType::Mana,
                amount: 100,
            }),
        ]);
    }
    for effect_chain in effects {
        apply_effect_chain(ctx, effect_chain);
    }

    // Apply actions at each step
    for step in 0..NB_STEPS {
        update_character_step_state(ctx, step as usize);

        // Generate effects from actions
        let mut effects: Vec<Vec<Effect>> = Vec::new();
        for player_id in &game.players {
            let player = ctx.db.player().id().find(player_id).unwrap();
            let character = ctx
                .db
                .character()
                .id()
                .find(player.character_id.unwrap())
                .unwrap();
            let action = character
                .choosen_actions
                .get(step as usize)
                .cloned()
                .unwrap();
            if let Some(action) = action {
                effects.push(get_action_effects(character, action));
            }
        }

        // Apply effects
        log::info!("Apply effects...");
        for effect_chain in effects {
            apply_effect_chain(ctx, effect_chain);
        }
    }

    // Check if game still exists
    for player_id in &game.players {
        // Unready players
        let player = ctx.db.player().id().find(player_id).unwrap();
        ctx.db.player().id().update(Player {
            ready: false,
            ..player
        });

        // Remove action selections
        let character = ctx
            .db
            .character()
            .id()
            .find(player.character_id.unwrap())
            .unwrap();
        ctx.db.character().id().update(Character {
            choosen_actions: vec![Option::None; NB_STEPS as usize],
            ..character
        });
    }

    // Update Game
    let game = ctx.db.game().id().find(game.id).unwrap();

    ctx.db.game().id().update(Game {
        round: game.round + 1,
        ..game.clone()
    });

    log::info!("Game ending: {:?}", game.ending);
    if game.ending {
        end_game(ctx, &game);
    }
}

pub fn end_game(ctx: &ReducerContext, game: &Game) {
    for player_id in &game.players {
        let player = ctx.db.player().id().find(player_id).unwrap();

        // Delete character
        if let Some(character_id) = player.character_id {
            ctx.db.character().id().delete(character_id);
        }

        // Update player
        ctx.db.player().id().update(Player {
            game_id: Option::None,
            character_class_id: Option::None,
            character_id: Option::None,
            ready: false,
            ..player
        });
    }
    ctx.db.game().id().delete(&game.id);
}

pub fn quit_game(ctx: &ReducerContext) {
    let player = ctx.db.player().id().find(ctx.sender).unwrap();
    let game = ctx.db.game().id().find(player.game_id.unwrap()).unwrap();
    let new_players: Vec<Identity> = game
        .players
        .clone()
        .into_iter()
        .filter(|&player_id| player_id != player.id)
        .collect();

    if new_players.len() <= 1 {
        end_game(ctx, &game);
    } else {
        // Update game
        ctx.db.game().id().update(Game {
            players: new_players,
            ..game
        });

        // Delete character
        if let Some(character_id) = player.character_id {
            ctx.db.character().id().delete(character_id);
        }

        // Update player
        ctx.db.player().id().update(Player {
            game_id: Option::None,
            character_class_id: Option::None,
            character_id: Option::None,
            ready: false,
            eliminated: false,
            ..player
        });
    }
}
