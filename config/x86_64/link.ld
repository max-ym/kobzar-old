ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS
{
    /* Begin putting sections at 1 MiB, a conventional place for kernels to be
       loaded at by the bootloader. */
    . = 1M;


    .init BLOCK(8) : ALIGN(8) {
        KEEP( *(.multiboot) )
    }

    /* First put the multiboot header, as it is required to be put very
       early in the image or the bootloader won't recognize the file
       format. */
    .text BLOCK(16) : ALIGN(16)
    {
        *(.text)
    }

    /* Read-only data. */
    .rodata BLOCK(8) : ALIGN(8)
    {
        *(.rodata)
    }

    /* Read-write data (initialized) */
    .data BLOCK(8) : ALIGN(8)
    {
        *(.data)
    }

    /* Read-write data (uninitialized) */
    .bss BLOCK(8) : ALIGN(8)
    {
        *(COMMON)
        *(.bss)
    }
}
