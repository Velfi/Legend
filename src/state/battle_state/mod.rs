use super::{ggez::{event,
                   graphics::{self, Point2},
                   Context,
                   GameResult},
            State,
            StateTransition};

// use super::shrev::EventChannel;
pub mod combatant;
use self::combatant::{Combatant, CombatantTurnPhase};

pub mod team;
use self::team::Team;

pub struct BattleState {
    quit: bool,
    battle: Battle,
    state_label: graphics::Text,
    font: graphics::Font,
}

impl BattleState {
    pub fn new(ctx: &mut Context) -> GameResult<BattleState> {
        let player = Combatant::new(String::from("Player"));
        let enemy = Combatant::new(String::from("Enemy"));

        let battle = Battle::new(
            vec!(
                Team::new(vec!(player)),
                Team::new(vec!(enemy))
            )
        );

        let font = graphics::Font::new(ctx, "/font.ttf", 48)?;

        let s = BattleState {
            quit: false,
            battle,
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
                debug!("Quitting");
                self.quit = true;
            }
            input => {
                // self.battle.teams[self.battle.current_team].process_event();
            },
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
    turn_count: usize,
}

impl Battle {
    pub fn new(teams: Vec<Team>) -> Self {
        Battle {
            running: false,
            teams,
            current_team: 0,
            turn_count: 0,
            battle_event: BattleEvent::BattleStart,
        }
    }

    pub fn check_win(&self) -> Option<usize> {
        let mut is_a_team_dead = false;
        for (index, team) in self.teams.iter().enumerate() {
            if team.is_dead() {
                return Some(index);
            }
        }
        None
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
            self.teams[self.current_team].update();
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
}

enum BattleEvent {
    BattleStart,
    TurnStart,
    TurnEnd,
    BattleEnd,
    None,
}
