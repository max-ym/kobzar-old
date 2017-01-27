# Rules to build binary executable file with kernel.

# All object files list.
OBJS := $(ASMOBJ) $(KERNOBJ)

# Kernel executable binary file.
KERNELBIN := $(BUILDDIR)kernel.bin

bin: $(KERNELBIN)

# Rule to build a binary executable kernel file.
$(KERNELBIN): $(LD) $(OBJS) $(LINKSCRIPT)
	$(LD) -o $@ $(LDFLAGS) $(OBJS)
