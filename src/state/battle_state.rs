
use super::{ggez::{event,
                   graphics::{self, DrawMode, Point2},
                   Context,
                   GameResult},
            State,
            StateTransition};

use std::sync::mpsc;
use std::fmt;
use std::default::Default;

const MAX_PARRY_CHARGES: usize = 2;

/// The main state for now. Will maybe become a menu later.
pub struct BattleState {
    quit: bool,
    battle: Battle,
    battle_result: BattleResult,
    player_events_tx: mpsc::Sender<Action>,
    player_events_rx: mpsc::Receiver<Action>,
    state_label: graphics::Text,
    font: graphics::Font,
}

impl BattleState {
    pub fn new(ctx: &mut Context) -> GameResult<BattleState> {

        let (tx, rx) = mpsc::channel();

        let player = Combatant::new(String::from("Player"));
        let enemy = Combatant::new(String::from("Enemy"));

        let battle = Battle::new(
            vec!(player),
            vec!(enemy),
        );

        let font = graphics::Font::new(ctx, "/font.ttf", 48)?;

        let s = BattleState {
            quit: false,
            battle,
            battle_result: BattleResult::None,
            player_events_tx: tx,
            player_events_rx: rx,
            state_label: graphics::Text::new(ctx, "Battle State", &font)?,
            font,
        };
        Ok(s)
    }
}

