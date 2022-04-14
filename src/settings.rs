#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameSpeed {
    SLOW,
    REGULAR,
    FAST,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameSettings {
    pub speed: GameSpeed,
    pub speedrun_mode: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            speed: GameSpeed::REGULAR,
            speedrun_mode: false,
        }
    }
}
