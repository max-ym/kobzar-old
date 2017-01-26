# Rules to build Rust kernel sources.

krust: $(RSRCLIST) $(KERNOBJ)

$(KERNOBJ): $(RSRCLIST) $(OBJCORE) $(TARGETSPEC)
	@mkdir -p $(OBJBDIR)
	$(RUSTCF) --out-dir=$(OBJBDIR) -C lto --emit=asm,obj --extern core=$(OBJCORE) $(MAINRS)
