extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::Context;

extern crate slab;

extern crate shrev;

#[macro_use]
extern crate log;
use log::LevelFilter;
extern crate fern;
use fern::colors::{Color, ColoredLevelConfig};

mod state;
use state::{main_state::MainState, StateMachine};

mod ui;

pub fn main() {
    if let Err(e) = setup_logger() {
        println!("The logger failed to start with error: {}", e);
        println!("This may cause some log messages to be swallowed =(");
    } else {
        debug!("The logger has started successfully.")
    }

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("legend", "ggez", c).unwrap();

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
        trace!("Adding path {:?}", path);
    } else {
        warn!("not building with cargo?");
    }

    let state = MainState::new(ctx).unwrap();
    let mut events = event::Events::new(ctx).unwrap();

    let state_machine = StateMachine::new(state);
    state_machine.start(ctx, &mut events);
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Green)
        .error(Color::BrightRed)
        .info(Color::Blue)
        .trace(Color::BrightMagenta)
        .warn(Color::Yellow);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] {} -> {}",
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level_for("gfx_device_gl", LevelFilter::Error)
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
