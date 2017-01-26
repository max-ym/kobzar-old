include mk/config.mk

# Kernel object file
KERNOBJ := $(OBJBDIR)kernel.o

# Rules to get Rust Compiler repo. Used to build compiler core libraries.
include mk/rustrepo.mk

# Rules to make libcore (used by rust compiler for kernel sources).
include mk/libcore.mk

# Rules to build Rust kernel sources.
include mk/kernobj.mk
