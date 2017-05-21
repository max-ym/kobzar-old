# Rules to build binary executable file with kernel.

# All object files list.
OBJS := $(ASMOBJ) $(KERNOBJ) $(OBJASM-X86_64)

# Kernel executable binary file.
KERNELBIN := $(BUILDDIR)kernel.bin

LDFLAGS := -T $(LINKSCRIPT)
LDFLAGS += --gc-sections
LDFLAGS += -z max-page-size=0x1000

bin: $(KERNELBIN)

# Rule to build a binary executable kernel file.
$(KERNELBIN): $(LD) $(OBJS) $(LINKSCRIPT)
	$(LD) -o $@ $(LDFLAGS) $(OBJS)
