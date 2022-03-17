mod error;

pub struct Args {
	pub exec: String,
	pub show_help: bool,
	pub show_version: bool,
	pub file: Option<String>
}

impl Args {
	pub fn from_cmd() -> Result<Args, Box<dyn std::error::Error>> {
		let args: Vec<String> = std::env::args().collect();
		let mut show_help = false;
		let mut show_version = false;
		let mut file = None;
		let exec = if args.len() > 0 {
			args[0].clone()
		} else {
			"<program>".into()
		};
		for arg in &args[1..] {
			if arg.len() != 0 && arg.as_bytes()[0] == '-' as u8 {
				match arg.as_str() {
					"-h" | "--help" => show_help = true,
					"-v" | "--version" => show_version = true,
					_ => return Err(Box::new(error::UnknownFlag(arg.clone())))
				}
			} else if file == None {
				file = Some(arg.clone());
			} else {
				return Err(Box::new(error::UnexpectedArg(arg.clone())));
			}
		}
		Ok(Args {
			exec,
			show_version,
			show_help,
			file
		})
	}
}
