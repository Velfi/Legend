use super::{Combatant, CombatantTurnPhase};
use ggez::event;

pub struct Team {
    current_teammate: usize,
    current_teammate_turn_phase: CombatantTurnPhase,
    teammates: Vec<Combatant>,
    pub turn_over: bool,
}

impl Team {

    pub fn process_event(event: event::Event) {
        unimplemented!()
    }

    pub fn new(teammates: Vec<Combatant>) -> Self {
        Team {
            current_teammate: 0,
            current_teammate_turn_phase: CombatantTurnPhase::Start,
            teammates,
            turn_over: false,
        }
    }

    pub fn update(&mut self) {
        ()
    }

    pub fn is_dead(&self) -> bool {
        unimplemented!()
    }

    pub fn do_actions(&mut self) {
        for mut teammate in self.teammates.iter_mut() {
            teammate.do_queued_action();
        }
    }

    pub fn next_teammate(&mut self) {
        self.current_teammate += 1;
        self.current_teammate %= self.teammates.len();
        // if self.current_teammate == 0 {
        //     self.current_teammate_turn_phase = CombatantTurnPhase::Start;
        // }
    }
}

enum TeamEvent {
    TurnStart,
    TeammateTurnStart,
    TeammateTurnEnd,
    TurnEnd,
}
