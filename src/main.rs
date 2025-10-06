use clap::Parser;
use std::fs;
use std::io;
use std::io::Read;

/// Execute shell scripts embedded within text.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The input file.  Defaults to the STDIN.
    #[arg(short, long)]
    input: Option<String>,

    /// The output file.  Defaults to the STDOUT.
    #[arg(short, long)]
    output: Option<String>,

    /// The delimiters that mark an embedded script.
    #[arg(short, long, default_value = "!(( ))!")]
    delimiters: String,

    /// Run recursively on script outputs.
    #[arg(short, long)]
    recur: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let delimiters: Vec<&str> = args.delimiters.split(' ').collect();
    let [opening_tag, closing_tag] = delimiters[..] else {
        return Err(format!("Invalid delimiters: {:?}", args.delimiters).into());
    };

    let input_text = match args.input {
        Some(filepath) => fs::read_to_string(filepath)?,
        None => {
            let mut buffer = Vec::new();
            io::stdin().read_to_end(&mut buffer)?;
            String::from_utf8(buffer.to_vec())?
        }
    };

    let output_text =
        corre::run_embedded_scripts(input_text, opening_tag, closing_tag, args.recur)?;

    match args.output {
        Some(filepath) => fs::write(filepath, output_text)?,
        None => println!("{}", output_text),
    }

    Ok(())
}
