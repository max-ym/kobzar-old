# Rules to build binary executable file with kernel.

# Binutils directory.
BINUTILSDIR ?= $(DOWNDIR)binutils/

# Name of downloaded binutils archive.
BINUTILSARCHIVENAME ?= binutils-2.27.tar.bz2

# Name of dearchivated folder.
BINUTILSDEARCHDIR ?= binutils-2.27

# Path to download binutils from.
BINUTILSLINK ?= https://ftp.gnu.org/gnu/binutils/$(BINUTILSARCHIVENAME)

# Rule to download and build binutils.
$(BINUTILSDIR):
	@echo 'Binutils archive is being downloaded.'
	@mkdir -p $@
	@wget -P $@ $(BINUTILSLINK)
	@tar zxf $@$(BINUTILSARCHIVENAME) --directory $@..
	@rm $@$(BINUTILSARCHIVENAME)
	@mkdir $@build/
	@cd $@build/ && ../$(BINUTILSDEARCHDIR)/configure --target=$(TRIPLE) --prefix="$@" --with-sysroot --disable-nls --disable-werror && make && make install
	@rm -r $@build/
	@rm -r $@$(BINUTILSDEARCHDIR)
