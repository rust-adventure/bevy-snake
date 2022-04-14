use std::time::{Duration, Instant};

use bevy::prelude::{App, Plugin, Res, ResMut, SystemSet};

use crate::common::{Game, RunState};

pub struct SpeedrunPlugin;

impl Plugin for SpeedrunPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Timer>()
            .init_resource::<Speedruns>()
            .add_system_set(
                SystemSet::on_enter(RunState::Playing)
                    .with_system(start_timer),
            )
            .add_system_set(
                SystemSet::on_exit(RunState::Playing)
                    .with_system(close_timer),
            );
    }
}

#[derive(Debug, Clone)]
pub struct Run {
    time: Duration,
    score: u32,
}

pub struct Speedruns {
    runs: Vec<Run>,
}
impl Speedruns {
    fn sorted_by_score(&self) -> Vec<Run> {
        let mut runs = self.runs.clone();
        runs.sort_by_key(|run| run.score);
        runs
    }
    fn sorted_by_run(&self) -> Vec<Run> {
        self.runs.clone()
    }
}

impl Default for Speedruns {
    fn default() -> Self {
        Speedruns { runs: vec![] }
    }
}

pub struct Timer {
    pub start: Option<Instant>,
    pub runtime: Option<Duration>,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start: Some(Instant::now()),
            runtime: None,
        }
    }
}

fn start_timer(mut timer: ResMut<Timer>) {
    timer.start = Some(Instant::now());
}

fn close_timer(
    mut timer: ResMut<Timer>,
    mut runs: ResMut<Speedruns>,
    game: Res<Game>,
) {
    timer.runtime = Some(timer.start.unwrap().elapsed());
    runs.runs.push(Run {
        time: timer.runtime.unwrap(),
        score: game.score,
    });
    dbg!(&runs.runs);
}
