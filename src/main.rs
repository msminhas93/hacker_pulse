use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind, Event},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use hacker_pulse::{fetch_hacker_news, Submission};
use open;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use std::error::Error;
use std::io::{stdout, Stdout};

fn draw_tui(
    submissions: &[Submission],
    selected_index: usize,
    current_page: usize,
    total_pages: usize,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| {

        let items: Vec<ListItem> = submissions
            .iter()
            .map(|submission| {
                ListItem::new(format!("{} | ({})", submission.title, submission.link))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Hacker Pulse"))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol(">> ");

        let mut list_state = ListState::default();
        list_state.select(Some(selected_index));

        // Render the list in the available space, leaving the last two lines for pagination and tooltip
        let list_area = Rect::new(0, 0, f.area().width, f.area().height - 2);
        f.render_stateful_widget(list, list_area, &mut list_state);

        // Render the pagination info in the second-to-last line
        let footer_info = format!("Page {} of {} | ↑↓: Navigate | →: Open link | r: Refresh | n: Next Page | p: Prev Page | Esc/q: Quit", current_page, total_pages);
        let footer_paragraph = Paragraph::new(footer_info)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        f.render_widget(
            footer_paragraph,
            Rect::new(0, f.area().height - 2, f.area().width, 1),
        );
    })?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Enable mouse capture
    crossterm::execute!(std::io::stdout(), EnableMouseCapture)?;

    let mut current_page = 1;
    let mut selected_index = 0;

    // Fetch the first page of submissions
    let mut page = fetch_hacker_news(current_page)?;

    loop {
        draw_tui(
            &page.submissions[..],
            selected_index,
            page.current_page,
            page.total_pages,
            &mut terminal,
        )?;

        // Handle user input
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Right => {
                        // Open the link in the browser
                        if let Some(article) = page.submissions.get(selected_index) {
                            if let Err(e) = open::that(&article.link) {
                                eprintln!("Failed to open link: {}", e);
                            }
                        }
                    }
                    KeyCode::Down => {
                        // Move down the list
                        selected_index = (selected_index + 1).min(page.submissions.len() - 1);
                    }
                    KeyCode::Up => {
                        // Move up the list
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // Exit the application
                        break;
                    }
                    KeyCode::Char('r') => {
                        // Refresh the submissions
                        page = fetch_hacker_news(current_page)?;
                        selected_index = 0;
                    }
                    KeyCode::Char('n') => {
                        // Go to next page
                        if current_page < page.total_pages {
                            current_page += 1;
                            page = fetch_hacker_news(current_page)?;
                            selected_index = 0;
                        }
                    }
                    KeyCode::Char('p') => {
                        // Go to previous page
                        if current_page > 1 {
                            current_page -= 1;
                            page = fetch_hacker_news(current_page)?;
                            selected_index = 0;
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

    // Disable mouse capture
    crossterm::execute!(std::io::stdout(), DisableMouseCapture)?;

    Ok(())
}
