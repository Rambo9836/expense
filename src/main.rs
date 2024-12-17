use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

struct ExpenseTracker {
    expenses: HashMap<String, f64>,
}

impl ExpenseTracker {
    fn new() -> Self {
        ExpenseTracker {
            expenses: HashMap::new(),
        }
    }

    fn add_expense(&mut self, category: String, amount: f64) {
        let counter = self.expenses.entry(category).or_insert(0.0);
        *counter += amount;
    }

    fn view_report(&self) {
        println!("Expenses Report:");
        for (category, amount) in &self.expenses {
            println!("{}: {:.2}", category, amount);
        }
    }

    fn export_to_csv(&self, filename: &str) {
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        for (category, amount) in &self.expenses {
            if let Err(why) = writeln!(file, "{},{}", category, amount) {
                panic!("couldn't write to {}: {}", display, why)
            }
        }

        println!("Data exported to {}", filename);
    }
}

fn main() {
    let mut tracker = ExpenseTracker::new();
    loop {
        println!("Choose an option: (1) Add Expense (2) View Report (3) Export to CSV (4) Exit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => {
                let mut category = String::new();
                let mut amount = String::new();

                println!("Enter category:");
                io::stdin().read_line(&mut category).expect("Failed to read line");

                println!("Enter amount:");
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64 = amount.trim().parse().expect("Invalid input");

                tracker.add_expense(category.trim().to_string(), amount);
            }
            2 => {
                tracker.view_report();
            }
            3 => {
                println!("Enter filename to export to (e.g., expenses.csv):");
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).expect("Failed to read line");
                tracker.export_to_csv(filename.trim());
            }
            4 => break,
            _ => println!("Invalid choice!"),
        }
    }
}
