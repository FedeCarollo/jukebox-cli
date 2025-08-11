use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCmd {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Fire,
    Pause,
    Exit,
    Restart,
    ToggleMusic,
}

pub fn map_key(ev: KeyEvent) -> Option<GameCmd> {
    if ev.kind != KeyEventKind::Press {
        return None;
    }
    match ev.code {
        KeyCode::Char('a') | KeyCode::Left  => Some(GameCmd::MoveLeft),
        KeyCode::Char('d') | KeyCode::Right => Some(GameCmd::MoveRight),
        KeyCode::Char('w') | KeyCode::Up    => Some(GameCmd::MoveUp),
        KeyCode::Char('s') | KeyCode::Down  => Some(GameCmd::MoveDown),
        KeyCode::Char(' ')                  => Some(GameCmd::Fire),
        KeyCode::Char('p')                  => Some(GameCmd::Pause),
        KeyCode::Esc | KeyCode::Char('q')   => Some(GameCmd::Exit),
        KeyCode::Char('r')                  => Some(GameCmd::Restart),
        KeyCode::Char('m')                  => Some(GameCmd::ToggleMusic),
        _ => None,
    }
}