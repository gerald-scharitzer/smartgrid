# The Smart Grid

This is just to skill smart grids and Rust.

# Develop

1. Install Rust as in https://forge.rust-lang.org/infra/other-installation-methods.html
	1. Download installer `rust-version-platform.format` from https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers
	2. Download signature `rust-version-platform.format.asc`
	3. Verify with `gpg --verify rust-version-platform.format.asc rust-version-platform.format`
	4. Unpack installer
	5. Uninstall old version with `sudo ./install.sh --uninstall`
	6. Install new version with `sudo ./install.sh`
	7. Check with `rustc --version`
2. Clone repository with `git clone https://github.com/gerald-scharitzer/smartgrid.git`
3. Build and run with `cargo run`
