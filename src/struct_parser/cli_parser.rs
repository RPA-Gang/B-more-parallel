use anyhow::Context;
use clap::Parser;

const UPPER_LIMIT: u32 = 100_000_000;

#[derive(Parser)]
pub struct Cli {
    pub value: u32,
}

impl Cli {
    pub fn new() -> Self {
        Cli::info_printout();
        let val = self::Cli::arguments_given().ensure_valid_upperbound();
        self::Cli { value: val }
    }

    fn info_printout() {
        println!("\nYou can assign values via the prompt, or pass them in on the command line");
        println!(
            "The maximum value available is {}. \n
        NOTE: Anything under 20 will complete quickly, but anything over 40 will take a long time.",
            UPPER_LIMIT
        );
    }

    fn arguments_given() -> Self {
        match std::env::args().len() {
            1 => {
                let mut input = String::new();
                println!("Enter a value: ");
                let _ = std::io::stdin()
                    .read_line(&mut input)
                    .with_context(|| ("Failed to read line from stdin"));
                let input = input.trim();
                match input.parse::<u32>() {
                    Ok(val) => Cli { value: val },
                    Err(_) => {
                        println!("Invalid input, please enter a number");
                        pause();
                        std::process::exit(1);
                    }
                }
            }
            _ => Cli::parse(),
        }
    }

    fn ensure_valid_upperbound(&self) -> u32 {
        match self.value {
            0..=UPPER_LIMIT => self.value,
            _ => {
                println!("Value must be between 0 and {}", UPPER_LIMIT);
                std::process::exit(1);
            }
        }
    }
}

pub fn pause() {
    let mut input = String::new();
    println!("Press enter to continue...");
    let _ = std::io::stdin()
        .read_line(&mut input)
        .with_context(|| ("Failed to read line from stdin"));
}
