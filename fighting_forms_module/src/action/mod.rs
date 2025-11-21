pub mod effect;

use spacetimedb::{table, SpacetimeType};

use crate::{
    action::effect::{
        ApplyStatusConfig, CostConfig, DamageTileConfig, Effect, MoveConfig, StatusTileConfig,
    },
    character::{
        Character, DamageReductionConfig, JaugeType, RefundOnDamageConfig, Status, StunnedConfig,
    },
    game::{Game, NB_STEPS},
    CardinalDirection, Direction, Position,
};
#[table(name=action_class_config, public)]
pub struct ActionClassConfig {
    #[primary_key]
    #[auto_inc]
    pub id: u16,
    #[unique]
    pub action_class: ActionClass,
    pub name: String,
    pub description: String,
    pub mana_cost: u8,
    pub stamina_cost: u8,
    pub cooldown: u8,
    pub duration: u8,
}

#[derive(Clone, SpacetimeType)]
pub enum ActionClass {
    Move,
    Zytex1,
    Zytex2,
    Zytex3,
    Bardass1,
    Bardass2,
    Bardass3,
    Stunlor1,
    Stunlor2,
    Stunlor3,
}

#[derive(SpacetimeType, Clone)]
pub enum Action {
    Move(MoveAction),
    Zytex1(Zytex1Action),
    Zytex2(Zytex2Action),
    Zytex3(Zytex3Action),
    Bardass1(Bardass1Action),
    Bardass2(Bardass2Action),
    Bardass3(Bardass3Action),
    Stunlor1(Stunlor1Action),
    Stunlor2(Stunlor2Action),
    Stunlor3(Stunlor3Action),
}

#[derive(SpacetimeType, Clone)]
pub struct MoveAction {
    pub direction: Direction,
}

#[derive(SpacetimeType, Clone)]
pub struct Zytex1Action {}

#[derive(SpacetimeType, Clone)]
pub struct Zytex2Action {
    pub direction: Direction,
}

#[derive(SpacetimeType, Clone)]
pub struct Zytex3Action {
    pub direction: Direction,
}

#[derive(SpacetimeType, Clone)]
pub struct Bardass1Action {}

#[derive(SpacetimeType, Clone)]
pub struct Bardass2Action {
    pub direction: CardinalDirection,
}

#[derive(SpacetimeType, Clone)]
pub struct Bardass3Action {}

#[derive(SpacetimeType, Clone)]
pub struct Stunlor1Action {
    pub direction: CardinalDirection,
}

#[derive(SpacetimeType, Clone)]
pub struct Stunlor2Action {
    pub direction: CardinalDirection,
}

#[derive(SpacetimeType, Clone)]
pub struct Stunlor3Action {
    pub direction: CardinalDirection,
}

