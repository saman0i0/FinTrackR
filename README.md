# FinTrackR - A Simple Financial Tracker

  

FinTrackR is a basic command-line tool built with Rust that allows you to track your financial transactions (expenses and income). It provides a user-friendly interface for adding transactions, viewing them in a table format, and a basic chart visualizing income vs. expenses.

  

## Getting Started

  

1.  **Clone the repository:**

git clone https://github.com/saman0i0/FinTrackR

  

2.  **Navigate to the directory:**

cd fintrackr

  

3.  **Build the application:**

cargo build --release

  

4.  **Run the application:**

cargo run --release

  

## Key Features

  

-  **Adding Transactions:**

- Choose from "Add Expense" or "Add Income" tabs.

- Input the amount, category, date (in YYYY-MM-DD format), and a description.

- Input fields are validated to ensure accurate data entry.

- Newly added transactions are automatically saved to the `transactions.json` file for persistence.

  

-  **Viewing Transactions:**

- Navigate to the "Transactions" tab to view all your tracked expenses and income.

- Scroll through the transaction table using the up and down arrow keys or 'j' and 'k'.

  

-  **Chart Visualization:**

- The "Chart" tab displays a bar chart representing your total income and expenses.

  

-  **Instructions:**

- The "Instructions" tab provides a quick guide on how to navigate and use FinTrackR.

  

## Navigation

  

-  **Tabs:** Switch between tabs using the Tab and Shift+Tab keys.

-  **Input Fields:** In the "Add Expense" and "Add Income" tabs, use the down arrow to move to the next input field, and Shift+Tab (or up arrow) to move to the previous field. Press Enter to submit a new transaction.

  

-  **Table Scrolling:** Use the up and down arrow keys to scroll through the Transactions table.

  

-  **Quit:** Press 'q' or Esc to exit the application.

  

## Data Persistence

  

FinTrackR automatically saves all your transactions to a file named `transactions.json`. When you restart the application, the saved data will be loaded, allowing you to track your finances over time.