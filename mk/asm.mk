# Rules to build assembly.

############################################
########## Rules to download FASM ##########

# Directory where downloaded FASM assembler will be stored.
FASMDOWNDIR ?= $(DOWNDIR)fasm/

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
	@tar zxf $(FASMDOWNDIR)$(FASMARCHIVENAME) --directory $(FASMDOWNDIR)..
	# Delete source archive
	@rm $(FASMDOWNDIR)$(FASMARCHIVENAME)

#####################################################
########## Rules to build the asembly code ##########

# Assembler object file
ASMOBJ ?= $(OBJBDIR)asm.o

# All assembler sources list
# TODO: architecture filter
ASRCLIST := $(shell find $(SRCDIR) -type f -name '*.fasm')

# Command to build all assembly files.
asm: $(ASMOBJ)

$(ASMOBJ): $(ASRCLIST) $(FASM)
	$(FASM) $(ASRCLIST) $@
