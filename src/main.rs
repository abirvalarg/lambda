mod args;

fn run(args: args::Args) -> Result<(), Box<dyn std::error::Error>> {
    if args.show_help {
    } else if args.show_version {
    } else {
        let mut vm: lambda::Lambda = Default::default();
        vm.set_input(|_promt, _index| Err(Box::new(lambda::error::NotImplemented) as Box<dyn std::error::Error>));
    }
    Ok(())
}

fn main() {
    use std::process::exit;
    match args::Args::from_cmd() {
        Ok(args) => {
            if let Err(err) = run(args) {
                eprintln!("Error: {}", err);
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
}
