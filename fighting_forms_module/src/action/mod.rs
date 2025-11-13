pub mod effect;

use spacetimedb::{table, SpacetimeType};

use crate::{
    action::effect::{CostConfig, DamageTileConfig, Effect, MoveConfig},
    character::{Character, JaugeType},
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
pub fn get_action_effects(character: Character, action: Action) -> Vec<Effect> {
    let character_id = character.id;
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
            let (delta_x, delta_y, range) = match zytex2_action.direction {
                Direction::North => (0, -1, 4),
                Direction::NorthEast => (1, -1, 3),
                Direction::East => (1, 0, 4),
                Direction::SouthEast => (1, 1, 3),
                Direction::South => (0, 1, 4),
                Direction::SouthWest => (-1, 1, 3),
                Direction::West => (-1, 0, 4),
                Direction::NorthWest => (-1, -1, 3),
            };

            let mut effects = vec![Effect::Cost(CostConfig {
                character_id,
                jauge_type: JaugeType::Mana,
                amount: 3,
            })];
            for k in 1..4 {
                effects.push(Effect::DamageTile(DamageTileConfig {
                    position: crate::Position {
                        x: (character.current_state.position.x as i32 + k * delta_x) as u8,
                        y: (character.current_state.position.y as i32 + k * delta_y) as u8,
                    },
                    amount: 1,
                }));
            }
            if range == 4 {
                effects.push(Effect::DamageTile(DamageTileConfig {
                    position: crate::Position {
                        x: (character.current_state.position.x as i32 + 4 * delta_x) as u8,
                        y: (character.current_state.position.y as i32 + 4 * delta_y) as u8,
                    },
                    amount: 2,
                }));
            }
            effects
        }
        Action::Zytex3(zytex3_action) => {
            let (delta_x, delta_y) = match zytex3_action.direction {
                Direction::North => (0, -1),
                Direction::NorthEast => (1, -1),
                Direction::East => (1, 0),
                Direction::SouthEast => (1, 1),
                Direction::South => (0, 1),
                Direction::SouthWest => (-1, 1),
                Direction::West => (-1, 0),
                Direction::NorthWest => (-1, -1),
            };
            vec![
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Stamina,
                    amount: 1,
                }),
                Effect::Cost(CostConfig {
                    character_id,
                    jauge_type: JaugeType::Mana,
                    amount: 2,
                }),
                Effect::Move(MoveConfig {
                    character_id,
                    direction: zytex3_action.direction,
                    distance: 2,
                }),
                Effect::DamageTile(DamageTileConfig {
                    position: Position {
                        x: (character.current_state.position.x as i32 + delta_x) as u8,
                        y: (character.current_state.position.y as i32 + delta_y) as u8,
                    },
                    amount: 1,
                }),
            ]
        }
        Action::Bardass1(bardass1_action) => vec![],
        Action::Bardass2(bardass2_action) => vec![],
        Action::Bardass3(bardass3_action) => vec![],
        Action::Stunlor1(stunlor1_action) => vec![],
        Action::Stunlor2(stunlor2_action) => vec![],
        Action::Stunlor3(stunlor3_action) => vec![],
    }
}
