# Source code directory
SRCDIR ?= ./src/

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

# Rust compiler, flags and combination.
RUSTC ?= rustc
RUSTF ?= -O --cfg arch__$(ARCH) --target=$(TARGETSPEC)
RUSTCF := $(RUSTC)
RUSTCF += $(RUSTF)

# Build directory
BUILDDIR ?= ./build/

# Object directory
OBJDIR := $(BUILDDIR)obj/

# ISO root directory
ISODIR := $(BUILDDIR)iso/

# Rust libraries directory
RUSTLIBDIR := ./rustlibs/

# All rust sources list
RSRCLIST := $(shell find . -type f -name '*.rs')

# Configuration scripts
CONFIGDIR  ?= ./config/$(ARCH)/
GRUBCFG    := $(CONFIGDIR)grub.cfg
LINKSCRIPT := $(CONFIGDIR)link.ld
TARGETSPEC := $(CONFIGDIR)target.json
