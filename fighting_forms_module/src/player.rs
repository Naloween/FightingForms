use spacetimedb::{reducer, table, Identity, ReducerContext};

#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    pub id: Identity,
    pub name: String,
    pub character_class_id: Option<u16>,
    pub character_id: Option<u64>,
    pub connected: bool,
    pub game_id: Option<u64>,
    pub ready: bool,
    pub eliminated: bool,
}

#[reducer]
pub fn set_name(ctx: &ReducerContext, name: String) {
    let player = ctx.db.player().id().find(&ctx.sender).unwrap();
    ctx.db.player().id().update(Player { name, ..player });
}
