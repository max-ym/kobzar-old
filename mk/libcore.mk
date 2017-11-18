# This file purpose is to make a Rust LibCore library object for
# further rust code compilation.

# Rust Core library object
OBJCORE ?= $(OBJBDIR)libcore.rlib

# Rule to build libcore object file
$(OBJCORE): $(RUSTDOWNDIR)src/libcore/lib.rs $(TARGETSPEC)
	@mkdir -p $(dir $@)
	$(RUSTCF) --crate-name core -C panic=abort --out-dir=$(OBJBDIR) --crate-type=lib --emit=link,dep-info $<

# Rule to get lib.rs if it is not available. This means that rust was not
# downloaded and so this rule downloads rust.
$(RUSTDOWNDIR)src/libcore/lib.rs: $(RUSTDOWNDIR)
