use crate::app::{App, Tab};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Bar, BarChart, BarGroup, Block, BorderType, Borders, Cell, List, ListItem, Padding,
        Paragraph, Row, Scrollbar, ScrollbarOrientation, Table, TableState, Tabs,
    },
    Frame, Terminal,
};
use std::io;
const INFO_TEXT: &str = "(Esc) Quit | (Tab) Next |  (Shift+Tab) Prev  |  (↓) Down  |  (↑) Up  ";
const ITEM_HEIGHT: usize = 4;
pub struct Ui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl Ui {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn draw(&mut self, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| render_ui(f, app))?;
        Ok(())
    }
}

fn render_ui(f: &mut Frame, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("FinTrackR")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    f.render_widget(title, main_chunks[0]);

    let titles: Vec<Span> = app
        .tabs
        .iter()
        .map(|t| Span::styled(t.to_string(), Style::default().fg(Color::Green)))
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Menu ")
                .style(
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .highlight_style(Style::default().bg(Color::LightMagenta))
        .select(app.current_tab as usize)
        .divider("|")
        .padding(" ", " ");
    f.render_widget(tabs, main_chunks[1]);

    match app.current_tab {
        Tab::Home => instruct(f, app, main_chunks[2]),
        Tab::Transactions => render_transactions(f, app, main_chunks[2]),
        Tab::AddExpense | Tab::AddIncome => {
            let category_choices = match app.current_tab {
                Tab::AddExpense => app
                    .expense_categories
                    .iter()
                    .map(|c| Span::styled(*c, Style::default().fg(Color::Yellow)))
                    .collect::<Vec<Span>>(),
                Tab::AddIncome => app
                    .income_categories
                    .iter()
                    .map(|c| Span::styled(*c, Style::default().fg(Color::Yellow)))
                    .collect::<Vec<Span>>(),
                _ => vec![],
            };
            render_add_transaction(f, app, main_chunks[2], app.current_tab);
        }
        Tab::Report => render_chart(f, app, main_chunks[2]),
    }

    //  -------------- FOOTER SECTION --------------
    render_footer(f, main_chunks[3]); // Now use chunks[2]
}

// Now render_footer is an independent function with proper arguments.
fn render_footer(frame: &mut Frame, area: Rect) {
    let footer_text = Paragraph::new(Line::from(INFO_TEXT)) // INFO_TEXT is now in scope
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::Cyan))
                .padding(Padding::new(1, 2, 1, 0)),
        );

    frame.render_widget(footer_text, area);
}

// This function is to render scrollbar:
fn render_scrollbar(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.vertical_scroll_state,
    );
}

