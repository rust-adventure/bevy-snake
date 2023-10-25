use crate::{assets::FontAssets, board::Board, GameState};
use bevy::{
    prelude::{
        App, OnEnter, OnExit, Plugin, PostStartup, Res,
        ResMut, Resource, Update,
    },
    sprite::Anchor,
};
use std::time::{Duration, Instant};

mod display;
use display::update_score_displays;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Timer>()
            .init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(
                PostStartup,
                display::scorekeeping_ui,
            )
            .add_systems(
                OnEnter(GameState::Playing),
                start_timer,
            )
            .add_systems(
                OnExit(GameState::Playing),
                close_timer,
            )
            .add_systems(Update, update_score_displays);
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Resource,
)]
pub struct Score {
    pub score: u32,
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Resource,
)]
pub struct HighScore {
    pub score: u32,
    pub time: Duration,
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, Resource,
)]
pub struct Timer {
    pub start: Option<Instant>,
    pub runtime: Option<Duration>,
}

fn start_timer(mut timer: ResMut<Timer>) {
    *timer = Timer {
        start: Some(Instant::now()),
        runtime: None,
    };
}

fn close_timer(
    mut timer: ResMut<Timer>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    let elapsed = timer.start.unwrap().elapsed();
    timer.runtime = Some(elapsed);
    if score.score > high_score.score
        || score.score == high_score.score
            && elapsed < high_score.time
    {
        *high_score = HighScore {
            score: score.score,
            time: elapsed,
        }
    }
}
