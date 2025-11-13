use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

use crate::{
    character::{character as character_trait, Character, CharacterState, JaugeType},
    game::{game, Game},
    player::{player, Player},
    Direction, Position,
};

#[derive(SpacetimeType)]
pub enum Effect {
    Cost(CostConfig),
    Restore(RestoreConfig),
    Move(MoveConfig),
    Teleport(TeleportConfig),
    DamageTile(DamageTileConfig),
}

#[derive(SpacetimeType)]
pub struct CostConfig {
    pub character_id: u64,
    pub jauge_type: JaugeType,
    pub amount: u8,
}

#[derive(SpacetimeType)]
pub struct RestoreConfig {
    pub character_id: u64,
    pub jauge_type: JaugeType,
    pub amount: u8,
}

#[derive(SpacetimeType)]
pub struct MoveConfig {
    pub character_id: u64,
    pub direction: Direction,
    pub distance: u8,
}

#[derive(SpacetimeType)]
pub struct TeleportConfig {
    pub character_id: u64,
    pub position: Position,
}

#[derive(SpacetimeType)]
pub struct DamageTileConfig {
    pub position: Position,
    pub amount: u8,
}

pub fn apply_effect_chain(ctx: &ReducerContext, effect_chain: Vec<Effect>) {
    for effect in effect_chain {
        let applied = match effect {
            Effect::Cost(cost_config) => cost_effect(ctx, cost_config),
            Effect::Restore(restore_config) => restore_effect(ctx, restore_config),
            Effect::Move(move_config) => move_effect(ctx, move_config),
            Effect::Teleport(teleport_config) => teleport_effect(ctx, teleport_config),
            Effect::DamageTile(damage_tile_config) => damage_tile_effect(ctx, damage_tile_config),
        };

        if !applied {
            return;
        }
    }
}

pub fn cost_effect(ctx: &ReducerContext, config: CostConfig) -> bool {
    let character = ctx.db.character().id().find(config.character_id).unwrap();

    return match config.jauge_type {
        JaugeType::HP => {
            if character.current_state.hp < config.amount {
                false
            } else {
                ctx.db.character().id().update(Character {
                    current_state: CharacterState {
                        hp: character.current_state.hp - config.amount,
                        ..character.current_state
                    },
                    ..character
                });
                true
            }
        }
        JaugeType::Mana => {
            if character.current_state.mana < config.amount {
                false
            } else {
                ctx.db.character().id().update(Character {
                    current_state: CharacterState {
                        mana: character.current_state.mana - config.amount,
                        ..character.current_state
                    },
                    ..character
                });
                true
            }
        }
        JaugeType::Stamina => {
            if character.current_state.stamina < config.amount {
                false
            } else {
                ctx.db.character().id().update(Character {
                    current_state: CharacterState {
                        stamina: character.current_state.stamina - config.amount,
                        ..character.current_state
                    },
                    ..character
                });
                true
            }
        }
    };
}

pub fn restore_effect(ctx: &ReducerContext, config: RestoreConfig) -> bool {
    let character = ctx.db.character().id().find(config.character_id).unwrap();

    return match config.jauge_type {
        JaugeType::HP => {
            ctx.db.character().id().update(Character {
                current_state: CharacterState {
                    hp: (character.current_state.hp + config.amount)
                        .min(character.current_state.max_hp),
                    ..character.current_state
                },
                ..character
            });
            true
        }
        JaugeType::Mana => {
            ctx.db.character().id().update(Character {
                current_state: CharacterState {
                    mana: (character.current_state.mana + config.amount)
                        .min(character.current_state.max_mana),
                    ..character.current_state
                },
                ..character
            });
            true
        }
        JaugeType::Stamina => {
            ctx.db.character().id().update(Character {
                current_state: CharacterState {
                    stamina: (character.current_state.stamina + config.amount)
                        .min(character.current_state.max_stamina),
                    ..character.current_state
                },
                ..character
            });
            true
        }
    };
}

pub fn move_effect(ctx: &ReducerContext, config: MoveConfig) -> bool {
    log::info!("Start move effect");
    // Just teleport for now, but TODO check along path
    let character = ctx.db.character().id().find(config.character_id).unwrap();

    let mut delta_x: i32 = 0;
    let mut delta_y: i32 = 0;
    if config.direction == Direction::NorthEast
        || config.direction == Direction::East
        || config.direction == Direction::SouthEast
    {
        delta_x += 1;
    }
    if config.direction == Direction::NorthEast
        || config.direction == Direction::North
        || config.direction == Direction::NorthWest
    {
        delta_y -= 1;
    }
    if config.direction == Direction::NorthWest
        || config.direction == Direction::West
        || config.direction == Direction::SouthWest
    {
        delta_x -= 1;
    }
    if config.direction == Direction::SouthEast
        || config.direction == Direction::South
        || config.direction == Direction::SouthWest
    {
        delta_y += 1;
    }

    let new_position = Position {
        x: (character.current_state.position.x as i32 + delta_x * config.distance as i32) as u8,
        y: (character.current_state.position.y as i32 + delta_y * config.distance as i32) as u8,
    };
    log::info!("position y: {:?}", new_position.y);

    return teleport_effect(
        ctx,
        TeleportConfig {
            character_id: config.character_id,
            position: new_position,
        },
    );
}

pub fn teleport_effect(ctx: &ReducerContext, config: TeleportConfig) -> bool {
    log::info!("Start teleport effect");

    let character = ctx.db.character().id().find(config.character_id).unwrap();

    // Check collision with other character
    for other_character in ctx.db.character().iter() {
        if other_character.current_state.position == config.position {
            return false;
        }
    }
    log::info!("Teleporting character");

    // Teleport character
    ctx.db.character().id().update(Character {
        current_state: CharacterState {
            position: config.position,
            ..character.current_state
        },
        ..character
    });

    return true;
}

pub fn damage_tile_effect(ctx: &ReducerContext, config: DamageTileConfig) -> bool {
    // Apply damage to any character on a the tile
    for character in ctx.db.character().iter() {
        if character.game_id
            == ctx
                .db
                .player()
                .id()
                .find(ctx.sender)
                .unwrap()
                .game_id
                .unwrap()
            && character.current_state.position == config.position
        {
            apply_damage_character(ctx, character, config.amount);
        }
    }

    return true;
}

fn apply_damage_character(ctx: &ReducerContext, character: Character, damage: u8) {
    let mut new_hp = character.current_state.hp as i32 - damage as i32;

    if new_hp <= 0 {
        new_hp = 0;
        eliminate_player(ctx, character.player_id);
    }

    ctx.db.character().id().update(Character {
        current_state: CharacterState {
            hp: new_hp as u8,
            ..character.current_state
        },
        ..character
    });
}

fn eliminate_player(ctx: &ReducerContext, player_id: Identity) {
    log::info!("Eliminate player");

    let player = ctx.db.player().id().find(player_id).unwrap();

    // Update player status
    ctx.db.player().id().update(Player {
        eliminated: true,
        ..player
    });

    // End game if all players except one is eliminated
    let game = ctx.db.game().id().find(player.game_id.unwrap()).unwrap();

    let mut nb_alive_players = 0;
    for player_id in game.players.iter() {
        let player = ctx.db.player().id().find(player_id).unwrap();
        if !player.eliminated {
            nb_alive_players += 1;
        }
    }
    log::info!("Alive players: {:?}", nb_alive_players);

    if nb_alive_players <= 1 {
        log::info!("End game");
        ctx.db.game().id().update(Game {
            ending: true,
            ..game
        });
    }
}
