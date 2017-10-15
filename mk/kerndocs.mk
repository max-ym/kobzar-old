docs: $(RSRCLIST)
	@rustdoc --output $(DOCSDIR) --cfg arch__$(ARCH)  --target $(TARGETSPEC) -L $(OBJBDIR) --no-defaults --passes "collapse-docs" --passes "unindent-comments" $(RURSTMAIN)
