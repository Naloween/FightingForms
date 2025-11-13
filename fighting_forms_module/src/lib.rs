pub mod action;
pub mod character;
pub mod game;
pub mod player;

use spacetimedb::{reducer, ReducerContext, SpacetimeType, Table};

use crate::{
    action::*,
    character::*,
    game::*,
    player::{player as player_table, Player},
};

#[derive(SpacetimeType, PartialEq, Eq, Clone)]
pub struct Position {
    x: u8,
    y: u8,
}

#[derive(SpacetimeType, PartialEq, Eq, Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(SpacetimeType, PartialEq, Eq, Clone)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    // Adding action_class_configs
    let move_action_class = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Move,
        name: "Move".to_string(),
        description: "Moving one tile".to_string(),
        mana_cost: 0,
        stamina_cost: 1,
        cooldown: 1,
        duration: 0,
    });
    let zytex1 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Zytex1,
        name: "Zytex1".to_string(),
        description: "".to_string(),
        mana_cost: 1,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });
    let zytex2 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Zytex2,
        name: "Zytex2".to_string(),
        description: "".to_string(),
        mana_cost: 1,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });
    let zytex3 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Zytex3,
        name: "Zytex3".to_string(),
        description: "".to_string(),
        mana_cost: 1,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });

    let bardass1 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Bardass1,
        name: "Bardass1".to_string(),
        description: "".to_string(),
        mana_cost: 1,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });
    let bardass2 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Bardass2,
        name: "Bardass2".to_string(),
        description: "".to_string(),
        mana_cost: 2,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });
    let bardass3 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Bardass3,
        name: "Bardass3".to_string(),
        description: "".to_string(),
        mana_cost: 4,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });

    let stunlor1 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Stunlor1,
        name: "Stunlor1".to_string(),
        description: "".to_string(),
        mana_cost: 1,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });
    let stunlor2 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Stunlor2,
        name: "Stunlor2".to_string(),
        description: "".to_string(),
        mana_cost: 3,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });
    let stunlor3 = ctx.db.action_class_config().insert(ActionClassConfig {
        id: 0,
        action_class: ActionClass::Stunlor3,
        name: "Stunlor3".to_string(),
        description: "".to_string(),
        mana_cost: 2,
        stamina_cost: 0,
        cooldown: 1,
        duration: 0,
    });

    // Adding characters
    ctx.db.character_class().insert(CharacterClass {
        id: 0,
        name: "Zytex".to_string(),
        description: "A strong melee fighter".to_string(),
        hp: 4,
        mana: 5,
        stamina: 1,
        actions_class: vec![
            ActionClass::Move,
            ActionClass::Zytex1,
            ActionClass::Zytex2,
            ActionClass::Zytex3,
        ],
    });
    ctx.db.character_class().insert(CharacterClass {
        id: 0,
        name: "Bardass".to_string(),
        description: "A strong melee fighter".to_string(),
        hp: 6,
        mana: 5,
        stamina: 1,
        actions_class: vec![
            ActionClass::Move,
            ActionClass::Bardass1,
            ActionClass::Bardass2,
            ActionClass::Bardass3,
        ],
    });
    ctx.db.character_class().insert(CharacterClass {
        id: 0,
        name: "Stunlor".to_string(),
        description: "A strong melee fighter".to_string(),
        hp: 5,
        mana: 5,
        stamina: 1,
        actions_class: vec![
            ActionClass::Move,
            ActionClass::Stunlor1,
            ActionClass::Stunlor2,
            ActionClass::Stunlor3,
        ],
    });
}

#[reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    let player = ctx.db.player().id().find(&ctx.sender);
    if let Option::None = player {
        ctx.db.player().insert(Player {
            id: ctx.sender,
            name: "New Player".to_string(),
            character_class_id: Option::None,
            character_id: Option::None,
            connected: true,
            game_id: Option::None,
            ready: false,
            eliminated: false,
        });
    } else {
        ctx.db.player().id().update(Player {
            connected: true,
            ..player.unwrap()
        });
    }
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    let player = ctx.db.player().id().find(&ctx.sender).unwrap();
    if player.game_id.is_some() {
        quit_game(ctx);
    }
    ctx.db.player().id().delete(player.id);
}
