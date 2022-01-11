use std::{
    error::Error,
    fs::File,
    io::{self, Write},
    process::Command,
};

use clap::Parser;

/// Brainfuck compiler
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// File to compile
    in_file: String,

    /// Size of the data array
    #[clap(short, long, default_value_t = 1024)]
    data_size: usize,

    #[clap(short, long)]
    out_file: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let out_file = match args.out_file {
        Some(file) => file,
        None => args
            .in_file
            .strip_suffix(".bf")
            .ok_or("If no output file is provided, the input file needs to end in .bf")?
            .to_string(),
    };
    let brainfuck = std::fs::read_to_string(&args.in_file)?;
    let mut rust_code = String::new();
    let mut bracket_count: u8 = 0;
    for c in brainfuck.chars() {
        rust_code.push_str(match c {
            '+' => "data[ptr] = data[ptr].wrapping_add(1);\n",
            '-' => "data[ptr] = data[ptr].wrapping_sub(1);\n",
            '>' => "ptr = (ptr + 1) % DATA_SIZE;\n",
            '<' => "ptr = ptr.checked_sub(1).unwrap_or(DATA_SIZE - 1);\n",
            '[' => {
                bracket_count = bracket_count
                    .checked_add(1)
                    .ok_or("Bracket nesting depth exceeded")?;
                "while data[ptr] > 0 {\n"
            }
            ']' => {
                bracket_count = bracket_count
                    .checked_sub(1)
                    .ok_or("Unmatched closing bracket")?;
                "}\n"
            }
            ',' => "data[ptr] = stdin.next().expect(\"Not enough input\")?;\n",
            '.' => "stdout.write(&[data[ptr]])?;\n",
            _ => "",
        })
    }
    if bracket_count > 0 {
        return Err("Unmatched opening bracket".into());
    }
    let temp_rs_file = "/tmp/temp.rs";
    write!(
        File::create(temp_rs_file)?,
        include_str!("template.txt"),
        args.data_size,
        rust_code
    )?;
    let output = Command::new("rustc")
        .args(["-o", &out_file, "-O", temp_rs_file])
        .output()?;
    if !output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        return Err("The rust compiler failed".into());
    }
    Ok(())
}
