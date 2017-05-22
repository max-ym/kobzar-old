# Rules to build Rust kernel sources.

# Kernel object file
KERNOBJ := $(OBJBDIR)kernel.o

krust: $(RSRCLIST) $(KERNOBJ)

$(KERNOBJ): $(RSRCLIST) $(OBJCORE) $(OBJASM-X86_64) $(TARGETSPEC)
	@mkdir -p $(OBJBDIR)
	$(RUSTCF) --out-dir=$(OBJBDIR) -C lto --emit=asm,obj --extern core=$(OBJCORE) --extern asm_x86_64=$(OBJASM-X86_64) $(MAINRS)
