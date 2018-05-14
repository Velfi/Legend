use std::collections::HashMap;
use std::fmt;

use super::{ggez::{event,
                   graphics::{self, Font, Image, Point2, Text},
                   Context,
                   GameResult},
            State,
            StateTransition};

use super::super::slab::Slab;

// use super::shrev::EventChannel;
pub mod combatant;
use self::combatant::Combatant;

pub mod team;
use self::team::Team;

pub struct BattleState {
    quit: bool,
    battle: Battle,
    state_label: graphics::Text,
    sprites: HashMap<String, Image>,
}

impl BattleState {
    pub fn new(ctx: &mut Context) -> GameResult<BattleState> {
        let font = Font::new(ctx, "/font.ttf", 48)?;

        let mut s = BattleState {
            quit: false,
            battle: Battle::new(),
            state_label: Text::new(ctx, "Battle State", &font)?,
            sprites: HashMap::new(),
        };

        let gs = Image::new(ctx, "/green_swordsman.png").unwrap();
        let rs = Image::new(ctx, "/red_swordsman.png").unwrap();
        let hw = Image::new(ctx, "/hollow_warrior.png").unwrap();

        s.sprites.insert(String::from("green_swordsman"), gs);
        s.sprites.insert(String::from("red_swordsman"), rs);
        s.sprites.insert(String::from("hollow_warrior"), hw);

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
                debug!("Quitting");
                self.quit = true;
            }
            input => {
                self.battle.teams[self.battle.current_team].process_event(input);
            }
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> Result<StateTransition, ()> {
        if self.quit {
            return Ok(StateTransition::Quit);
        }

        self.battle.update();

        Ok(StateTransition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        // draw_turn_phase(ctx, &self.font, &self.battle.turn_phase);
        graphics::draw(ctx, &self.state_label, Point2::new(600.0, 10.0), 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

// fn draw_turn_phase(ctx: &mut Context, font: &graphics::Font, phase: &CombatantTurnPhase) {
//     let phase_str = match phase {
//         CombatantTurnPhase::AwaitChoice => "Awaiting choice",
//         // CombatantTurnPhase::CheckWin => "Checking for win",
//         // CombatantTurnPhase::DoChoice => "Doing choice",
//         CombatantTurnPhase::End => "Turn end",
//         CombatantTurnPhase::Start => "Turn start",
//     };
//     let phase_text = graphics::Text::new(ctx, phase_str, font).unwrap();
//     graphics::draw(
//         ctx,
//         &phase_text,
//         Point2::new(10.0, 10.0),
//         0.0
//     ).unwrap();
// }

// fn draw_combatant_hp(ctx: &mut Context, font: &graphics::Font, Combatant) {
//     unimplemented!()
// }

pub struct Battle {
    battle_event: BattleEvent,
    current_team: usize,
    running: bool,
    teams: Vec<Team>,
    combatants: Slab<Combatant>,
    turn_count: usize,
}

impl Battle {
    pub fn new() -> Self {
        let mut combatants = Slab::with_capacity(8);

        let player_team = Team::new(vec![combatants.insert(Combatant::new(
            String::from("Player"),
            String::from("green_swordsman"),
        ))]);

        let enemy_team = Team::new(vec![combatants.insert(Combatant::new(
            String::from("Enemy"),
            String::from("hollow_warrior"),
        ))]);

        Battle {
            running: false,
            teams: vec![player_team, enemy_team],
            combatants,
            current_team: 0,
            turn_count: 0,
            battle_event: BattleEvent::BattleStart,
        }
    }

    pub fn update(&mut self) {
        match self.battle_event {
            BattleEvent::BattleStart => self.start(),
            BattleEvent::TurnStart => self.turn_start(),
            BattleEvent::TurnEnd => self.turn_end(),
            BattleEvent::BattleEnd => self.end(),
            BattleEvent::None => (),
        }

        self.battle_event = BattleEvent::None;

        if self.running {
            self.teams[self.current_team].update(&mut self.combatants);
            if self.teams[self.current_team].turn_over {
                self.next_team();
            }
        }
    }

    pub fn next_team(&mut self) {
        self.current_team += 1;
        self.current_team %= self.teams.len();
        if self.current_team == 0 {
            self.battle_event = BattleEvent::TurnEnd;
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        println!("Battle Start")
    }

    pub fn end(&mut self) {
        self.running = false;
        println!("Battle End")
    }

    pub fn turn_start(&mut self) {
        self.turn_count += 1;
        println!("t{}: Turn Start", self.turn_count)
    }

    pub fn turn_end(&self) {
        println!("t{}: Turn End", self.turn_count)
    }

    pub fn get_team_combatants(&self, team: &Team) -> Vec<&Combatant> {
        let mut combatant_refs = Vec::new();
        for teammate in team.get_members().iter() {
            if let Some(combatant) = self.combatants.get(*teammate) {
                combatant_refs.push(combatant);
            }
        }
        combatant_refs
    }

    /// Return `Vec<usize>` of indexs of teams whose combatants are all dead.
    pub fn check_for_dead_team(&self) -> Vec<usize> {
        let mut dead_teams = Vec::new();
        for (index, team) in self.teams.iter().enumerate() {
            let combatant_refs = self.get_team_combatants(team);
            if self.is_team_dead(&combatant_refs) {
                dead_teams.push(index);
            }
        }
        dead_teams
    }

    pub fn is_team_dead(&self, combatant_refs: &Vec<&Combatant>) -> bool {
        let mut someone_is_alive = false;
        if combatant_refs.len() > 0 {
            for combatant_ref in combatant_refs {
                if !combatant_ref.is_dead() {
                    someone_is_alive = true;
                }
            }
        }
        !someone_is_alive
    }
}

enum BattleEvent {
    BattleStart,
    TurnStart,
    TurnEnd,
    BattleEnd,
    None,
}

pub enum Action {
    Attack(usize, usize),
    Charge(usize),
    Parry(usize),
    None,
}

impl<'a> Default for Action {
    fn default() -> Self {
        Action::None
    }
}

impl<'a> fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let action = match *self {
            Action::Attack(_, _) => "Attack",
            Action::Charge(_) => "Charge",
            Action::Parry(_) => "Parry",
            Action::None => "No Action",
        };
        write!(f, "{}", action)
    }
}
