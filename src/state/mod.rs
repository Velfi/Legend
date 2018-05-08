/// The state that runs whenever a player enters battle.
pub mod battle_state;
pub mod field_state;
pub mod main_state;

use super::ggez::{self, event, Context, GameResult};
use super::ui;

use std::mem;

pub trait State {
    /// Called at a fixed rate
    fn update(&mut self, ctx: &mut Context) -> Result<StateTransition, ()>;
    /// Called as often as possible
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
    /// A lifecycle event that runs once when the state is first started.
    fn on_start(&mut self) {}
    /// A lifecycle event that runs once just before the state is popped off the stack.
    fn on_end(&mut self) {}
    // fn on_pause(&mut self);
    // fn on_resume(&mut self);
    fn process_event(&mut self, event: &event::Event);
}

/// A struct for managing game states and transitions between them. Calls `update()`
/// and `draw()` on the state on top of the `state_stack`.
pub struct StateMachine<'a> {
    /// A Vec of states that will be managed by the State Machine.
    state_stack: Vec<Box<State + 'a>>,
    /// The state machine will halt if this is set to false. Useful for quitting.
    running: bool,
    /// The transition stored in this field will be evaluated and replaced with a
    /// `StateTransition::None` each loop.
    transition: StateTransition,
}

impl<'a> StateMachine<'a> {

    pub fn new<S: State + 'a>(initial_state: S) -> StateMachine<'a> {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
            transition: StateTransition::None,
        }
    }

    pub fn start(mut self, ctx: &mut Context, events: &mut event::Events) {
        self.running = true;

        while self.running {
            ctx.timer_context.tick();

            if let Some(state) = self.state_stack.last_mut() {
                for event in events.poll() {
                    ctx.process_event(&event);
                    state.process_event(&event);
                }
                self.transition = state.update(ctx).expect("Failed to update game state.");
                state.draw(ctx).expect("Failed to render game state.");
            }

            let transition = mem::replace(&mut self.transition, StateTransition::None);
            self.transition(transition);

            ggez::timer::yield_now();
        }
    }

    /// Push a new state on top of the `state_stack`. The `StateMachine` will
    /// begin running the new state on the next loop.
    pub fn push(&mut self, state: Box<State>) {
        self.state_stack.push(state);
        if let Some(state) = self.state_stack.last_mut() {
            state.on_start();
        }
    }

    /// Remove the top state on the state and run its end lifecycle event.
    pub fn pop(&mut self) {
        if let Some(state) = self.state_stack.last_mut() {
            state.on_end();
        }
        self.state_stack.pop();
    }

    pub fn switch(&mut self, state: Box<State>) {
        self.pop();
        self.push(state);
    }

    /// Sets running to false. This will cause the `StateMachine` to halt on the
    /// tick after this is called.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn transition(&mut self, transition: StateTransition) {
        match transition {
            StateTransition::None => (),
            StateTransition::Pop => self.pop(),
            StateTransition::Push(new_state) => self.push(new_state),
            StateTransition::Switch(new_state) => self.switch(new_state),
            StateTransition::Quit => self.quit(),
        }
    }
}

/// Types of state transitions
pub enum StateTransition {
    None,
    Pop,
    Push(Box<State>),
    Switch(Box<State>),
    Quit,
    // Pause,
    // Unpause,
}
