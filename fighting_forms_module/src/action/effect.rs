use spacetimedb::{Identity, ReducerContext, SpacetimeType};

use crate::{
    character::{character as character_trait, Character, CharacterState, JaugeType, Status},
    game::{game, Game},
    player::{player, Player},
    Direction, Position,
};

#[derive(SpacetimeType, Clone)]
pub struct AppliedEffect {
    effect: Effect,
    applied: bool,
    step: u8,
}

#[derive(SpacetimeType, Clone)]
pub enum Effect {
    Cost(CostConfig),
    Restore(RestoreConfig),
    Move(MoveConfig),
    Teleport(TeleportConfig),
    DamageTile(DamageTileConfig),
    StatusTile(StatusTileConfig),
    ApplyStatus(ApplyStatusConfig),
}

#[derive(SpacetimeType, Clone)]
pub struct CostConfig {
    pub character_id: u64,
    pub jauge_type: JaugeType,
    pub amount: u8,
}

#[derive(SpacetimeType, Clone)]
pub struct RestoreConfig {
    pub character_id: u64,
    pub jauge_type: JaugeType,
    pub amount: u8,
}

#[derive(SpacetimeType, Clone)]
pub struct MoveConfig {
    pub character_id: u64,
    pub direction: Direction,
    pub distance: u8,
}

#[derive(SpacetimeType, Clone)]
pub struct TeleportConfig {
    pub character_id: u64,
    pub position: Position,
}

#[derive(SpacetimeType, Clone)]
pub struct DamageTileConfig {
    pub position: Position,
    pub amount: u8,
}

#[derive(SpacetimeType, Clone)]
pub struct StatusTileConfig {
    pub position: Position,
    pub status: Status,
}

#[derive(SpacetimeType, Clone)]
pub struct ApplyStatusConfig {
    pub character_id: u64,
    pub status: Status,
}

pub fn apply_effect_chain(
    ctx: &ReducerContext,
    effect_chain: Vec<Effect>,
    step: u8,
) -> Vec<AppliedEffect> {
    let mut applied_effects: Vec<AppliedEffect> = Vec::with_capacity(effect_chain.len());
    let mut continue_chain: bool = true;

    for effect in effect_chain {
        if continue_chain {
            let applied = match effect.clone() {
                Effect::Cost(cost_config) => cost_effect(ctx, cost_config),
                Effect::Restore(restore_config) => restore_effect(ctx, restore_config),
                Effect::Move(move_config) => move_effect(ctx, move_config),
                Effect::Teleport(teleport_config) => teleport_effect(ctx, teleport_config),
                Effect::DamageTile(damage_tile_config) => {
                    damage_tile_effect(ctx, damage_tile_config)
                }
                Effect::StatusTile(status_tile_config) => {
                    status_tile_effect(ctx, status_tile_config)
                }
                Effect::ApplyStatus(apply_status_config) => {
                    apply_status_effect(ctx, apply_status_config)
                }
            };
            applied_effects.push(AppliedEffect {
                effect,
                applied,
                step,
            });
            if !applied {
                continue_chain = false;
            }
        } else {
            applied_effects.push(AppliedEffect {
                effect,
                applied: false,
                step,
            });
        }
    }

    return applied_effects;
}

pub fn cost_effect(ctx: &ReducerContext, config: CostConfig) -> bool {
    let character = ctx.db.character().id().find(config.character_id).unwrap();

    return match config.jauge_type {
        JaugeType::HP => {
            if character.current_state.hp < config.amount {
                false
            } else {
                apply_damage_character(ctx, character, config.amount);
                return true;
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

    return teleport_effect(
        ctx,
        TeleportConfig {
            character_id: config.character_id,
            position: new_position,
        },
    );
}

pub fn teleport_effect(ctx: &ReducerContext, config: TeleportConfig) -> bool {
    let character = ctx.db.character().id().find(config.character_id).unwrap();

    // Check collision with other character
    for other_character in ctx.db.character().game_id().filter(character.game_id) {
        if other_character.current_state.position == config.position {
            return false;
        }
    }

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
    let player = ctx.db.player().id().find(ctx.sender).unwrap();
    for character in ctx.db.character().game_id().filter(player.game_id.unwrap()) {
        if character.current_state.position == config.position {
            apply_damage_character(ctx, character, config.amount);
        }
    }

    return true;
}

pub fn status_tile_effect(ctx: &ReducerContext, config: StatusTileConfig) -> bool {
    // Apply status to any character on a the tile
    let player = ctx.db.player().id().find(ctx.sender).unwrap();
    for character in ctx.db.character().game_id().filter(player.game_id.unwrap()) {
        if character.current_state.position == config.position {
            apply_status_effect(
                ctx,
                ApplyStatusConfig {
                    character_id: character.id,
                    status: config.status.clone(),
                },
            );
        }
    }

    return true;
}

pub fn apply_status_effect(ctx: &ReducerContext, config: ApplyStatusConfig) -> bool {
    let character = ctx.db.character().id().find(config.character_id).unwrap();
    ctx.db.character().id().update(Character {
        current_state: CharacterState {
            status: {
                let mut new_status = character.current_state.status.clone();
                new_status.push(config.status.clone());
                new_status
            },
            ..character.current_state
        },
        ..character
    });

    return true;
}

// Utility functions

fn apply_damage_character(ctx: &ReducerContext, character: Character, damage: u8) {
    let mut new_damage = damage as i32;

    let mut new_statuses: Vec<Status> = Vec::new();
    for status in character.current_state.status.iter() {
        if let Status::DamageReduction(damage_reduction_config) = status {
            new_damage = 0.max(new_damage - damage_reduction_config.amount as i32);
            if !damage_reduction_config.only_once {
                new_statuses.push(status.clone());
            }
        } else if let Status::RefundOnDamage(refund_on_damage_config) = status {
            restore_effect(
                ctx,
                RestoreConfig {
                    character_id: character.id,
                    jauge_type: refund_on_damage_config.jauge_type,
                    amount: refund_on_damage_config.amount,
                },
            );
            if !refund_on_damage_config.only_once {
                new_statuses.push(status.clone());
            }
        } else {
            new_statuses.push(status.clone());
        }
    }

    let mut new_hp = character.current_state.hp as i32 - new_damage as i32;

    if new_hp <= 0 {
        new_hp = 0;
        eliminate_player(ctx, character.player_id);
    }

    ctx.db.character().id().update(Character {
        current_state: CharacterState {
            hp: new_hp as u8,
            status: new_statuses,
            ..character.current_state
        },
        ..character
    });
}

fn eliminate_player(ctx: &ReducerContext, player_id: Identity) {
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

    if nb_alive_players <= 1 {
        ctx.db.game().id().update(Game {
            ending: true,
            ..game
        });
    }
}
