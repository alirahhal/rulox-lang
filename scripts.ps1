param (
	[ValidateSet("build", "release", "test", "doc", "doc-open", "format", "lint")]
	[string]$Script = ""
)

if ($Script -eq "") {
	$Script = "build"
}

if ($Script -eq "build") {
	cargo build
}

if ($Script -eq "release") {
	cargo build --release
}

if ($Script -eq "test") {
	cargo test
}

if ($Script -eq "doc") {
	cargo doc
}

if ($Script -eq "doc-open") {
	cargo doc --open
}

if ($Script -eq "format") {
	cargo fmt
}

if ($Script -eq "lint") {
	cargo clippy --fix
}