impl State for BattleState {
    fn process_event(&mut self, event: &event::Event) {
        match event {
            event::Event::Quit { .. }
            | event::Event::KeyDown {
                keycode: Some(event::Keycode::Escape),
                ..
            } => {
                println!("Quitting");
                self.quit = true;
            }
            event::Event::KeyDown {
                keycode: Some(event::Keycode::Num1),
                ..
            } => {
                self.player_events_tx.send(Action::Attack).unwrap();
            }
            event::Event::KeyDown {
                keycode: Some(event::Keycode::Num2),
                ..
            } => {
                self.player_events_tx.send(Action::Charge).unwrap();
            }
            event::Event::KeyDown {
                keycode: Some(event::Keycode::Num3),
                ..
            } => {
                self.player_events_tx.send(Action::Parry).unwrap();
            }
            input => println!("Event fired: {:?}", input),
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> Result<StateTransition, ()> {
        if self.quit {
            Ok(StateTransition::Quit)
        } else if self.battle.turn_count % 2 == 1 {
            ai_combatant_turn(self);
            Ok(StateTransition::None)
        } else {
            let next_phase = player_combatant_turn(self);
            self.battle.turn_phase = next_phase;
            Ok(StateTransition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_turn_phase(ctx, &self.font, &self.battle.turn_phase);
        graphics::draw(
            ctx,
            &self.state_label,
            Point2::new(600.0, 10.0),
            0.0
        )?;
        graphics::present(ctx);
        Ok(())
    }
}

fn draw_turn_phase(ctx: &mut Context, font: &graphics::Font, phase: &TurnPhase) {
    let phase_str = match phase {
        TurnPhase::AwaitChoice => "Awaiting choice",
        TurnPhase::CheckWin => "Checking for win",
        TurnPhase::DoChoice => "Doing choice",
        TurnPhase::End => "Turn end",
        TurnPhase::Start => "Turn start",
    };
    let phase_text = graphics::Text::new(ctx, phase_str, font).unwrap();
    graphics::draw(
        ctx,
        &phase_text,
        Point2::new(10.0, 10.0),
        0.0
    ).unwrap();
}

// fn draw_combatant_hp(ctx: &mut Context, font: &graphics::Font, Combatant) {
//     unimplemented!()
// }

#[derive(Clone)]
pub enum TurnPhase {
    Start,
    AwaitChoice,
    DoChoice,
    CheckWin,
    End,
}

impl TurnPhase {
    pub fn next(self) -> Self {
        match self {
            TurnPhase::AwaitChoice => TurnPhase::DoChoice,
            TurnPhase::CheckWin => TurnPhase::End,
            TurnPhase::DoChoice => TurnPhase::CheckWin,
            TurnPhase::End => TurnPhase::Start,
            TurnPhase::Start => TurnPhase::AwaitChoice,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Attack,
    Charge,
    Parry,
    None,
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match *self{
            Action::Attack => "Attack",
            Action::Charge => "Charge",
            Action::Parry => "Parry",
            Action::None => "No Action",
        };
        write!(f, "{}", action)
    }
}

pub struct Combatant {
    name: String,
    attack: isize,
    defense: isize,
    hp: isize,
    action: Action,
    is_charged: bool,
    is_parry: bool,
    parry_charges: isize,
    parry_charge_counter: isize,
}

impl Combatant {
    pub fn new(name: String)-> Self {
        Combatant {
            name,
            attack: 10,
            defense: 10,
            hp: 100,
            action: Action::None,
            is_charged: false,
            is_parry: false,
            parry_charges: 0,
            parry_charge_counter: 3,
        }
    }

    pub fn do_attack(&mut self, target: &mut Combatant) {
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
            println!("{} attacked {} for {} damage!", self.name, target.name, damage);
            target.hp -= damage;
        }
    }

    pub fn do_charge(&mut self) -> bool {
        if self.is_charged {
            false
        } else {
            self.is_charged = true;
            true
        }
    }

    pub fn do_parry(&mut self) -> bool {
        if self.is_parry {
            false
        } else {
            self.is_parry = true;
            true
        }
    }
}

pub struct Battle {
    turn_count: usize,
    attackers: Vec<Combatant>,
    defenders: Vec<Combatant>,
    turn_phase: TurnPhase,
}

impl Battle {
    pub fn new(attackers: Vec<Combatant>, defenders: Vec<Combatant>) -> Self {
        Battle {
            attackers,
            defenders,
            turn_count: 0,
            turn_phase: TurnPhase::Start,
        }
    }

    pub fn check_win(&self) -> BattleResult {
        if self.attackers[0].hp <= 0 {
            BattleResult::AttackersLose
        } else if self.defenders[0].hp <= 0 {
            BattleResult::AttackersWin
        } else {
            BattleResult::None
        }
    }
}

#[derive(PartialEq)]
pub enum BattleResult {
    None,
    AttackersWin,
    AttackersLose,
}

impl Default for BattleResult {
    fn default() -> Self {
        BattleResult::None
    }
}

pub fn player_combatant_turn(state: &mut BattleState) -> TurnPhase {
    state.battle.attackers[0].is_parry = false;
    let mut next_phase = state.battle.turn_phase.clone();
    next_phase = match state.battle.turn_phase {
        TurnPhase::Start => next_phase.next(),
        TurnPhase::AwaitChoice => {
            let action = state.player_events_rx.try_recv().unwrap_or_default();
            state.player_events_rx.try_iter().for_each(|_| ());
            if action != Action::None {
                println!("Player chose action: {}", action);
                state.battle.attackers[0].action = action;
                next_phase.next()
            } else {
                next_phase
            }
        },
        TurnPhase::DoChoice => {
            match state.battle.attackers[0].action {
                Action::Attack => state.battle.attackers[0].do_attack(&mut state.battle.defenders[0]),
                Action::Charge => if state.battle.attackers[0].do_charge() {
                    println!("{} is charging their attack!", state.battle.attackers[0].name);
                },
                Action::Parry => if state.battle.attackers[0].do_parry() {
                    println!("{} is ready for anything!", state.battle.attackers[0].name);
                },
                Action::None => (),
            };
            next_phase.next()
        },
        TurnPhase::CheckWin => {
            match state.battle.check_win() {
                BattleResult::AttackersWin => {
                    println!("The attackers have won the battle!");
                    state.quit = true;
                },
                BattleResult::AttackersLose => {
                    println!("The attackers have lost the battle.");
                    state.quit = true;
                },
                BattleResult::None => (),
            };
            next_phase.next()
        },
        TurnPhase::End => {
            state.battle.turn_count += 1;
            next_phase.next()
        },
    };

    next_phase
}

pub fn ai_combatant_turn(state: &mut BattleState) {
    state.battle.defenders[0].is_parry = false;
    state.battle.turn_count += 1;
    state.battle.turn_phase = TurnPhase::Start;
    println!("AI skipped turn.");
}
