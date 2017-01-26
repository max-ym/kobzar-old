# Scipt to download repository of Rust Compiler and it's libraries.

# Rust download directory
RUSTDOWNDIR := $(DOWNDIR)rust/

# Rust GIT repository sources can be downloaded from.
RUSTGITREPO := https://github.com/rust-lang/rust.git

# Rule to download the Rust compiler and libraries from the official repo.
# Some libraries (like libcore) are always needed to build kernel Rust sources
$(RUSTDOWNDIR):
	@echo 'New Rust Compiler sources will be downloaded from github.'
	@echo 'Make sure your Rust compiler is as new as the newest beta or else errors while building some libraries may be found.'
	@echo 'If library fail to compile because of your old compiler version, you can build it from the sources being downloaded now. You can find them in: $(RUSTDOWNDIR)'
	@echo 'For more information about building the compiler visit: $(RUSTGITREPO)'
	@mkdir -p $(DOWNDIR)
	@git -C $(DOWNDIR) clone $(RUSTGITREPO) --branch beta
