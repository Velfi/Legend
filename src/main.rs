// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

extern crate ggez;
use ggez::conf;
use ggez::event;
use ggez::Context;

mod state;
use state::{main_state::MainState, StateMachine};

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = Box::new(MainState::new(ctx).unwrap());
    let mut events = event::Events::new(ctx).unwrap();

    let state_machine = StateMachine::new(vec![state]);
    state_machine.start(ctx, &mut events);
}
