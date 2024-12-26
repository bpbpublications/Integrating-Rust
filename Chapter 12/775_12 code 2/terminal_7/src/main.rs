use tui::{
    widgets::{Block, Borders, Table, Row, Cell},
    layout::{Layout, Constraint, Direction},
    Terminal,
    backend::CrosstermBackend,
    style::{Style, Color}
};
use std::io;

fn main() -> Result<(), io::Error> {
    // Set up terminal and backend
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create rows for the table
    let rows = vec![
        Row::new(vec!["Abhishek", "35", "India"]),
        Row::new(vec!["Vijay", "73", "India"]),
    ];

    let _style = Style::default().bg(Color::Blue);
    let header_cells = ["Name", "Age", "Country"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(_style)
        .height(1)
        .bottom_margin(1);
    // Create a table with the rows
    let table = Table::new(rows)
        .header(header)
        .block(Block::default().title("Sample Table").borders(Borders::ALL))
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Length(20),
            Constraint::Min(10),
        ]);

    // Define layout
    
    terminal.draw(|f| {
        let layout = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .direction(Direction::Vertical)
            .margin(5)
            .split(f.size());
        
        f.render_widget(table, layout[0]);
    })?;
  
    Ok(())
}
