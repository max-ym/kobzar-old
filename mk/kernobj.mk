# Rules to build Rust kernel sources.

# Kernel object file
KERNOBJ := $(OBJBDIR)kernel.o

krust: $(RSRCLIST) $(KERNOBJ)

$(KERNOBJ): $(RSRCLIST) $(OBJCORE) $(TARGETSPEC)
	@mkdir -p $(OBJBDIR)
	$(RUSTCF) --out-dir=$(OBJBDIR) -C lto --emit=asm,obj --extern core=$(OBJCORE) $(MAINRS)
