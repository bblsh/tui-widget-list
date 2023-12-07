// Setup code copied from Ratatui's hello_world example

use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};

use tui_widget_list::widget_list::widget_list::WidgetList;
use tui_widget_list::widget_list::widget_list_item::WidgetListItem;
use tui_widget_list::widget_list::widget_list_item::WidgetListItemType;

const DATA2: [(f64, f64); 7] = [
    (0.0, 0.0),
    (10.0, 1.0),
    (20.0, 0.5),
    (30.0, 1.5),
    (40.0, 1.0),
    (50.0, 2.5),
    (60.0, 3.0),
];

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(crate::render_app)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render_app(frame: &mut Frame) {
    let str_1 = "This is a test with a long line that should be wrappable depending on the terminal width. Hello!";
    let str_2 = "Testing again with another Paragraph";

    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&DATA2)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Chart 3".cyan().bold())
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 50.0])
                .labels(vec!["0".bold(), "25".into(), "50".bold()]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec!["0".bold(), "2.5".into(), "5".bold()]),
        );

    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&DATA2)];

    let widget_list_items = vec![
        WidgetListItem::new(
            WidgetListItemType::Paragraph(Paragraph::new(str_1).red().wrap(Wrap { trim: false })),
            frame.size().width as usize,
            textwrap::wrap(str_1, frame.size().width as usize).len(),
        ),
        WidgetListItem::new(
            WidgetListItemType::Paragraph(Paragraph::new(str_2).green()),
            frame.size().width as usize,
            textwrap::wrap(str_2, frame.size().width as usize).len(),
        ),
        WidgetListItem::new(
            WidgetListItemType::Chart(
                Chart::new(datasets)
                    .block(
                        Block::default()
                            .title(Span::styled(
                                "Chart 3",
                                Style::default()
                                    .fg(Color::Cyan)
                                    .add_modifier(Modifier::BOLD),
                            ))
                            .borders(Borders::ALL),
                    )
                    .x_axis(
                        Axis::default()
                            .title("X Axis")
                            .style(Style::default().fg(Color::Gray))
                            .bounds([0.0, 50.0])
                            .labels(vec![
                                Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                                Span::raw("25"),
                                Span::styled("50", Style::default().add_modifier(Modifier::BOLD)),
                            ]),
                    )
                    .y_axis(
                        Axis::default()
                            .title("Y Axis")
                            .style(Style::default().fg(Color::Gray))
                            .bounds([0.0, 5.0])
                            .labels(vec![
                                Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                                Span::raw("2.5"),
                                Span::styled("5", Style::default().add_modifier(Modifier::BOLD)),
                            ]),
                    ),
            ),
            frame.size().width as usize,
            15,
        ),
    ];

    let widget_list = WidgetList::from(widget_list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Widget List "),
    );

    frame.render_widget(widget_list, frame.size());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
