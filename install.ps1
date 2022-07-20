#requires -runAsAdministrator
pushd $psScriptRoot

$targets = @(
	@{triple = "x86_64-pc-windows-msvc"; path = "$env:SYSTEMROOT/system32" }
	@{triple = "i686-pc-windows-msvc"; path = "$env:SYSTEMROOT/syswow64" }
)

cargo build -q --release -Zmultitarget ($targets | % { "--target", $_.triple })

if($lastExitCode -ne 0) {
	write-error failed
	popd
	exit $lastExitCode
}

foreach($x in $targets.getEnumerator()) {
	$dir = "target/{0}/release/" -f $x.triple
	copy-item -force "$dir/shebang.exe", "$dir/env.exe" $x.path
}

popd
