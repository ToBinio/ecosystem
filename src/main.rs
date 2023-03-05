use ggez::{event, GameResult};

use crate::state::State;

mod state;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = State {};
    event::run(ctx, event_loop, state)
}
