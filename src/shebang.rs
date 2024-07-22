use std::{
	env,
	fs::File,
	io::{
		self,
		BufRead,
		BufReader,
		ErrorKind,
		Read,
	},
	path::{
		Path,
		PathBuf,
	},
	process::{
		self,
		Command,
	},
};

const BINS: &[&str] = &["/bin/", "/usr/bin/", "/sbin/"];

fn read_shebang(buf: &mut String, p: &Path) -> io::Result<bool> {
	let mut f = File::open(p)?;
	let mut arr = [0, 0];
	match f.read_exact(&mut arr) {
		Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(false),
		Err(e) => return Err(e),
		Ok(()) => (),
	}
	if arr != [b'#', b'!'] {
		return Ok(false);
	}
	let mut f = BufReader::with_capacity(1024, f);
	f.read_line(buf)?;
	Ok(true)
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
	let mut args = env::args_os();
	// discard the first
	let _ = args.next();
	let file: PathBuf = args.next().ok_or("no file to run")?.into();
	let mut shebang = String::with_capacity(1024);
	let mut c = if read_shebang(&mut shebang, &file)? {
		let shebang = shebang.trim();
		if shebang.is_empty() {
			return Err("no interpreter".into());
		}

		let mut words = shebang.split_ascii_whitespace();
		let first = words.next().unwrap();

		// check if we should replace the interpreter
		let mut c = env::var("SHEBANG_BIN")
			.ok()
			.filter(|s| !s.is_empty())
			.and_then(|bin| {
				let bin = bin.trim_end_matches(['\\', '/']);
				BINS.iter().find_map(|path| {
					first.strip_prefix(path).map(|cmd| {
						let mut bin = PathBuf::from(bin);
						bin.push(cmd);
						Command::new(bin)
					})
				})
			})
			.unwrap_or_else(|| Command::new(first));

		c.args(words);
		c
	} else {
		// Check if we have an extension interpreter
		let shebang = file
			.extension()
			.and_then(|ext| ext.to_str())
			.and_then(|ext| env::var(format!("SHEBANG.{ext}")).ok())
			.ok_or("no interpreter")?;
		let mut words = shebang.trim().split_ascii_whitespace();
		let first = words.next().ok_or("no interpreter")?;
		let mut c = Command::new(first);
		c.args(words);
		c
	};

	Ok(c.arg(&file).args(args).status()?.code().unwrap_or(0))
}

fn main() {
	match run() {
		Ok(n) => process::exit(n),
		Err(e) => {
			eprintln!("shebang: error: {e}");
			process::exit(-1);
		}
	}
}
