# Rules to build an ISO image with OS.

# ISO image location.
ISO ?= $(BUILDDIR)image.iso

# Directory where ISO file structure is composed.
ISOBDIR ?= $(BUILDDIR)iso/

iso: $(ISO)

$(ISO): $(KERNELBIN) $(GRUBCFG)
	@mkdir -p $(ISOBDIR)boot/grub
	@ln -f $(GRUBCFG) $(ISOBDIR)boot/grub/grub.cfg
	@ln -f $(KERNELBIN) $(ISOBDIR)boot/kernel.bin
	# Remove old image if any
	@rm -f $(KERNELISO)
	@grub-mkrescue -o $@ $(ISOBDIR)
	# Clean
	@rm -r $(ISOBDIR)
