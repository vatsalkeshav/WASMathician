use std::io::{self, Write};
use std::f64::consts::PI;
use colored::*;

// Constants for UI
const HEADER_WIDTH: usize = 60;
const DISPLAY_WIDTH: usize = 40;

pub struct Calculator {
    display: String,
    first_number: Option<f64>,
    operation: Option<String>,
    new_number: bool,
    memory: f64,
    history: Vec<String>,
    degree_mode: bool,
    error_message: Option<String>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {
            display: String::from("0"),
            first_number: None,
            operation: None,
            new_number: true,
            memory: 0.0,
            history: Vec::new(),
            degree_mode: true,
            error_message: None,
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
        self.error_message = None;
    }

    pub fn press_operation(&mut self, op: &str) {
        match self.display.parse::<f64>() {
            Ok(num) => {
                self.first_number = Some(num);
                self.operation = Some(op.to_string());
                self.new_number = true;
                self.error_message = None;
            }
            Err(_) => {
                self.error_message = Some("Invalid number".to_string());
            }
        }
    }

    pub fn calculate(&mut self) {
        if let Some(first) = self.first_number {
            if let Some(op) = &self.operation {
                match self.display.parse::<f64>() {
                    Ok(second) => {
                        let result = match op.as_str() {
                            "+" => Ok(first + second),
                            "-" => Ok(first - second),
                            "*" => Ok(first * second),
                            "/" => if second != 0.0 { 
                                Ok(first / second)
                            } else { 
                                Err("Division by zero")
                            },
                            "^" => Ok(first.powf(second)),
                            "root" => if second != 0.0 { 
                                Ok(first.powf(1.0/second))
                            } else {
                                Err("Invalid root")
                            },
                            _ => Ok(second),
                        };

                        match result {
                            Ok(value) => {
                                let operation_str = format!("{} {} {} = {}", first, op, second, value);
                                self.history.push(operation_str);
                                self.display = format!("{}", value);
                                self.error_message = None;
                            }
                            Err(msg) => {
                                self.error_message = Some(msg.to_string());
                            }
                        }
                    }
                    Err(_) => {
                        self.error_message = Some("Invalid number".to_string());
                    }
                }
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
        self.error_message = None;
    }

    // Memory functions
    pub fn memory_store(&mut self) {
        match self.display.parse::<f64>() {
            Ok(value) => {
                self.memory = value;
                println!("{}", "Value stored in memory".green());
            }
            Err(_) => {
                self.error_message = Some("Invalid number for memory".to_string());
            }
        }
    }

    pub fn memory_recall(&mut self) {
        self.display = self.memory.to_string();
        self.new_number = true;
    }

    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
        println!("{}", "Memory cleared".green());
    }

    pub fn memory_add(&mut self) {
        match self.display.parse::<f64>() {
            Ok(value) => {
                self.memory += value;
                println!("{}", format!("Memory updated: {}", self.memory).green());
            }
            Err(_) => {
                self.error_message = Some("Invalid number for memory".to_string());
            }
        }
    }

    // Scientific functions
    pub fn toggle_angle_mode(&mut self) {
        self.degree_mode = !self.degree_mode;
        println!("{}", format!("Angle mode: {}", 
            if self.degree_mode { "Degrees" } else { "Radians" }).blue());
    }

    fn to_radians(&self, x: f64) -> f64 {
        if self.degree_mode {
            x * PI / 180.0
        } else {
            x
        }
    }

    fn to_degrees(&self, x: f64) -> f64 {
        if self.degree_mode {
            x * 180.0 / PI
        } else {
            x
        }
    }

    pub fn scientific_op(&mut self, op: &str) {
        match self.display.parse::<f64>() {
            Ok(x) => {
                let result = match op {
                    "sin" => Ok(self.to_radians(x).sin()),
                    "cos" => Ok(self.to_radians(x).cos()),
                    "tan" => Ok(self.to_radians(x).tan()),
                    "asin" => Ok(self.to_degrees(x.asin())),
                    "acos" => Ok(self.to_degrees(x.acos())),
                    "atan" => Ok(self.to_degrees(x.atan())),
                    "ln" => if x > 0.0 { Ok(x.ln()) } else { Err("Invalid input for ln") },
                    "log" => if x > 0.0 { Ok(x.log10()) } else { Err("Invalid input for log") },
                    "sqrt" => if x >= 0.0 { Ok(x.sqrt()) } else { Err("Cannot calculate square root of negative number") },
                    "%" => Ok(x / 100.0),
                    "!" => {
                        let n = x as u32;
                        if n as f64 != x || n > 20 {
                            Err("Factorial only works with positive integers <= 20")
                        } else {
                            Ok((1..=n).product::<u32>() as f64)
                        }
                    },
                    _ => Ok(x)
                };

                match result {
                    Ok(value) => {
                        let operation_str = format!("{}({}) = {}", op, x, value);
                        self.history.push(operation_str);
                        self.display = format!("{}", value);
                        self.error_message = None;
                    }
                    Err(msg) => {
                        self.error_message = Some(msg.to_string());
                    }
                }
            }
            Err(_) => {
                self.error_message = Some("Invalid number for operation".to_string());
            }
        }
        self.new_number = true;
    }

    pub fn get_display_formatted(&self) -> String {
        let display = if let Some(err) = &self.error_message {
            format!("Error: {}", err).red().to_string()
        } else {
            self.display.bright_green().to_string()
        };

        format!("│ {: >width$} │", display, width = DISPLAY_WIDTH - 2)
    }

    pub fn get_status_line(&self) -> String {
        let mode = if self.degree_mode { "DEG".blue() } else { "RAD".blue() };
        let mem = if self.memory != 0.0 { "M".yellow() } else { " ".normal() };
        format!("│ {: <6} Memory: {: <10} Mode: {} │", 
            mem,
            format!("{:.2}", self.memory).yellow(),
            mode
        )
    }

    pub fn show_history(&self) {
        if self.history.is_empty() {
            println!("{}", "No history available".yellow());
        } else {
            println!("\n{}", "Calculation History:".blue().bold());
            for (i, entry) in self.history.iter().enumerate() {
                println!("{}. {}", (i + 1).to_string().yellow(), entry.bright_white());
            }
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
        println!("{}", "History cleared".green());
    }
}

fn draw_header() {
    println!("{}", "╭────────────────────────────────────────────────────────────╮".bright_blue());
    println!("│{}│", format!("{:^width$}", "Advanced Scientific Calculator".bold(), width = HEADER_WIDTH).bright_blue());
    println!("{}", "├────────────────────────────────────────────────────────────┤".bright_blue());
}

fn draw_footer() {
    println!("{}", "╰────────────────────────────────────────────────────────────╯".bright_blue());
}

fn print_help() {
    draw_header();
    println!("│{}│", format!("{:^width$}", "Available Commands".bold(), width = HEADER_WIDTH).yellow());
    println!("{}", "├────────────────────────────────────────────────────────────┤".bright_blue());

    let sections = [
        ("Basic Operations".bold().green(), vec![
            ("Numbers", "Enter any number"),
            ("+, -, *, /", "Basic arithmetic"),
            ("=", "Calculate result"),
            ("c", "Clear calculator")
        ]),
        ("Memory Functions".bold().yellow(), vec![
            ("ms", "Store in memory"),
            ("mr", "Recall from memory"),
            ("mc", "Clear memory"),
            ("m+", "Add to memory")
        ]),
        ("Scientific Functions".bold().cyan(), vec![
            ("sin, cos, tan", "Trigonometric"),
            ("asin, acos, atan", "Inverse trigonometric"),
            ("ln, log", "Natural and base-10 log"),
            ("sqrt", "Square root"),
            ("!", "Factorial"),
            ("%", "Percentage")
        ]),
        ("Other Commands".bold().magenta(), vec![
            ("mode", "Toggle DEG/RAD"),
            ("hist", "Show history"),
            ("clrhist", "Clear history"),
            ("help", "Show this help"),
            ("exit/quit", "Exit calculator")
        ])
    ];

    for (section, cmds) in sections.iter() {
        println!("│ {} │", section);
        println!("{}", "├────────────────────────────────────────────────────────────┤".bright_blue());
        for (cmd, desc) in cmds {
            println!("│ {: <15} : {: <38}│", 
                cmd.bright_white(),
                desc.bright_white()
            );
        }
        println!("{}", "├────────────────────────────────────────────────────────────┤".bright_blue());
    }
    draw_footer();
}

fn handle_input(calculator: &mut Calculator, input: &str) {
    match input.trim().to_lowercase().as_str() {
        "=" => calculator.calculate(),
        "c" => calculator.clear(),
        "+" | "-" | "*" | "/" | "^" | "root" => calculator.press_operation(input),
        "ms" => calculator.memory_store(),
        "mr" => calculator.memory_recall(),
        "mc" => calculator.memory_clear(),
        "m+" => calculator.memory_add(),
        "mode" => calculator.toggle_angle_mode(),
        "hist" => calculator.show_history(),
        "clrhist" => calculator.clear_history(),
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | 
        "ln" | "log" | "sqrt" | "%" | "!" => calculator.scientific_op(input),
        _ => {
            if let Ok(_) = input.parse::<f64>() {
                calculator.press_number(input);
            } else {
                println!("{}", "Invalid input. Type 'help' for commands.".red());
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn run() {
    let mut calculator = Calculator::new();
    
    draw_header();
    println!("│{}│", 
        format!("{:^width$}", "Type 'help' for available commands".bold(), width = HEADER_WIDTH)
        .bright_yellow()
    );
    println!("{}", "├────────────────────────────────────────────────────────────┤".bright_blue());

    loop {
        // Display current value
        println!("{}", calculator.get_display_formatted());
        println!("{}", calculator.get_status_line());
        println!("{}", "├────────────────────────────────────────────────────────────┤".bright_blue());
        
        // Input prompt
        print!("│ {} ", ">".bright_green());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "exit" | "quit" => {
                println!("│{}│", 
                    format!("{:^width$}", "Goodbye!".bold(), width = HEADER_WIDTH)
                    .bright_yellow()
                );
                draw_footer();
                break;
            }
            "help" => print_help(),
            _ => {
                handle_input(&mut calculator, input);
            }
        }
    }
} 