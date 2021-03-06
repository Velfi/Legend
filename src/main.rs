extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::Context;

extern crate shrev;

mod state;
use state::{main_state::MainState, StateMachine};

mod ui;

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("legend", "ggez", c).unwrap();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
        println!("Adding path {:?}", path);
    } else {
        println!("not building with cargo?");
    }

    let state = MainState::new(ctx).unwrap();
    let mut events = event::Events::new(ctx).unwrap();

    let state_machine = StateMachine::new(state);
    state_machine.start(ctx, &mut events);
}
