use crate::data::{Data, Transaction, Transactions};
use crate::ui::Ui;
use chrono::NaiveDate;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, ListState, Padding, ScrollbarState, TableState},
};
use std::error::Error;
use std::time::{Duration, Instant};
use tui_textarea::TextArea;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Home,
    Transactions,
    AddExpense,
    AddIncome,
    Report,
}

impl ToString for Tab {
    fn to_string(&self) -> String {
        match self {
            Tab::Home => "Home".to_string(),
            Tab::Transactions => "Transactions".to_string(),
            Tab::AddExpense => "Add Expense".to_string(),
            Tab::AddIncome => "Add Income".to_string(),
            Tab::Report => "Report".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive()]
pub struct App {
    pub data: Data,
    pub current_tab: Tab,
    pub tabs: Vec<Tab>,
    pub transactions: Transactions,
    pub amount_input: TextArea<'static>,
    pub category_input: TextArea<'static>,
    pub date_input: TextArea<'static>,
    pub description_input: TextArea<'static>,
    pub active_input: usize,
    pub cursor_visible: bool,
    pub last_tick: Instant,
    pub transaction_type: TransactionType,
    pub input_modified: [bool; 4],
    pub expense_categories: &'static [&'static str],
    pub income_categories: &'static [&'static str],
    pub table_state: TableState,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub category_list_state: ListState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum TransactionType {
    Expense,
    Income,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let data = Data::new("transactions.json")?;
        let transactions = data.get_transactions().clone();
        let mut app = Self {
            data,
            current_tab: Tab::Home,
            input_modified: [false; 4],
            tabs: vec![
                Tab::Home,
                Tab::Transactions,
                Tab::AddExpense,
                Tab::AddIncome,
                Tab::Report,
            ],
            transaction_type: TransactionType::Expense,
            amount_input: TextArea::default(),
            category_input: TextArea::default(),
            date_input: TextArea::default(),
            description_input: TextArea::default(),
            active_input: 0,
            cursor_visible: false,
            last_tick: Instant::now(),
            transactions: Transactions {
                expenses: transactions.expenses,
                income: transactions.income,
                expense_categories: vec![
                    "Food".to_string(),
                    "Housing".to_string(),
                    "Transportation".to_string(),
                    "Entertainment".to_string(),
                    "Health".to_string(),
                    "Bills".to_string(),
                    "Other".to_string(),
                ],
                income_categories: vec![
                    "Salary".to_string(),
                    "Interest".to_string(),
                    "Gifts".to_string(),
                    "Other".to_string(),
                ],
            },
            table_state: TableState::default(),
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            expense_categories: &[
                "Food",
                "Housing",
                "Transportation",
                "Entertainment",
                "Health",
                "Bills",
                "Other",
            ],
            income_categories: &["Salary", "Interest", "Gifts", "Other"],
            category_list_state: ListState::default(),
        };
        app.reset_inputs();
        Ok(app)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ui = Ui::new()?;
        loop {
            self.update_cursor();
            ui.draw(self)?;
            if event::poll(Duration::from_millis(100))? {
                if self.handle_input()? {
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle_input(&mut self) -> Result<bool, Box<dyn Error>> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Tab => self.next_tab(),
                    KeyCode::BackTab => self.previous_tab(),
                    KeyCode::Esc => return Ok(true), // Quit the app
                    _ => self.handle_tab_specific_input(key),
                }
            }
        }
        Ok(false)
    }

    fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Home => Tab::Transactions,
            Tab::Transactions => Tab::AddExpense,
            Tab::AddExpense => Tab::AddIncome,
            Tab::AddIncome => Tab::Home,
            Tab::Report => Tab::Transactions,
        };
        self.reset_inputs();
    }

    fn previous_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Home => Tab::Report,
            Tab::Report => Tab::AddIncome,
            Tab::AddIncome => Tab::AddExpense,
            Tab::AddExpense => Tab::Transactions,
            Tab::Transactions => Tab::Home,
        };
        self.reset_inputs();
    }

    fn handle_tab_specific_input(&mut self, key: event::KeyEvent) {
        match self.current_tab {
            Tab::Transactions => match key.code {
                KeyCode::Down => {
                    // Scroll down the table (adjust table state).
                    self.table_state.scroll_down_by(1);
                    if let Some(selected_row) = self.table_state.selected() {
                        // The `vertical_scroll` MUST match the selected row!
                        self.vertical_scroll = selected_row;
                    } else {
                        // If table state isn't showing anything...
                        self.vertical_scroll = 0;
                        self.table_state.select(Some(self.vertical_scroll)) // Default the table_state
                    }
                    self.vertical_scroll_state =
                        self.vertical_scroll_state.position(self.vertical_scroll);
                }
                KeyCode::Up => {
                    // Scroll up the table
                    self.table_state.scroll_up_by(1);

                    // Update vertical scroll position (always based on selected table row)
                    if let Some(selected_row) = self.table_state.selected() {
                        if selected_row > 0 {
                            // Need to be on row 1 or above for this to work
                            self.vertical_scroll = selected_row - 1; // Make sure your scroll value matches the selected row in your table
                        }
                    } else {
                        // Handling for the case where no rows are selected.
                        self.vertical_scroll = 0; // Or, set the vertical_scroll position as required
                    }
                    self.vertical_scroll_state =
                        self.vertical_scroll_state.position(self.vertical_scroll);
                }
                _ => {
                    self.input_to_active_field(key);
                }
            },
            Tab::AddExpense | Tab::AddIncome => match key.code {
                KeyCode::Down => {
                    self.next_input();
                }
                KeyCode::Up => {
                    self.previous_input();
                }
                KeyCode::Enter => {
                    if let Ok(true) = self.submit_transaction() {
                        self.current_tab = Tab::Transactions;
                        self.reset_inputs();
                    }
                }
                _ => {
                    self.input_to_active_field(key);
                }
            },
            Tab::Report => {} // Handle Report tab inputs if necessary
            Tab::Home => {}
        }
    }

    fn next_input(&mut self) {
        self.active_input = (self.active_input + 1) % 4;
    }

    fn previous_input(&mut self) {
        self.active_input = (self.active_input + 3) % 4;
    }

    fn input_to_active_field(&mut self, key: event::KeyEvent) {
        let input_received = match self.active_input {
            0 => {
                let input_received = self.amount_input.input(key);
                if input_received {
                    self.input_modified[0] = true;
                    let input = self.amount_input.lines()[0].clone();
                    if !input.is_empty() {
                        self.validate_amount(&input);
                    }
                }
                input_received
            }
            1 => {
                let input_received = self.category_input.input(key);
                if input_received {
                    self.input_modified[1] = true;
                    let input = self.category_input.lines()[0].clone();
                    if !input.is_empty() {
                        self.validate_category(&input);
                    }
                }
                input_received
            }
            2 => {
                let input_received = self.date_input.input(key);
                if input_received {
                    self.input_modified[2] = true;
                    let input = self.date_input.lines()[0].clone();
                    if !input.is_empty() {
                        self.validate_date(&input);
                    }
                }
                input_received
            }
            3 => {
                let input_received = self.description_input.input(key);
                if input_received {
                    self.input_modified[3] = true;
                    let input = self.description_input.lines()[0].clone();
                    if !input.is_empty() {
                        self.validate_description(&input);
                    }
                }
                input_received
            }
            _ => false,
        };

        if input_received {
            self.cursor_visible = true;
            self.last_tick = Instant::now();
        }
    }

    // Validation function for Amount
    fn validate_amount(&mut self, input: &str) -> bool {
        if let Err(err) = input.parse::<f64>() {
            // Set ERROR styles for Amount
            self.amount_input
                .set_style(Style::default().fg(Color::LightRed)); // This assumes that the styling for a valid amount is something else
            self.amount_input.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::LightRed)
                    // .title("ERROR: Invalid Amount"),
                    .title(format!("ERROR: {}", err)),
            );
            false // Validation failed
        } else {
            // Set OK styles for Amount
            self.amount_input
                .set_style(Style::default().fg(Color::LightGreen)); // Make the change to good styling here
            self.amount_input.set_block(
                Block::default()
                    .border_style(Color::LightGreen)
                    .borders(Borders::ALL)
                    .title("OK"),
            );
            true // Validation succeeded
        }
    }

    // Validation function for Category
    fn validate_category(&mut self, input: &str) -> bool {
        // Check for empty string or invalid characters (You might want to expand this check)
        if input.is_empty()
            || input
                .chars()
                .any(|c| !c.is_alphanumeric() && !c.is_whitespace())
        {
            // Set ERROR styles for Category
            self.category_input
                .set_style(Style::default().fg(Color::LightRed));
            self.category_input.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::LightRed)
                    .title("ERROR: Invalid Category"), // Change block title to the error
            );
            false
        } else {
            // Set OK styles for Category
            self.category_input
                .set_style(Style::default().fg(Color::LightGreen));
            self.category_input.set_block(
                Block::default()
                    .border_style(Color::LightGreen)
                    .borders(Borders::ALL)
                    .title("OK"), // Change the title
            );
            true
        }
    }

    // Validation function for Date
    fn validate_date(&mut self, input: &str) -> bool {
        if let Err(_) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            // Set ERROR styles for Date
            self.date_input
                .set_style(Style::default().fg(Color::LightRed));
            self.date_input.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::LightRed)
                    .title("ERROR: Invalid Date Format"),
            );
            false
        } else {
            // Set OK styles for Date
            self.date_input
                .set_style(Style::default().fg(Color::LightGreen));
            self.date_input.set_block(
                Block::default()
                    .border_style(Color::LightGreen)
                    .borders(Borders::ALL)
                    .title("OK"),
            );
            true
        }
    }
    // Validation function for Description
    fn validate_description(&mut self, input: &str) -> bool {
        if !input.is_empty() {
            // Set OK styles for Description
            self.description_input
                .set_style(Style::default().fg(Color::LightGreen));
            self.description_input.set_block(
                Block::default()
                    .border_style(Color::LightGreen)
                    .borders(Borders::ALL)
                    .title("OK"),
            );
            true
        } else {
            // Set ERROR styles for Description
            self.description_input
                .set_style(Style::default().fg(Color::LightRed));
            self.description_input.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Color::LightRed)
                    .title("ERROR: Enter a Description"),
            );
            false
        }
    }

    fn reset_inputs(&mut self) {
        let amount_title = match self.current_tab {
            Tab::AddExpense => " Enter Expense Amount",
            Tab::AddIncome => " Enter Income Amount",
            _ => " Enter Amount",
        };

        self.table_state = TableState::default();
        self.amount_input = TextArea::default();
        self.amount_input.set_block(
            Block::default()
                .title(amount_title)
                .title_style(Style::default().fg(Color::Yellow).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        );
        self.category_input = TextArea::default();
        self.category_input.set_block(
            Block::default()
                .title(" Enter Category")
                .title_style(Style::default().fg(Color::Yellow).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        );
        self.date_input = TextArea::default();
        self.date_input.set_block(
            Block::default()
                .title(" Enter Date (YYYY-MM-DD)")
                .title_style(Style::default().fg(Color::Yellow).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        );
        self.description_input = TextArea::default();
        self.description_input.set_block(
            Block::default()
                .title(" Enter Description")
                .title_style(Style::default().fg(Color::Yellow).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 0, 0)),
        );
        // Reset styling for all inputs
        for textarea in [
            &mut self.amount_input,
            &mut self.category_input,
            &mut self.date_input,
            &mut self.description_input,
        ] {
            textarea.set_style(Style::default());
            textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow))
                    .border_type(BorderType::Rounded)
                    .padding(Padding::new(1, 1, 0, 0)),
            );
        }

        self.input_modified = [false; 4];
        self.active_input = 0;
        self.cursor_visible = false;
        self.last_tick = Instant::now();

        // Initial validation
        let amount = self.amount_input.lines()[0].clone();
        self.validate_amount(&amount);
        let category = self.category_input.lines()[0].clone();
        self.validate_category(&category);
        let date = self.date_input.lines()[0].clone();
        self.validate_date(&date);
        let description = self.description_input.lines()[0].clone();
        self.validate_description(&description);
    }

    pub fn update_cursor(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_tick) >= Duration::from_millis(150) {
            self.cursor_visible = !self.cursor_visible;
            self.last_tick = now;
        }
    }

    pub fn get_active_textarea(&self) -> &TextArea<'static> {
        match self.active_input {
            0 => &self.amount_input,
            1 => &self.category_input,
            2 => &self.date_input,
            3 => &self.description_input,
            _ => unreachable!(),
        }
    }

    fn submit_transaction(&mut self) -> Result<bool, Box<dyn Error>> {
        // Get input values
        let amount_input_str = self.amount_input.lines()[0].clone();
        let category_input_str = self.category_input.lines()[0].clone();
        let date_input_str = self.date_input.lines()[0].clone();
        let description_input_str = self.description_input.lines()[0].clone();

        let valid_amount = !self.input_modified[0] || self.validate_amount(&amount_input_str);
        let valid_category = !self.input_modified[1] || self.validate_category(&category_input_str);
        let valid_date = !self.input_modified[2] || self.validate_date(&date_input_str);
        let valid_description =
            !self.input_modified[3] || self.validate_description(&description_input_str);

        if !valid_amount || !valid_category || !valid_date || !valid_description {
            return Ok(false);
        }

        let amount = match self.current_tab {
            Tab::AddExpense => -amount_input_str.parse::<f64>()?,
            Tab::AddIncome => amount_input_str.parse::<f64>()?,
            _ => unreachable!(),
        };
        let date = NaiveDate::parse_from_str(&date_input_str, "%Y-%m-%d")?;

        let category = match self.current_tab {
            Tab::AddExpense => self
                .expense_categories
                .iter()
                .find(|c| c.trim() == category_input_str.trim())
                .map(|s| *s)
                .unwrap_or_else(|| "Other"),
            Tab::AddIncome => self
                .income_categories
                .iter()
                .find(|c| c.trim() == category_input_str.trim())
                .map(|s| *s)
                .unwrap_or_else(|| "Other"),
            _ => unreachable!(),
        };

        // Create the Transaction
        let transaction = Transaction {
            id: match self.current_tab {
                Tab::AddExpense => self.transactions.expenses.len() as u32 + 1,
                Tab::AddIncome => self.transactions.income.len() as u32 + 1,
                _ => unreachable!(),
            },
            amount,
            category: description_input_str.clone(),
            date,
            description: description_input_str.clone(),
        };

        self.data.add_transaction(
            transaction.clone(),
            match self.current_tab {
                Tab::AddExpense => "expenses",
                Tab::AddIncome => "income",
                _ => unreachable!(),
            },
        )?;

        match self.current_tab {
            Tab::AddExpense => self.transactions.expenses.push(transaction),
            Tab::AddIncome => self.transactions.income.push(transaction),
            _ => unreachable!(),
        };

        // If successful, you might want to signal this (perhaps by changing a flag in App for UI updates)
        Ok(true)
    }
}
