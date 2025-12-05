use clap::Parser;
use colored::*;
use std::collections::VecDeque;

mod parser;
mod roller;

use parser::DiceExpression;
use roller::RollResult;

#[derive(Parser)]
#[command(name = "dice-roll")]
#[command(about = "CLI dice roller for RPGs with statistics", long_about = None)]
struct Cli {
    #[arg(help = "Dice notation (e.g., 2d6+3, 4d20k3, adv)")]
    expression: String,

    #[arg(short, long, default_value_t = 1, help = "Number of times to roll")]
    count: usize,

    #[arg(short, long, help = "Show detailed calculation steps")]
    verbose: bool,

    #[arg(short, long, help = "Show probability distribution")]
    stats: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut history: VecDeque<(String, i32)> = VecDeque::new();

    match parser::parse(&cli.expression) {
        Ok(expr) => {
            if cli.stats {
                display_statistics(&expr);
            }

            for i in 0..cli.count {
                if cli.count > 1 {
                    println!("\n{} {}:", "Roll".bold(), i + 1);
                }

                let result = roller::roll(&expr);
                display_result(&result, cli.verbose);

                history.push_back((cli.expression.clone(), result.total));
                if history.len() > 10 {
                    history.pop_front();
                }
            }

            if cli.count > 1 {
                display_summary(&history);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

fn display_result(result: &RollResult, verbose: bool) {
    if verbose {
        println!("{}:", "Dice rolls".cyan());
        for (i, roll) in result.rolls.iter().enumerate() {
            println!("  d{}: {}", result.die_size, format_die_value(*roll, result.die_size));
        }

        if !result.kept_indices.is_empty() {
            println!("{}:", "Kept dice".cyan());
            for idx in &result.kept_indices {
                println!("  {}", result.rolls[*idx]);
            }
        }

        if result.modifier != 0 {
            println!("{}: {:+}", "Modifier".cyan(), result.modifier);
        }
    } else {
        print!("Rolls: [");
        for (i, roll) in result.rolls.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", format_die_value(*roll, result.die_size));
        }
        println!("]");
    }

    let total_str = format!("Total: {}", result.total);
    println!("{}", total_str.green().bold());
}

fn format_die_value(value: i32, die_size: i32) -> ColoredString {
    if value == die_size {
        value.to_string().bright_green().bold()
    } else if value == 1 {
        value.to_string().bright_red()
    } else {
        value.to_string().normal()
    }
}

fn display_statistics(expr: &DiceExpression) {
    let stats = roller::calculate_statistics(expr);
    println!("{}:", "Statistics".yellow().bold());
    println!("  Min: {}", stats.min);
    println!("  Max: {}", stats.max);
    println!("  Average: {:.2}", stats.average);
    println!();
}

fn display_summary(history: &VecDeque<(String, i32)>) {
    println!("\n{}:", "Summary".yellow().bold());
    let sum: i32 = history.iter().map(|(_, total)| total).sum();
    let avg = sum as f64 / history.len() as f64;
    println!("  Total: {}", sum);
    println!("  Average: {:.2}", avg);
}
