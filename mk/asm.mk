# Rules to build assembly.

# Directory where downloaded FASM assembler will be stored.
FASMDOWNDIR ?= $(BUILDDIR)fasm/

# FASM assembler command
FASM ?= $(FASMDOWNDIR)fasm

FASMARCHIVENAME ?= fasm-1.71.59.tgz
FASMDOWNLOADLINK ?= https://flatassembler.net/$(FASMARCHIVENAME)

# To get FASM assembler, script must download it from official website.
$(FASM): $(FASMDOWNDIR)

$(FASMDOWNDIR):
	@mkdir -p $(FASMDOWNDIR)
	@echo '$(FASMARCHIVENAME) assembler archive is being downloaded.'
	# Download the archive
	@wget -P $@ $(FASMDOWNLOADLINK)
	# Unzip content
	@tar --directory $(FASMDOWNDIR) zxvf $(FASMARCHIVENAME)
	# Delete source archive
	@rm $(FASMDOWNDIR)$(FASMARCHIVENAME)
