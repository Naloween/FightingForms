use spacetimedb::{reducer, table, Identity, ReducerContext, Table};

use crate::{
    action::{effect::*, get_action_effects},
    character::*,
    player::*,
    Position,
};

pub const NB_STEPS: u8 = 4;

#[table(name = game, public)]
pub struct Game {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub players: Vec<Identity>,
    pub size: u8,
    pub started: bool,
    pub round: u8,
    pub ending: bool,
    pub creator_id: Identity,
    pub max_nb_players: u8,
    pub round_effects: Vec<AppliedEffect>,
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
    log::info!("Creating game for player {:?}", ctx.sender);
    let player = ctx.db.player().id().find(ctx.sender).unwrap();
    if player.game_id.is_some() {
        return;
    }

    let game = ctx.db.game().insert(Game {
        id: 0,
        players: vec![ctx.sender],
        size: 10,
        started: false,
        round: 0,
        ending: false,
        creator_id: ctx.sender,
        max_nb_players: 4,
        round_effects: Vec::new(),
    });
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
                if game.started {
                    next_round(ctx, game);
                } else if game.players.len() >= 1 {
                    start_game(ctx, game);
                }
            }
        }
    }
}

pub fn start_game(ctx: &ReducerContext, game: Game) {
    let start_positions = vec![
        Position { x: 0, y: 0 },
        Position {
            x: game.size - 1,
            y: game.size - 1,
        },
        Position {
            x: 0,
            y: game.size - 1,
        },
        Position {
            x: game.size - 1,
            y: 0,
        },
    ];

    let mut k = 0;
    for player_id in &game.players {
        let player = ctx.db.player().id().find(player_id).unwrap();
        ctx.db.player().id().update(Player {
            ready: false,
            eliminated: false,
            ..player
        });

        let character = ctx
            .db
            .character()
            .id()
            .find(player.character_id.unwrap())
            .unwrap();

        ctx.db.character().id().update(Character {
            current_state: CharacterState {
                position: start_positions[k].clone(),
                ..character.current_state
            },
            ..character
        });
        k += 1;
    }

    ctx.db.game().id().update(Game {
        started: true,
        round: 1,
        ..game
    });
}

pub fn next_round(ctx: &ReducerContext, game: Game) {
    let mut round_effects: Vec<AppliedEffect> = Vec::new();

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
        let mut applied_effects = apply_effect_chain(ctx, effect_chain, 0);
        round_effects.append(&mut applied_effects);
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

            if character.current_state.hp > 0 {
                if let Some(action) = action {
                    effects.push(get_action_effects(character, action));
                }
            }
        }

        // Apply effects
        for effect_chain in effects {
            let mut applied_effects = apply_effect_chain(ctx, effect_chain, step + 1);
            round_effects.append(&mut applied_effects);
        }
    }

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
        round_effects,
        ..game
    });

    if game.ending {
        end_game(ctx, game.id);
    }
}

pub fn end_game(ctx: &ReducerContext, game_id: u64) {
    let game = ctx.db.game().id().find(game_id).unwrap();

    for player_id in game.players {
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

#[reducer]
pub fn quit_game(ctx: &ReducerContext) {
    let player = ctx.db.player().id().find(ctx.sender).unwrap();
    if player.game_id.is_none() {
        return;
    }

    let game = ctx.db.game().id().find(player.game_id.unwrap()).unwrap();
    let new_players: Vec<Identity> = game
        .players
        .clone()
        .into_iter()
        .filter(|&player_id| player_id != player.id)
        .collect();

    let nb_players_remaining = new_players.len();

    // Update player
    ctx.db.player().id().update(Player {
        game_id: Option::None,
        character_class_id: Option::None,
        character_id: Option::None,
        ready: false,
        eliminated: true,
        ..player
    });

    // Delete character
    if let Some(character_id) = player.character_id {
        ctx.db.character().id().delete(character_id);
    }

    // Update game
    ctx.db.game().id().update(Game {
        players: new_players,
        ..game
    });

    if nb_players_remaining == 0 || (game.started && nb_players_remaining == 1) {
        end_game(ctx, game.id);
    }
}
