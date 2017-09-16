# Rules to make 'new_bitflags' crate.

NEW_BITFLAGS-NAME := new_bitflags

NEW_BITFLAGS-GITNAME := new_bitflags

NEW_BITFLAGS-GITREPO := https://github.com/murarth/$(NEW_BITFLAGS-GITNAME).git

OBJNEW_BITFLAGS-DOWNDIR := $(DOWNDIR)$(NEW_BITFLAGS-NAME)/

OBJNEW_BITFLAGS := $(OBJBDIR)libnew_bitflags.rlib

$(OBJNEW_BITFLAGS-DOWNDIR):
	@echo 'new_bitflags library will be downloaded from github.'
	@mkdir -p $(DOWNDIR)
	@git -C $(DOWNDIR) clone $(NEW_BITFLAGS-GITREPO) --branch master

$(OBJNEW_BITFLAGS): $(OBJNEW_BITFLAGS-DOWNDIR)
	$(RUSTCF) --crate-name $(NEW_BITFLAGS-NAME) --crate-type=rlib --out-dir=$(OBJBDIR) $(OBJNEW_BITFLAGS-DOWNDIR)src/lib.rs --extern std=$(OBJCORE)
