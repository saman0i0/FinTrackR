use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Struct representing a single transaction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: u32,             // Unique identifier for the transaction
    pub amount: f64,         // Amount of money for the transaction
    pub category: String,    // Category of the transaction (e.g., "Food", "Salary")
    pub date: NaiveDate,     // Date of the transaction
    pub description: String, // Description or notes about the transaction
}

// Struct for holding a collection of transactions and categories
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transactions {
    pub expenses: Vec<Transaction>,      // List of expense transactions
    pub income: Vec<Transaction>,        // List of income transactions
    pub expense_categories: Vec<String>, // List of categories for expenses
    pub income_categories: Vec<String>,  // List of categories for income
}

// Struct representing the overall data structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataStructure {
    pub transactions: Transactions, // Transactions and categories data
}

// Struct for managing data with file persistence
pub struct Data {
    data: DataStructure, // Data structure holding transactions and categories
    file_path: String,   // Path to the file where data is stored
}

impl Data {
    // Constructor to create a new Data instance, loading or creating the file if necessary
    pub fn new(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Check if the file exists
        if !Path::new(file_path).exists() {
            // Create a new default data structure
            let data = DataStructure {
                transactions: Transactions {
                    expenses: Vec::new(), // Initialize empty list of expenses
                    income: Vec::new(),   // Initialize empty list of income
                    // Set initial expense categories
                    expense_categories: vec![
                        "Food".to_string(),
                        "Housing".to_string(),
                        "Transportation".to_string(),
                        "Entertainment".to_string(),
                        "Health".to_string(),
                        "Bills".to_string(),
                        "Other".to_string(),
                    ],
                    // Set initial income categories
                    income_categories: vec![
                        "Salary".to_string(),
                        "Interest".to_string(),
                        "Gifts".to_string(),
                        "Other".to_string(),
                    ],
                },
            };

            // Save the new data structure to the file
            let json = serde_json::to_string(&data)?;
            fs::write(file_path, json)?;
        }

        // Load existing or newly created data
        let contents = fs::read_to_string(file_path)?;
        let data = serde_json::from_str(&contents)?;

        Ok(Self {
            data,
            file_path: file_path.to_string(),
        })
    }

    // Method to add a transaction to either expenses or income
    pub fn add_transaction(
        &mut self,
        transaction: Transaction,
        transaction_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match transaction_type.to_lowercase().as_str() {
            "expenses" => self.data.transactions.expenses.push(transaction),
            "income" => self.data.transactions.income.push(transaction),
            _ => return Err("Invalid transaction type".into()), // Return an error if the type is invalid
        }
        self.save()?;
        Ok(())
    }

    // Method to get a reference to the transactions data
    pub fn get_transactions(&self) -> &Transactions {
        &self.data.transactions
    }

    // Helper method to save the current state of data to the file
    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string(&self.data)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }
}
