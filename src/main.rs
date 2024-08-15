use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use hacker_news_tui::{fetch_hacker_news, Submission};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use std::error::Error;
use std::io::{stdout, Stdout};
use std::process::Command;

fn draw_tui(
    submissions: &[Submission],
    selected_index: usize,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| {
        let block = Block::default().title("Hacker News").borders(Borders::ALL);
        f.render_widget(block, f.area());

        // Create a list of submissions
        let items: Vec<ListItem> = submissions
            .iter()
            .map(|submission| {
                ListItem::new(format!("{} by {}", submission.title, submission.author))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Submissions"))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol(">> ");

        // Create and set the ListState
        let mut list_state = ListState::default();
        list_state.select(Some(selected_index)); // Set the selected index

        f.render_stateful_widget(list, f.area(), &mut list_state);
    })?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Fetch submissions and handle errors
    let mut submissions = fetch_hacker_news().unwrap_or_else(|err| {
        eprintln!("Error fetching Hacker News: {}", err);
        Vec::new() // Return an empty vector on error
    });

    let mut selected_index = 0;

    loop {
        draw_tui(&submissions[..], selected_index, &mut terminal)?;

        // Check for user input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => break, // Exit on Esc key
                    KeyCode::Down => {
                        selected_index += 1;
                        if selected_index >= submissions.len() {
                            selected_index = submissions.len().saturating_sub(1);
                            // Prevent going out of bounds
                        }
                    }
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(link) = submissions.get(selected_index).map(|s| &s.link) {
                            if !link.is_empty() {
                                // Open the link in the default web browser
                                if let Err(e) = Command::new("xdg-open").arg(link).spawn() {
                                    eprintln!("Failed to open link: {}", e);
                                }
                            }
                        }
                    }
                    KeyCode::Char('r') => {
                        // Refresh submissions
                        let new_submissions = fetch_hacker_news();
                        if let Ok(subs) = new_submissions {
                            submissions = subs;
                        } else {
                            eprintln!("Failed to refresh submissions.");
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Clean up
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
