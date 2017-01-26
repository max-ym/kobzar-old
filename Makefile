# Source code directory
SRCDIR ?= ./src/

# Main rust file to start kernel compilation from
MAINRS ?= $(SRCDIR)main.rs

##########
# Operating system architecture.
# Change if needed. Currently only x86_64 is valid.
ARCH = x86_64

ifeq ($(ARCH), x86_64)
	TRIPLE ?= x86_64-elf-
else
	$(error Unsupported architecture: $(ARCH))
endif
##########

# Build directory
BUILDDIR ?= ./build/$(ARCH)/

# Downloads directory. Any related packages from internet are stored there.
DOWNDIR ?= ./build/downloads/

# Object build directory
OBJBDIR := $(BUILDDIR)obj/

# ISO root directory
ISODIR := $(BUILDDIR)iso/

# Rust libraries directory
RUSTLIBDIR := ./rustlibs/

# All rust sources list
RSRCLIST := $(shell find $(SRCDIR) -type f -name '*.rs')

# Configuration scripts
CONFIGDIR  ?= ./config/$(ARCH)/
GRUBCFG    := $(CONFIGDIR)grub.cfg
LINKSCRIPT := $(CONFIGDIR)link.ld
TARGETSPEC := $(CONFIGDIR)target.json

# Rust compiler, flags and combination.
RUSTC ?= rustc
RUSTF ?= -O --cfg arch__$(ARCH) --target=$(TARGETSPEC)
RUSTCF := $(RUSTC)
RUSTCF += $(RUSTF)

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
