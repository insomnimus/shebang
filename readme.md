# Shebang for Windows
This small program implements a shebang runner and a simple `env` command for Windows.

The intended use is through the `PATHEXT` environment variable coupled with the `cmd.exe` builtins `assoc` and `ftype`.

## Building the Project
You'll need to install the Rust language toolchain.

```powershell
# after cloning
cargo build --release
```

You'll want to put the binaries `shebang.exe` and `env.exe` to the system32 folder but this isn't necessary.

For 64 bit machines an installer script is provided that
- Compiles for both 32 bit and 64 bit targets.
- Puts the 32 bit binaries in `%SYSTEMROOT%/syswow64/` and 64 bit binaries in `%SYSTEMROOT%/system32/`.

## Mapping Linux Bin Folders
You can set the `SHEBANG_BIN` environment variable to a directory and the runner will replace following Linux bin folders with it:
- `/bin/`
- `/usr/bin/`
- `/sbin/`

So a line like `#!/bin/bash`
will be interpreted as `#!<VALUE>/bash` where `<VALUE>` is the value of the `SHEBANG_BIN` environment variable.

## Fallback Interpreters
You can associate an interpreter for an extension by setting the `SHEBANG.<EXT>` environment variable to a space-separated command.

However if a file contains a shebang this value will be ignored and the shebang is used.

## Example Scenario
Say you want to run shell scripts using the [Windows port of Busybox](https://frippery.org/busybox) bash shell.

1. Open an administrator command prompt. (Not Powershell since we need `cmd.exe` builtins.)
2. Create a file type and name it something - we use `shebang` here. Associate it with the path to the `shebang.exe` executable (if it's in your system path, you can omit the full path like we do here):\
	`ftype shebang=shebang.exe %1 "%*"`
3. Associate the `.sh` extension with this filetype using the `ftype` command:\
	`assoc .sh=shebang`
4. Optionally if you want to be able to run `.sh` files on powershell and be able to omit the extension, add it to the `PATHEXT` environment variable (example is in Powershell):\
	`$env:PATHEXT += ";.sh"
5. After installing busybox to some directory, set the `SHEBANG_ENV` environment variable to it. For example (again, on Powershell):\
	`$env:SHEBANG_BIN = "D:\busybox\bin"`

Now you can run a file like below:
```sh
#!/usr/bin/bash


for name in "$@"; do
	echo "Hi, $name!"
done
```

With a command like `./hello.sh foo bar`
