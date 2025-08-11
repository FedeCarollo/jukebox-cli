#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameScene {
    Menu,
    Running,
    Paused,
    GameOver,
}