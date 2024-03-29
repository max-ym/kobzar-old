# Rules to download and build Binutils.

# Binutils directory.
BINUTILSDIR ?= $(DOWNDIR)binutils/

# Name of downloaded binutils archive.
BINUTILSARCHIVENAME ?= binutils-2.27.tar.bz2

# Name of dearchivated folder.
BINUTILSDEARCHDIR ?= binutils-2.27/

# Path to download binutils from.
BINUTILSLINK ?= https://ftp.gnu.org/gnu/binutils/$(BINUTILSARCHIVENAME)

# Rule to download and build binutils.
$(BINUTILSDIR):
	@echo 'Binutils archive is being downloaded.'
	@mkdir -p $(BINUTILSDIR)
	@wget -P $(BINUTILSDIR) $(BINUTILSLINK)
	@echo 'Binutils archive is being decompressed...'
	@tar jxf $@$(BINUTILSARCHIVENAME) --directory $(BINUTILSDIR)..
	@mkdir -p $@build/
	@echo 'Binutils is being configured and built.'
	@cd $@../$(BINUTILSDEARCHDIR) && ../$(BINUTILSDEARCHDIR)configure --target=$(TRIPLE) --prefix="$(shell pwd)/$@" --with-sysroot --disable-nls --disable-werror && make && make install
	@rm $@$(BINUTILSARCHIVENAME)
	@rm -r $@../$(BINUTILSDEARCHDIR)
	@rm -r $@build/

# Linker
LD ?= $(BINUTILSDIR)bin/$(TRIPLE)-ld

# If linker is not available, binutils must be uncompiled, so compile.
$(LD): $(BINUTILSDIR)
