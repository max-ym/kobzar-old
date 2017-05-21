# Rules to download and build asm-x86_64 library for kernel.

# TODO fix naming conventions

# Git repository name
ASM-X86_64-GITNAME := rust-asm-x86_64

# Destination directory name
ASM-X86_64-NAME := asm-x86_64

ASM-X86_64-GITREPO := https://github.com/max-ym/$(ASM-X86_64-GITNAME).git

OBJASM-X86_64-DOWNDIR := $(DOWNDIR)$(ASM-X86_64-NAME)/

OBJASM-X86_64 := $(OBJBDIR)libasm_x86_64.rlib

# Rule to download asm-x86_64 library. This library is used in kernel
# source code.
$(OBJASM-X86_64-DOWNDIR):
	@echo 'New asm-x86_64 library will be downloaded from github.'
	@mkdir -p $(DOWNDIR)
	@git -C $(DOWNDIR) clone $(ASM-X86_64-GITREPO) --branch master
	@mv $(DOWNDIR)$(ASM-X86_64-GITNAME) $(OBJASM-X86_64-DOWNDIR)

$(OBJASM-X86_64): $(OBJASM-X86_64-DOWNDIR) $(TARGETSPEC)
	$(RUSTCF) --out-dir=$(OBJBDIR) --emit=link,dep-info --extern core=$(OBJCORE) $(OBJASM-X86_64-DOWNDIR)src/lib.rs