// TODO
pub fn get_action_effects(game: &Game, character: Character, action: Action) -> Vec<Effect> {
    let character_id = character.id;
    let step = game.step;
    match action {
        Action::Move(move_action) => {
            vec![
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Stamina,
                    amount: 1,
                }),
                Effect::Move(MoveConfig {
                    character_id,
                    direction: move_action.direction,
                    distance: 1,
                }),
            ]
        }
        Action::Zytex1(_zytex1_action) => vec![
            Effect::Cost(CostConfig {
                character_id,
                jauge_type: JaugeType::Mana,
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: crate::Position {
                    x: character.current_state.position.x,
                    y: character.current_state.position.y - 1,
                },
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: crate::Position {
                    x: character.current_state.position.x + 1,
                    y: character.current_state.position.y,
                },
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: crate::Position {
                    x: character.current_state.position.x,
                    y: character.current_state.position.y + 1,
                },
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: crate::Position {
                    x: character.current_state.position.x - 1,
                    y: character.current_state.position.y,
                },
                amount: 1,
            }),
        ],
        Action::Zytex2(zytex2_action) => {
            let (delta_x, delta_y) = zytex2_action.direction.to_delta();

            let mut effects = vec![Effect::Cost(CostConfig {
                character_id,
                jauge_type: JaugeType::Mana,
                amount: 3,
            })];
            for k in 1..4 {
                effects.push(Effect::DamageTile(DamageTileConfig {
                    position: crate::Position {
                        x: (character.current_state.position.x as i8 + k * delta_x) as u8,
                        y: (character.current_state.position.y as i8 + k * delta_y) as u8,
                    },
                    amount: 1,
                }));
            }
            // straight line
            if delta_x * delta_y == 0 {
                effects.push(Effect::DamageTile(DamageTileConfig {
                    position: crate::Position {
                        x: (character.current_state.position.x as i8 + 4 * delta_x) as u8,
                        y: (character.current_state.position.y as i8 + 4 * delta_y) as u8,
                    },
                    amount: 2,
                }));
            }
            effects
        }
        Action::Zytex3(zytex3_action) => {
            let (delta_x, delta_y) = zytex3_action.direction.to_delta();
            vec![
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Mana,
                    amount: 2,
                }),
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Stamina,
                    amount: 1,
                }),
                Effect::Move(MoveConfig {
                    character_id,
                    direction: zytex3_action.direction,
                    distance: 2,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i8 + delta_x) as u8,
                        y: (character.current_state.position.y as i8 + delta_y) as u8,
                    },
                    amount: 1,
                }),
            ]
        }
        Action::Bardass1(_bardass1_action) => vec![
            Effect::Cost(CostConfig {
                character_id,
                jauge_type: JaugeType::Mana,
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: Position {
                    x: character.current_state.position.x - 1,
                    y: character.current_state.position.y - 1,
                },
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: Position {
                    x: character.current_state.position.x + 1,
                    y: character.current_state.position.y - 1,
                },
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: Position {
                    x: character.current_state.position.x + 1,
                    y: character.current_state.position.y + 1,
                },
                amount: 1,
            }),
            Effect::DamageTile(DamageTileConfig {
                position: Position {
                    x: character.current_state.position.x - 1,
                    y: character.current_state.position.y + 1,
                },
                amount: 1,
            }),
        ],
        Action::Bardass2(bardass2_action) => {
            let (delta_x, delta_y) = bardass2_action.direction.to_delta();
            let (delta_x_left, delta_y_left) = bardass2_action.direction.rotate(1).to_delta();
            let (delta_x_right, delta_y_right) = bardass2_action.direction.rotate(-1).to_delta();
            vec![
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Mana,
                    amount: 2,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i8 + 1 * delta_x) as u8,
                        y: (character.current_state.position.y as i8 + 1 * delta_y) as u8,
                    },
                    amount: 2,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i8 + 2 * delta_x) as u8,
                        y: (character.current_state.position.y as i8 + 2 * delta_y) as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i8 + 3 * delta_x) as u8,
                        y: (character.current_state.position.y as i8 + 3 * delta_y) as u8,
                    },
                    amount: 2,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i8
                            + 2 * delta_x
                            + 1 * delta_x_left) as u8,
                        y: (character.current_state.position.y as i8
                            + 2 * delta_y
                            + 1 * delta_y_left) as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i8
                            + 2 * delta_x
                            + 1 * delta_x_right) as u8,
                        y: (character.current_state.position.y as i8
                            + 2 * delta_y
                            + 1 * delta_y_right) as u8,
                    },
                    amount: 1,
                }),
            ]
        }
        Action::Bardass3(_bardass3_action) => vec![
            Effect::Cost(CostConfig {
                character_id,
                jauge_type: JaugeType::Mana,
                amount: 4,
            }),
            Effect::ApplyStatus(ApplyStatusConfig {
                character_id,
                status: Status::DamageReduction(DamageReductionConfig {
                    amount: 1,
                    duration: NB_STEPS - step,
                    only_once: false,
                }),
            }),
            Effect::ApplyStatus(ApplyStatusConfig {
                character_id,
                status: Status::RefundOnDamage(RefundOnDamageConfig {
                    jauge_type: JaugeType::Mana,
                    only_once: true,
                    amount: 1,
                    duration: NB_STEPS - step,
                }),
            }),
        ],
        Action::Stunlor1(stunlor1_action) => {
            let (delta_x, delta_y) = stunlor1_action.direction.to_delta();
            vec![
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Mana,
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i8 + delta_x) as u8,
                        y: (character.current_state.position.y as i8 + delta_y) as u8,
                    },
                    amount: 3,
                }),
            ]
        }
        // Missing few effects for shield, removing passives and immunity
        Action::Stunlor2(stunlor2_action) => {
            let (delta_x, delta_y) = stunlor2_action.direction.to_delta();
            let (delta_x_left, delta_y_left) = stunlor2_action.direction.rotate(1).to_delta();
            let (delta_x_right, delta_y_right) = stunlor2_action.direction.rotate(-1).to_delta();
            vec![
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Mana,
                    amount: 3,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x + 4 * delta_x as u8,
                        y: character.current_state.position.y + 4 * delta_y as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x
                            + 3 * delta_x as u8
                            + 1 * delta_x_left as u8,
                        y: character.current_state.position.y
                            + 3 * delta_y as u8
                            + 1 * delta_y_left as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x
                            + 3 * delta_x as u8
                            + 1 * delta_x_right as u8,
                        y: character.current_state.position.y
                            + 3 * delta_y as u8
                            + 1 * delta_y_right as u8,
                    },
                    amount: 1,
                }),
                Effect::StatusTile(StatusTileConfig {
                    position: Position {
                        x: character.current_state.position.x + 3 * delta_x as u8,
                        y: character.current_state.position.y + 3 * delta_y as u8,
                    },
                    status: Status::Stunned(StunnedConfig {
                        duration: 2 * NB_STEPS,
                    }),
                }),
            ]
        }
        Action::Stunlor3(stunlor3_action) => {
            let (delta_x, delta_y) = stunlor3_action.direction.to_delta();
            let (delta_x_left, delta_y_left) = stunlor3_action.direction.rotate(1).to_delta();
            let (delta_x_right, delta_y_right) = stunlor3_action.direction.rotate(-1).to_delta();
            vec![
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Mana,
                    amount: 2,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x + 1 * delta_x as u8,
                        y: character.current_state.position.y + 1 * delta_y as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x + 2 * delta_x as u8,
                        y: character.current_state.position.y + 2 * delta_y as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x
                            + 1 * delta_x as u8
                            + 1 * delta_x_left as u8,
                        y: character.current_state.position.y
                            + 1 * delta_y as u8
                            + 1 * delta_y_left as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x
                            + 2 * delta_x as u8
                            + 1 * delta_x_left as u8,
                        y: character.current_state.position.y
                            + 2 * delta_y as u8
                            + 1 * delta_y_left as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x
                            + 1 * delta_x as u8
                            + 1 * delta_x_right as u8,
                        y: character.current_state.position.y
                            + 1 * delta_y as u8
                            + 1 * delta_y_right as u8,
                    },
                    amount: 1,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: character.current_state.position.x
                            + 2 * delta_x as u8
                            + 1 * delta_x_right as u8,
                        y: character.current_state.position.y
                            + 2 * delta_y as u8
                            + 1 * delta_y_right as u8,
                    },
                    amount: 1,
                }),
            ]
        }
    }
}
