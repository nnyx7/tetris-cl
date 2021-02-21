use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Wrap};

#[allow(dead_code)]
pub fn game_over_single_player() -> Paragraph<'static> {
    let text = vec![
        Spans::from(Span::styled("Game Over", Style::default().fg(Color::Red))),
        Spans::from("Press 'q' to quit."),
        Spans::from("Press 'r' to restart."),
    ];
    Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn game_over_multiplayer(
    has_game_ended_first: bool,
    has_game_ended_second: bool,
) -> Paragraph<'static> {
    let first_player_wins = Span::styled("First Player Wins!", Style::default().fg(Color::Cyan));
    let second_player_wins =
        Span::styled("Second Player Wins!", Style::default().fg(Color::Magenta));
    let equals = Span::styled("Equals!", Style::default().fg(Color::Yellow));

    let mut buttons_info = vec![
        Spans::from("Press 'q' to quit."),
        Spans::from("Press 'r' to restart."),
    ];

    let mut text = vec![
        Spans::from(Span::styled("Game Over", Style::default().fg(Color::Red))),
        Spans::from(Span::raw("")),
    ];
    if has_game_ended_first && has_game_ended_second {
        text.push(Spans::from(equals));
    } else if has_game_ended_first {
        text.push(Spans::from(second_player_wins));
    } else {
        text.push(Spans::from(first_player_wins));
    }

    text.push(Spans::from(Span::raw("")));
    text.append(&mut buttons_info);

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
