# Variables that are used in other rules. Configure building environment.
include mk/config.mk

# Rules to build rust kernel code HTML docs.
include mk/kerndocs.mk

# Rules to download and build binutils.
include mk/binutils.mk

# Rules to get Rust Compiler repo. Used to build compiler core libraries.
include mk/rustrepo.mk

# Rules to make libcore (used by rust compiler for kernel sources).
include mk/libcore.mk

# Rules to make 'new_bitflags' crate.
include mk/libnew_bitflags.mk

# Rules to build asm-x86_64 library for kernel.
include mk/libasm-x86_64.mk

# Rules to build Rust kernel sources.
include mk/kernobj.mk

# Rules to build assembly.
include mk/asm.mk

# Rules to build binary executable file with kernel.
include mk/bin.mk

# Rules to build an ISO image with OS.
include mk/iso.mk
