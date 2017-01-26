include mk/config.mk

# Kernel object file
KERNOBJ := $(OBJBDIR)kernel.o

# Rules to get Rust Compiler repo. Used to build compiler core libraries.
include mk/rustrepo.mk

# Rules to make libcore (used by rust compiler for kernel sources).
include mk/libcore.mk

##########
# Build only kernel rust code.
krust: $(RSRCLIST) $(KERNOBJ)

$(KERNOBJ): $(RSRCLIST) $(OBJCORE) $(TARGETSPEC)
	@mkdir -p $(OBJBDIR)
	$(RUSTCF) --out-dir=$(OBJBDIR) -C lto --emit=asm,obj --extern core=$(OBJCORE) $(MAINRS)
