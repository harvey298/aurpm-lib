C-Lib:
	cargo build --release
	cargo install --force cbindgen
	$(HOME)/.cargo/bin/cbindgen --config cbindgen.toml --crate aurpm_lib --output target/aurpm_lib.h