# The Smart Grid

This is just to skill smart grids and Rust.

# Develop

this packge as follows.

1. Install Rust with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Check Rust with `rustc -V`
3. Install `wasm-pack` with `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
4. Check `wasm-pack` with `wasm-pack -V`
5. Clone repository with `git clone https://github.com/gerald-scharitzer/smartgrid.git`
6. Build and run with `cargo run`

# Issues

Usually I install Rust manually as in the sections below, such that I can verify the signature of the installer first, but then installing `wasm-pack` with `cargo install wasm-pack` fails while compiling `openssl` with file not found and `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh` fails with rustup not found. Therefore I fell back to rustup. I did not try to install `wasm-pack` via `npm` or `yarn`.

## Install Rust

as in https://forge.rust-lang.org/infra/other-installation-methods.html.

1. Download installer `rust-version-platform.format` from https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers
2. Download signature `rust-version-platform.format.asc`
3. Verify with `gpg --verify rust-version-platform.format.asc rust-version-platform.format`
4. Unpack installer
5. Uninstall old version with `sudo ./install.sh --uninstall`
6. Install new version with `sudo ./install.sh`
7. Check with `rustc -V`

where `version` is the new Rust version `major.minor.patch` and `platform` is the compile target.

## Install Web Assembly

as in https://rustwasm.github.io/docs/wasm-pack/prerequisites/non-rustup-setups.html.

1. Download installer from https://static.rust-lang.org/dist/rust-std-version-wasm32-unknown-unknown.tar.gz
2. Download signature from https://static.rust-lang.org/dist/rust-std-version-wasm32-unknown-unknown.tar.gz.asc
3. Verify with `gpg --verify rust-std-version-wasm32-unknown-unknown.tar.gz.asc rust-std-version-wasm32-unknown-unknown.tar.gz`
4. Unpack installer
5. Get Rust sysroot with `rustc --print sysroot`
6. Copy the target `wasm32` to Rust with `sudo cp -r wasm32-unknown-unknown sysroot/lib/rustlib`
7. Install `wasm-pack` with `cargo install wasm-pack`
8. Check with `wasm-pack -V`

where `version` is your installed Rust version `major.minor.patch` and `sysroot` is the Rust sysroot.
