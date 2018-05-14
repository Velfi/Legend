const MAX_PARRY_CHARGES: usize = 2;

pub struct Combatant {
    // action: Action,
    pub attack: isize,
    pub defense: isize,
    pub hp: isize,
    pub is_charged: bool,
    pub is_parry: bool,
    pub name: String,
    pub parry_charge_counter: isize,
    pub parry_charges: isize,
    pub sprite_name: String,
    // queued_action: Action,
}

impl Combatant {
    pub fn new(name: String, sprite: String) -> Self {
        Combatant {
            // action: Action::None,
            attack: 10,
            defense: 10,
            hp: 100,
            is_charged: false,
            is_parry: false,
            name,
            parry_charge_counter: 3,
            parry_charges: 0,
            sprite_name: sprite,
            // queued_action: Action::None,
        }
    }

    fn do_attack(&mut self, target: &mut Combatant) {
        let mut damage = self.attack;

        if self.is_charged {
            damage *= 2;
            self.is_charged = false;
        }

        if target.is_parry {
            println!("{} parried {}'s attack!", target.name, self.name);
            self.hp -= damage;
            target.is_parry = false;
        } else {
            println!(
                "{} attacked {} for {} damage!",
                self.name, target.name, damage
            );
            target.hp -= damage;
        }
    }

    fn do_charge(&mut self) -> bool {
        if self.is_charged {
            false
        } else {
            self.is_charged = true;
            true
        }
    }

    fn do_parry(&mut self) -> bool {
        if self.is_parry {
            false
        } else {
            self.is_parry = true;
            true
        }
    }

    pub fn do_queued_action(&mut self) {
        // match self.queued_action {
        //     Action::Attack(target) => self.do_attack(target),
        //     Action::Charge => {self.do_charge();},
        //     Action::Parry => {self.do_parry();},
        //     Action::None => (),
        // };
    }

    pub fn is_dead(&self) -> bool {
        if self.hp <= 0 {
            true
        } else {
            false
        }
    }
}

// pub fn player_combatant_turn(state: &mut BattleState) -> CombatantTurnPhase {
//     state.battle.team_a[0].is_parry = false;
//     let mut next_phase = state.battle.turn_phase.clone();
//     next_phase = match state.battle.turn_phase {
//         CombatantTurnPhase::Start => next_phase.next(),
//         CombatantTurnPhase::AwaitChoice => {
//             let action = state.player_events_rx.try_recv().unwrap_or_default();
//             state.player_events_rx.try_iter().for_each(|_| ());
//             if action != Action::None {
//                 println!("Player chose action: {}", action);
//                 state.battle.team_a[0].action = action;
//                 next_phase.next()
//             } else {
//                 next_phase
//             }
//         },
//         // CombatantTurnPhase::DoChoice => {
//         //     match state.battle.team_a[0].action {
//         //         Action::Attack => state.battle.team_a[0].do_attack(&mut state.battle.team_b[0]),
//         //         Action::Charge => if state.battle.team_a[0].do_charge() {
//         //             println!("{} is charging their attack!", state.battle.team_a[0].name);
//         //         },
//         //         Action::Parry => if state.battle.team_a[0].do_parry() {
//         //             println!("{} is ready for anything!", state.battle.team_a[0].name);
//         //         },
//         //         Action::None => (),
//         //     };
//         //     next_phase.next()
//         // },
//         // CombatantTurnPhase::CheckWin => {
//         //     match state.battle.check_win() {
//         //         BattleResult::AttackersWin => {
//         //             println!("The team_a have won the battle!");
//         //             state.quit = true;
//         //         },
//         //         BattleResult::AttackersLose => {
//         //             println!("The team_a have lost the battle.");
//         //             state.quit = true;
//         //         },
//         //         BattleResult::None => (),
//         //     };
//         //     next_phase.next()
//         // },
//         CombatantTurnPhase::End => {
//             state.battle.turn_count += 1;
//             next_phase.next()
//         },
//     };

//     next_phase
// }

// pub fn ai_combatant_turn(state: &mut BattleState) {
//     state.battle.team_b[0].is_parry = false;
//     state.battle.turn_count += 1;
//     state.battle.turn_phase = CombatantTurnPhase::Start;
//     println!("AI skipped turn.");
// }
