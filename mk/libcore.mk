# This file purpose is to make a Rust LibCore library object for
# further rust code compilation.

# Rust download directory
RUSTDOWNDIR := $(DOWNDIR)rust

# Rule to download the Rust compiler and libraries from the official repo.
$(DOWNDIR)rust:
	@mkdir -p $(DOWNDIR)
	@git -C $(RUSTDOWNDIR) clone https://github.com/rust-lang/rust.git --branch beta

