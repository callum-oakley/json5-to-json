use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[clap(about, version, arg_required_else_help(true))]
/// Reads JSON5 files and print them to stdout as plain old JSON.
struct Args {
    #[clap(short, long, help = "Pretty-print JSON output")]
    pretty: bool,

    #[clap(min_values(1), help = "Files to load JSON5 data from")]
    files: Vec<String>,
}

fn main() {
    let Args { pretty, files } = Args::parse();

    let mut failed = false;
    for file in files {
        let data = match read(&*file) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("Failed to read file: {}: {}", file, error);
                failed = true;
                continue;
            }
        };

        let value = match json5::from_str::<serde_json::Value>(&data) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Failed to parse JSON: {}: {}", file, error);
                failed = true;
                continue;
            }
        };

        if pretty {
            println!("{:#}", value);
        } else {
            println!("{}", value);
        }
    }

    if failed {
        std::process::exit(1);
    }
}

fn read(file: &str) -> Result<String, std::io::Error> {
    if file == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        std::fs::read_to_string(&file)
    }
}
