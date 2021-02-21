use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn game_over() -> Paragraph<'static> {
    let text = vec![
        Spans::from(Span::styled("Game Over", Style::default().fg(Color::Red))),
        Spans::from("Press 'q' to quit."),
        Spans::from("Press 'r' to restart."),
    ];
    Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn score_bar(score: u32) -> Paragraph<'static> {
    let text = vec![
        Spans::from("Your score is:"),
        Spans::from(""),
        Spans::from(Span::styled(
            format!("{}", score),
            Style::default().fg(Color::Red),
        )),
    ];
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
