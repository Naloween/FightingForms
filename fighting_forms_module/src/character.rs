use spacetimedb::{reducer, table, Identity, ReducerContext, SpacetimeType, Table};

use crate::{
    action::*,
    game::{game as game_table, NB_STEPS},
    player::{player as player_table, Player},
    Position,
};

#[table(name = character_class, public)]
pub struct CharacterClass {
    #[primary_key]
    #[auto_inc]
    pub id: u16,
    pub name: String,
    pub description: String,
    pub hp: u8,
    pub mana: u8,
    pub stamina: u8,
    pub actions_class: Vec<ActionClass>,
}

#[derive(SpacetimeType, Clone)]
pub struct CharacterState {
    pub position: Position,
    pub hp: u8,
    pub mana: u8,
    pub stamina: u8,
    pub max_hp: u8,
    pub max_mana: u8,
    pub max_stamina: u8,
    pub status: Vec<Status>,
}

#[table(name = character, public)]
pub struct Character {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    #[index(btree)]
    pub game_id: u64,
    pub player_id: Identity,
    pub character_class_id: u16,
    pub choosen_actions: Vec<Option<Action>>,
    pub current_state: CharacterState,
    pub states: Vec<CharacterState>,
}

#[derive(SpacetimeType, Clone, Copy, PartialEq, Eq)]
pub enum JaugeType {
    HP,
    Mana,
    Stamina,
}

#[derive(SpacetimeType, Clone, Copy)]
pub enum Status {
    Stunned(StunnedConfig),
    DamageReduction(DamageReductionConfig),
    RefundOnDamage(RefundOnDamageConfig),
}

#[derive(SpacetimeType, Clone, Copy)]
pub struct StunnedConfig {
    pub duration: u8,
}

#[derive(SpacetimeType, Clone, Copy)]
pub struct DamageReductionConfig {
    pub amount: u8,
    pub duration: u8,
    pub only_once: bool,
}

#[derive(SpacetimeType, Clone, Copy)]
pub struct RefundOnDamageConfig {
    pub jauge_type: JaugeType,
    pub only_once: bool,
    pub amount: u8,
    pub duration: u8,
}

#[reducer]
pub fn select_character_class(ctx: &ReducerContext, character_class_id: u16) {
    let player = ctx.db.player().id().find(ctx.sender).unwrap();

    // check that in a game and in the lobby (game not started) as well as the character class exists
    if let Some(game_id) = player.game_id {
        let game = ctx.db.game().id().find(game_id).unwrap();
        if !game.started {
            if let Some(character_class) = ctx.db.character_class().id().find(character_class_id) {
                // Create a corresponding character from the class, and delete the previous one
                if let Some(character_id) = player.character_id {
                    ctx.db.character().id().delete(character_id);
                }
                let character_state = CharacterState {
                    position: Position { x: 0, y: 0 },
                    mana: character_class.mana,
                    hp: character_class.hp,
                    stamina: character_class.stamina,
                    max_mana: character_class.mana,
                    max_hp: character_class.hp,
                    max_stamina: character_class.stamina,
                    status: Vec::new(),
                };

                let character = ctx.db.character().insert(Character {
                    id: 0,
                    player_id: ctx.sender,
                    game_id,
                    character_class_id,
                    choosen_actions: vec![Option::None; NB_STEPS as usize],
                    current_state: character_state.clone(),
                    states: vec![character_state; NB_STEPS as usize + 1],
                });
                ctx.db.player().id().update(Player {
                    character_id: Some(character.id),
                    character_class_id: Some(character_class_id),
                    ..player
                });
            }
        }
    }
}

#[reducer]
pub fn choose_action(ctx: &ReducerContext, action: Option<Action>, step: u8) {
    if step > NB_STEPS - 1 {
        return;
    }

    let player = ctx.db.player().id().find(ctx.sender).unwrap();
    if let Some(character_id) = player.character_id {
        let character = ctx.db.character().id().find(character_id).unwrap();
        let mut new_choosen_actions = character.choosen_actions.clone();
        new_choosen_actions[step as usize] = action;

        ctx.db.character().id().update(Character {
            choosen_actions: new_choosen_actions,
            ..character
        });
    }
}

pub fn update_character_step_state(ctx: &ReducerContext, step: usize) {
    for character in ctx.db.character().iter() {
        let mut new_character_states = character.states.clone();
        new_character_states[step] = character.current_state.clone();
        ctx.db.character().id().update(Character {
            states: new_character_states,
            ..character
        });
    }
}
