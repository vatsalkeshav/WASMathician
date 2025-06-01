use std::io::{self, Write};

#[no_mangle]
pub extern "C" fn run() {
    let mut calculator = Calculator::new();
    
    loop {
        print!("\nCalculator> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "exit" | "quit" => break,
            "help" => print_help(),
            _ => handle_input(&mut calculator, input),
        }
        
        println!("Display: {}", calculator.get_display());
    }
}

fn print_help() {
    println!("\nCalculator Commands:");
    println!("  Numbers: Enter any number");
    println!("  Operations: +, -, *, /");
    println!("  Commands:");
    println!("    = : Calculate result");
    println!("    c : Clear calculator");
    println!("    help : Show this help");
    println!("    exit/quit : Exit calculator");
}

fn handle_input(calculator: &mut Calculator, input: &str) {
    match input {
        "=" => calculator.calculate(),
        "c" | "C" => calculator.clear(),
        "+" | "-" | "*" | "/" => calculator.press_operation(input),
        _ => {
            if let Ok(_) = input.parse::<f64>() {
                calculator.press_number(input);
            } else {
                println!("Invalid input. Type 'help' for commands.");
            }
        }
    }
}

pub struct Calculator {
    display: String,
    first_number: Option<f64>,
    operation: Option<String>,
    new_number: bool,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display: String::from("0"),
            first_number: None,
            operation: None,
            new_number: true,
        }
    }

    pub fn get_display(&self) -> String {
        self.display.clone()
    }

    pub fn press_number(&mut self, number: &str) {
        if self.new_number {
            self.display = number.to_string();
            self.new_number = false;
        } else {
            self.display.push_str(number);
        }
    }

    pub fn press_operation(&mut self, op: &str) {
        self.first_number = Some(self.display.parse::<f64>().unwrap_or(0.0));
        self.operation = Some(op.to_string());
        self.new_number = true;
    }

    pub fn calculate(&mut self) {
        if let Some(first) = self.first_number {
            if let Some(op) = &self.operation {
                let second = self.display.parse::<f64>().unwrap_or(0.0);
                let result = match op.as_str() {
                    "+" => first + second,
                    "-" => first - second,
                    "*" => first * second,
                    "/" => if second != 0.0 { first / second } else { 0.0 },
                    _ => second,
                };
                self.display = format!("{}", result);
                self.first_number = None;
                self.operation = None;
                self.new_number = true;
            }
        }
    }

    pub fn clear(&mut self) {
        self.display = String::from("0");
        self.first_number = None;
        self.operation = None;
        self.new_number = true;
    }
} 