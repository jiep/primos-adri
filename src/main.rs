use clap::{Args, Parser, Subcommand};
use primos_adri::{check_if_primo_adri, generate_primos_adri};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check(Check),
    Generate(Generate),
}

#[derive(Args)]

/// Generate Primos de Adri with given digits
struct Generate {
    #[clap(short, long, default_value_t = 5)]
    #[arg(value_parser = clap::value_parser!(u8).range(2..))]
    digits: u8,
}

#[derive(Args)]
/// Check if a given number is Primo de Adri
struct Check {
    #[clap(short, long)]
    number: u32,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(args) => {
            let maximum = 10_u64.pow(args.digits as u32);

            let primos_adri = generate_primos_adri(args.digits);

            let number_of_primos_adri = primos_adri.len();
            let primos_adri = primos_adri
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            println!("{number_of_primos_adri} Primos de Adri up to {maximum}:\n{primos_adri}");
        }
        Commands::Check(args) => {
            let result = check_if_primo_adri(args.number.into());

            println!(
                "{} is{} a Primo de Adri!",
                args.number,
                if result { "" } else { " NOT" }
            );
        }
    }
}
