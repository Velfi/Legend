use super::{Combatant, Battle};
use ggez::event;

pub struct Team {
    current_teammate: usize,
    current_teammate_turn_phase: CombatantTurnPhase,
    /// A vec of usizes used as keys to refer to elements in a Slab
    teammates: Vec<usize>,
    pub turn_over: bool,
}

impl Team {
    pub fn process_event(&mut self, event: &event::Event) {
        ()
    }

    pub fn new(teammates: Vec<usize>) -> Self {
        Team {
            current_teammate: 0,
            current_teammate_turn_phase: CombatantTurnPhase::Start,
            teammates,
            turn_over: false,
        }
    }

    pub fn update(&mut self, combatants: &mut super::Slab<Combatant>) {
        ()
    }

    pub fn is_dead(&self, combatants: &super::Slab<Combatant>) -> bool {
        if self.teammates.len() == 0 {
            return true
        }
        let mut is_everyone_dead = true;
        for teammate in self.teammates.iter() {
            if let Some(combatant) = combatants.get(*teammate) {
                if combatant.hp > 0 {
                    is_everyone_dead = false;
                }
            }
        }
        is_everyone_dead
    }

    pub fn do_actions(&mut self) {
        // for mut teammate in self.teammates.iter_mut() {
        //     teammate.do_queued_action();
        // }
    }

    pub fn next_teammate(&mut self) {
        self.current_teammate += 1;
        self.current_teammate %= self.teammates.len();
        // if self.current_teammate == 0 {
        //     self.current_teammate_turn_phase = CombatantTurnPhase::Start;
        // }
    }

    pub fn get_members(&self) -> &Vec<usize> {
        &self.teammates
    }
}

#[derive(Clone, Copy)]
enum TeamTurnPhase {
    TurnStart,
    TeammateTurnStart,
    TeammateTurnEnd,
    TurnEnd,
}

#[derive(Clone, Copy)]
pub enum CombatantTurnPhase {
    Start,
    AwaitChoice,
    End,
}

impl CombatantTurnPhase {
    pub fn next(self) -> Self {
        match self {
            CombatantTurnPhase::Start => CombatantTurnPhase::AwaitChoice,
            CombatantTurnPhase::AwaitChoice => CombatantTurnPhase::End,
            CombatantTurnPhase::End => CombatantTurnPhase::Start,
        }
    }
}