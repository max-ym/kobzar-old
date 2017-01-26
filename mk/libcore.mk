# This file purpose is to make a Rust LibCore library object for
# further rust code compilation.

# Rust download directory
RUSTDOWNDIR := $(DOWNDIR)rust/

# Rule to download the Rust compiler and libraries from the official repo.
$(RUSTDOWNDIR):
	@echo 'New Rust Compiler sources will be downloaded from github.'
	@mkdir -p $(DOWNDIR)
	@git -C $(DOWNDIR) clone https://github.com/rust-lang/rust.git --branch beta

# Rust Core library object
OBJCORE ?= $(OBJBDIR)libcore.o

# Rule to build libcore object file
$(OBJCORE): $(RUSTDOWNDIR)src/libcore/lib.rs $(TARGETSPEC)
	@mkdir -p $(dir $@)
	$(RUSTCF) -C panic=abort --out-dir=$(OBJDIR) --crate-type=lib --emit=link,dep-info $<

# Rule to get lib.rs if it is not available. This means that rust was not
# downloaded and so this rule downloads rust.
$(RUSTDOWNDIR)src/libcore/lib.rs: $(RUSTDOWNDIR)