fn render_transactions(f: &mut Frame, app: &mut App, area: Rect) {
    let header = Row::new(vec![
        Cell::from(Text::from("Date")).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(Text::from("Amount")).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(Text::from("Category")).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from(Text::from("Description")).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ])
    .height(1)
    .style(Style::default().bg(Color::DarkGray));

    // Create rows for each transaction
    let rows = app
        .transactions
        .expenses
        .iter()
        .chain(app.transactions.income.iter())
        .enumerate()
        .map(|(i, t)| {
            Row::new(vec![
                Cell::from(Text::from(t.date.to_string())).style(Style::default()),
                Cell::from(Text::from(format!("{:.2}$", t.amount))).style(Style::default().fg(
                    if t.amount >= 0.0 {
                        Color::Green
                    } else {
                        Color::Red
                    },
                )),
                Cell::from(Text::from(t.category.clone())).style(Style::default()),
                Cell::from(Text::from(t.description.clone())).style(Style::default()),
            ])
            .height(1)
            .style(if i % 2 == 0 {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default().bg(Color::Black)
            })
        })
        .collect::<Vec<Row>>();

    let mut table_state = TableState::default(); // Create a table state
    table_state.select(Some(0)); // Select the first row initially

    // Build the Table widget
    let transactions_table = Table::new(
        rows.clone(),
        &[
            Constraint::Percentage(25), // Date (25% width)
            Constraint::Percentage(25), // Amount (25% width)
            Constraint::Percentage(25), // Category (25% width)
            Constraint::Percentage(25), // Description (25% width)
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Transactions")
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(Style::default().fg(Color::Cyan))
            .border_type(BorderType::Rounded),
    )
    .highlight_style(Style::default().bg(Color::Cyan).fg(Color::Black))
    .column_spacing(1);

    // Render the table statefully, and manage the scrolling state
    f.render_stateful_widget(transactions_table, area, &mut app.table_state);

    app.vertical_scroll_state = app
        .vertical_scroll_state
        .content_length((rows.len() + 2) * ITEM_HEIGHT);

    app.vertical_scroll_state = app
        .vertical_scroll_state
        .position(app.vertical_scroll * ITEM_HEIGHT);
    render_scrollbar(f, app, area);
}

// working with all cursor visible
fn render_add_transaction(f: &mut Frame, app: &mut App, area: Rect, _current_tab: Tab) {
    let chunks: std::rc::Rc<[Rect]> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(30),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Min(1),
        ])
        .split(area);

    // Render all text areas
    f.render_widget(&app.amount_input, chunks[0]);
    f.render_widget(&app.category_input, chunks[1]);
    f.render_widget(&app.date_input, chunks[2]);
    f.render_widget(&app.description_input, chunks[3]);

    // Render the active text area with blinking cursor
    let active_textarea = app.get_active_textarea();
    let cursor_style = Style::default().fg(if app.cursor_visible {
        Color::Cyan
    } else {
        Color::Reset
    });

    let mut styled_textarea = active_textarea.clone();
    styled_textarea.set_cursor_style(cursor_style);

    f.render_widget(&styled_textarea, chunks[app.active_input]);

    let transaction_type = match app.current_tab {
        Tab::AddExpense => "Expense",
        Tab::AddIncome => "Income",
        _ => unreachable!(),
    };

    let instructions = Paragraph::new(format!(
        "Adding {}.Press Up & Down to switch fields. Press Enter to submit, Esc to Exit",
        transaction_type
    ))
    .style(Style::default().fg(Color::Yellow));
    f.render_widget(instructions, chunks[4]);
}

fn render_chart(f: &mut Frame, app: &App, area: Rect) {
    // Prepare data for the bar chart
    let total_expenses: f64 = app
        .transactions
        .expenses
        .iter()
        .map(|t| t.amount.abs())
        .sum();
    let total_income: f64 = app.transactions.income.iter().map(|t| t.amount).sum();

    let bars = vec![
        Bar::default()
            .value((total_expenses).round() as u64)
            .label(Line::from(Span::styled(
                "Expenses",
                Style::default().fg(Color::Red),
            )))
            .style(Style::default().fg(Color::Red)),
        Bar::default()
            .value((total_income).round() as u64)
            .label(Line::from(Span::styled(
                "Income",
                Style::default().fg(Color::Green),
            )))
            .style(Style::default().fg(Color::Green)),
    ];

    let chart = BarChart::default()
        .block(
            Block::default()
                .title(Line::from("Income vs Expenses").centered())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .data(BarGroup::default().bars(&bars))
        .bar_width(20)
        .bar_gap(2);

    f.render_widget(chart, area);
}

fn instruct(f: &mut Frame, _app: &App, area: Rect) {
    // Create a vector of list items.
    let items = vec![ListItem::new(
        "FinTrackR is a simple command-line application to help you track your finances."
    ),
    ListItem::new("Use the 'Transactions' tab to view your recorded income and expenses."),
    ListItem::new(
        "Add new income and expenses using the 'Add Expense' and 'Add Income' tabs."
    ),
    ListItem::new(
        "Use the 'Report' tab to see a chart of your income and expenses, giving you a quick view of your financial situation."
    ),
    ListItem::new("Navigating between the tabs can be done by pressing left arrow (←) and right arrow keys (→)."),
    ListItem::new(
        "To navigate between the fields inside each form tab, use Tab, Shift+Tab, up arrow key (↑), and down arrow key (↓)."
    ),
    ListItem::new("Press Enter to submit the current form or quit the app using Esc key." ),
];
    // Create a List from the items.
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FinTrackR")
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .style(Style::default().fg(Color::Gray));

    // Render the list.
    f.render_widget(list, area);
}
