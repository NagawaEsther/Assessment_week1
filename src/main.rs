use std::collections::HashMap;
use std::io;

/// Stores the bill information
#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

/// Reads a line of input from the user and returns it trimmed
fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_owned()
}

/// Displays the main menu and returns the user's choice
fn main_menu() {
    println!();
    println!("== Bill Manager ==");
    println!("1. Add bill");
    println!("2. View bills");
    println!("3. Remove bill");
    println!("4. Edit bill");
    println!("5. Exit");
    println!();
    print!("Enter selection: ");
    // flush stdout so the prompt appears before input
    use std::io::Write;
    io::stdout().flush().unwrap();
}

/// Adds a new bill to the HashMap
fn add_bill(bills: &mut HashMap<String, Bill>) {
    println!();
    println!("-- Add a new bill --");

    print!("Bill name: ");
    use std::io::Write;
    io::stdout().flush().unwrap();
    let name = get_input();

    if name.is_empty() {
        println!("Bill name cannot be empty.");
        return;
    }

    print!("Amount owed: ");
    io::stdout().flush().unwrap();
    let amount_input = get_input();

    let amount: f64 = match amount_input.parse() {
        Ok(amt) => amt,
        Err(_) => {
            println!("Invalid amount. Please enter a number.");
            return;
        }
    };

    let bill = Bill {
        name: name.clone(),
        amount,
    };

    bills.insert(name.clone(), bill);
    println!("Bill '{}' added successfully.", name);
}

/// Displays all existing bills
fn view_bills(bills: &HashMap<String, Bill>) {
    println!();
    if bills.is_empty() {
        println!("No bills found.");
        return;
    }

    println!("-- Your Bills --");
    println!("{:<4} {:<20} {:>10}", "#", "Name", "Amount");
    println!("{}", "-".repeat(36));

    for (i, (_, bill)) in bills.iter().enumerate() {
        println!("{:<4} {:<20} {:>10.2}", i + 1, bill.name, bill.amount);
    }

    println!("{}", "-".repeat(36));

    let total: f64 = bills.values().map(|b| b.amount).sum();
    println!("{:<4} {:<20} {:>10.2}", "", "Total", total);
}

/// Removes a bill from the HashMap
fn remove_bill(bills: &mut HashMap<String, Bill>) {
    println!();
    if bills.is_empty() {
        println!("No bills to remove.");
        return;
    }

    println!("-- Remove a bill --");
    view_bills(bills);

    println!();
    print!("Enter the name of the bill to remove (or 'back' to go back): ");
    use std::io::Write;
    io::stdout().flush().unwrap();
    let name = get_input();

    if name.to_lowercase() == "back" {
        println!("Going back to main menu.");
        return;
    }

    match bills.remove(&name) {
        Some(bill) => println!("Bill '{}' (amount: {:.2}) removed.", bill.name, bill.amount),
        None => println!("Bill '{}' not found. Check the name and try again.", name),
    }
}

/// Edits an existing bill in the HashMap
fn edit_bill(bills: &mut HashMap<String, Bill>) {
    println!();
    if bills.is_empty() {
        println!("No bills to edit.");
        return;
    }

    println!("-- Edit a bill --");
    view_bills(bills);

    println!();
    print!("Enter the name of the bill to edit (or 'back' to go back): ");
    use std::io::Write;
    io::stdout().flush().unwrap();
    let name = get_input();

    if name.to_lowercase() == "back" {
        println!("Going back to main menu.");
        return;
    }

    if !bills.contains_key(&name) {
        println!("Bill '{}' not found. Check the name and try again.", name);
        return;
    }

    println!("What would you like to edit?");
    println!("1. Name");
    println!("2. Amount");
    println!("3. Both");
    println!("4. Go back");
    print!("Enter choice: ");
    io::stdout().flush().unwrap();
    let choice = get_input();

    match choice.as_str() {
        "1" => {
            print!("Enter new name: ");
            io::stdout().flush().unwrap();
            let new_name = get_input();
            if new_name.is_empty() {
                println!("Name cannot be empty. No changes made.");
                return;
            }
            // Remove old entry and insert with new name
            if let Some(mut bill) = bills.remove(&name) {
                bill.name = new_name.clone();
                bills.insert(new_name.clone(), bill);
                println!("Bill name updated to '{}'.", new_name);
            }
        }
        "2" => {
            print!("Enter new amount: ");
            io::stdout().flush().unwrap();
            let amount_input = get_input();
            match amount_input.parse::<f64>() {
                Ok(new_amount) => {
                    if let Some(bill) = bills.get_mut(&name) {
                        bill.amount = new_amount;
                        println!("Amount updated to {:.2}.", new_amount);
                    }
                }
                Err(_) => println!("Invalid amount. No changes made."),
            }
        }
        "3" => {
            print!("Enter new name: ");
            io::stdout().flush().unwrap();
            let new_name = get_input();
            if new_name.is_empty() {
                println!("Name cannot be empty. No changes made.");
                return;
            }

            print!("Enter new amount: ");
            io::stdout().flush().unwrap();
            let amount_input = get_input();
            match amount_input.parse::<f64>() {
                Ok(new_amount) => {
                    bills.remove(&name);
                    let bill = Bill {
                        name: new_name.clone(),
                        amount: new_amount,
                    };
                    bills.insert(new_name.clone(), bill);
                    println!("Bill updated: '{}' - {:.2}", new_name, new_amount);
                }
                Err(_) => println!("Invalid amount. No changes made."),
            }
        }
        "4" => {
            println!("Going back to main menu.");
        }
        _ => println!("Invalid choice."),
    }
}

fn main() {
    let mut bills: HashMap<String, Bill> = HashMap::new();

    loop {
        main_menu();
        let choice = get_input();

        match choice.as_str() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => edit_bill(&mut bills),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please enter 1-5."),
        }
    }
}
