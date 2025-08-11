use ratatui::{
    prelude::*,
    widgets::{Paragraph, Block, Borders},
    style::{Style, Color},
    text::{Span, Line},
};
use crate::game::state::GameState;
use crate::game::scene::GameScene;

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &GameState, area: Rect) {
    let w = area.width as usize;
    let h = area.height as usize;
    if w == 0 || h == 0 {
        return;
    }

    let mut rows: Vec<Vec<Span>> = vec![vec![Span::raw(" "); w]; h];

    let paused = state.scene == GameScene::Paused;
    let speed = (state.base_speed + state.elapsed.as_secs_f32() * 0.02);
    let hud = format!(
        "Score: {}  Lives: {}  Speed: {:.1} {}",
        state.score,
        state.lives,
        speed,
        if paused { "[Paused]" } else { "" }
    );
    let hud_spans = Line::from(hud);

    let left_border_x = 0usize;
    let right_border_x = w.saturating_sub(1);
    let inner_left = 1usize;
    let inner_width = w.saturating_sub(2).max(1);
    let div1_x = inner_left + inner_width / 3;
    let div2_x = inner_left + (inner_width * 2) / 3;

    let border_style = Style::default().fg(Color::DarkGray);
    let divider_style = Style::default().fg(Color::Gray);

    for y in 1..h {
        rows[y][left_border_x] = Span::styled("|", border_style);
        rows[y][right_border_x] = Span::styled("|", border_style);
        if div1_x < w { rows[y][div1_x] = Span::styled(":", divider_style); }
        if div2_x < w { rows[y][div2_x] = Span::styled(":", divider_style); }
    }

    let enemy_style = Style::default().fg(Color::Red);
    for e in &state.enemies {
        let ex = e.x as usize;
        let ey = e.y.round() as isize;
        if ey >= 1 && (ey as usize) < h && ex < w {
            rows[ey as usize][ex] = Span::styled("V", enemy_style);
        }
    }

    let proj_style = Style::default().fg(Color::Yellow);
    for p in &state.projectiles {
        let px = p.x as usize;
        let py = p.y.round() as isize;
        if py >= 1 && (py as usize) < h && px < w {
            rows[py as usize][px] = Span::styled("^", proj_style);
        }
    }

    let player_style = Style::default().fg(Color::Green);
    let px = state.player.x as usize;
    let py = state.player.y as usize;
    if py < h && px < w {
        rows[py][px] = Span::styled("A", player_style);
    }

    let mut lines: Vec<Line> = Vec::with_capacity(h);
    lines.push(hud_spans);
    for y in 1..h {
        lines.push(Line::from(std::mem::take(&mut rows[y])));
    }

    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(para, area);

    if state.scene == GameScene::GameOver {
        let msg = format!("GAME OVER  Score: {}   [r]=Restart  [Esc/q]=Exit", state.score);
        let msg_line = Line::from(Span::styled(msg, Style::default().fg(Color::White).bg(Color::DarkGray)));
        let overlay = Paragraph::new(msg_line).block(Block::default().borders(Borders::ALL).title("Driving"));
        let ow = (area.width / 2).max(20);
        let oh = 3;
        let ox = area.x + (area.width.saturating_sub(ow)) / 2;
        let oy = area.y + (area.height.saturating_sub(oh)) / 2;
        let orect = Rect::new(ox, oy, ow, oh);
        f.render_widget(overlay, orect);
    }
}