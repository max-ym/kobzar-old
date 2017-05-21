# Rules to download and build asm-x86_64 library for kernel.

ASM-x86_64-GITREPO := https://github.com/max-ym/rust-asm-x86_64.git

OBJASM-x86_64-DOWNDIR := $(DOWNDIR)asm-x86_64/

OBJASM-x86_64 := $(OBJBDIR)libasm-x86_64.rlib

# Rule to download asm-x86_64 library. This library is used in kernel
# source code.
$(OBJASM-x86_64-DOWNDIR):
	@echo 'New asm-x86_64 library will be downloaded from github.'
	@mkdir -p $(DOWNDIR)
	@git -C $(DOWNDIR) clone $(ASM-x86_64-GITREPO) --branch master

$(OBJASM-x86_64): $(OBJASM-x86_64-DOWNDIR)
	$(RUSTCF) --crate-type rlib --crate-name asm_x86_64 $(OBJASM-x86_64-DOWNDIR)src/lib.rs

asm-x86_64-rm:
	@rm -rf $(OBJASM-x86_64-DOWNDIR)

asm-x86_64: $(OBJASM-x86_64)
