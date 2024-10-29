use std::{
	collections::BTreeMap,
	env,
	ffi::OsString,
	io,
	process::{
		self,
		Command,
	},
};

const HELP: &str = "Set environment variables and run a command
USAGE: env [NAME=VALUE...] [--] [COMMAND] [ARGS]

Without a command, prints environment variables.
With a command, sets the environment variables if any, then runs it.";

fn convert_env_var((k, v): (OsString, OsString)) -> Option<(String, String)> {
	use std::borrow::Cow::*;
	let k = match k.to_string_lossy() {
		Borrowed(k) => k.to_string(),
		Owned(s) => {
			eprintln!("warning: the variable {s} is not valid unicode");
			return None;
		}
	};

	match v.to_string_lossy() {
		Borrowed(v) => Some((k, v.into())),
		Owned(_) => {
			eprintln!("warning: the value of the variable {k} is not valid unicode");
			None
		}
	}
}

fn run() -> io::Result<i32> {
	let mut args = Vec::with_capacity(8);
	let mut rest = false;
	for a in env::args_os().skip(1) {
		if rest {
			args.push(a);
			continue;
		}
		match a.to_str() {
			None => {
				rest = true;
				args.push(a);
			}
			Some("-h" | "--help") => {
				println!("{HELP}");
				return Ok(0);
			}
			Some("--") => rest = true,
			Some(s) => match s.split_once('=') {
				None | Some(("", _)) => {
					rest = true;
					args.push(a);
				}
				Some((k, v)) => unsafe { env::set_var(k, v) },
			},
		}
	}

	if args.is_empty() {
		env::vars_os()
			.flat_map(convert_env_var)
			.map(|t| (t.0.to_uppercase(), t))
			.collect::<BTreeMap<_, _>>()
			.into_values()
			.for_each(|(k, v)| println!("{k}={v}"));

		return Ok(0);
	}

	Ok(Command::new(&args[0])
		.args(&args[1..])
		.status()?
		.code()
		.unwrap_or(-2))
}

fn main() {
	process::exit(run().unwrap_or_else(|e| {
		eprintln!("env: error: {e}");
		-1
	}))
}
