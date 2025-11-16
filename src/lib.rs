use pyo3::prelude::*;
use std::io::{self, Write};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}



/// Struct representing an investment
struct Investment {
    principal: f64,
    annual_contribution: f64,
    rate: f64,
    years: u32,
    history: Vec<f64>, // store past results
}

impl Investment {
    fn new(principal: f64, annual_contribution: f64, rate: f64, years: u32) -> Self {
        Self {
            principal,
            annual_contribution,
            rate,
            years,
            history: Vec::new(),
        }
    }

    fn final_amount(&self) -> f64 {
        let mut total = self.principal;
        for _ in 0..self.years {
            total = total * (1.0 + self.rate / 100.0) + self.annual_contribution;
        }
        total
    }

    fn run_terminal_gui(&mut self) {
        let mut input = String::new();

        loop {
            println!("+--------------------------------------+");
            println!("|         Investment Calculator        |");
            println!("+--------------------------------------+");
            println!("1. New calculation");
            println!("2. View history");
            println!("3. Quit");
            print!("Select option: ");
            io::stdout().flush().unwrap();

            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim();

            match choice {
                "1" => {
                    // Get inputs
                    print!("Enter starting amount: $");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    self.principal = input.trim().parse().unwrap_or(0.0);

                    print!("Enter yearly contribution: $");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    self.annual_contribution = input.trim().parse().unwrap_or(0.0);

                    print!("Enter number of years: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    self.years = input.trim().parse().unwrap_or(0);

                    print!("Enter annual interest rate (%): ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    self.rate = input.trim().parse().unwrap_or(0.0);

                    let total = self.final_amount();
                    self.history.push(total);

                    println!("+--------------------------------------+");
                    println!("| Final investment amount: ${:.2}", total);
                    println!("+--------------------------------------+");
                }
                "2" => {
                    println!("+--------------------------------------+");
                    println!("| History of results                   |");
                    if self.history.is_empty() {
                        println!("No calculations yet.");
                    } else {
                        for (i, val) in self.history.iter().enumerate() {
                            println!("{}: ${:.2}", i + 1, val);
                        }
                    }
                    println!("+--------------------------------------+");
                }
                "3" => {
                    println!("Exiting. Goodbye!");
                    break;
                }
                _ => {
                    println!("Invalid choice. Try again.");
                }
            }
        }
    }
}



/// Python interface
#[pyfunction]
fn run_gui() -> PyResult<()> {
    let mut investment = Investment::new(0.0, 0.0, 0.0, 0);
    investment.run_terminal_gui(); // works, we have a mutable instance
    Ok(())
}




/// A Python module implemented in Rust.
#[pymodule]
fn my_new_sprint_4(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run_gui, m)?)?;
    Ok(())
}
