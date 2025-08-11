use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    text::{Line, Span},
};

/// Render control block with key information
pub fn render_controls_block(f: &mut Frame, area: Rect) {
    let controls = vec![
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Red)),
            Span::raw(" - Quit    "),
            Span::styled("p/Enter", Style::default().fg(Color::Green)),
            Span::raw(" - Play/Resume    "),
            Span::styled("s", Style::default().fg(Color::Blue)),
            Span::raw(" - Pause"),
        ]),
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" - Navigate    "),
            Span::styled("+/-", Style::default().fg(Color::Magenta)),
            Span::raw(" - Volume"),
        ]),
        Line::from(vec![
            Span::styled("g", Style::default().fg(Color::Yellow)),
            Span::raw(" - Start Driving Game"),
        ]),
    ];

    let controls_paragraph = Paragraph::new(controls)
        .block(Block::default().title("Controls").borders(Borders::ALL));
    
    f.render_widget(controls_paragraph, area);
}