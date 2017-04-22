// build.rs

use std::process::Command;
use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::File;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    if !Command::new("armv4t-none-eabi-as").args(&["src/init.s", "-o"])
                       .arg(&format!("{}/init.o", out_dir))
        .status().unwrap().success() {
        panic!("Failed assembling init");
    }

    if !Command::new("armv4t-none-eabi-ar")
        .args(&["crus", "libgbainit.a", "init.o"])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap().success() {
        panic!("Failed linking gbainit");
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=gbainit");

    // Create a linker script needed by the game boy advance platform.
    let ld = String::from("
OUTPUT_FORMAT(\"elf32-littlearm\", \"elf32-bigarm\", \"elf32-littlearm\")
OUTPUT_ARCH(arm)
ENTRY(_header)

MEMORY
{
    EWRAM : ORIGIN = 0x02000000, LENGTH = 256K
    IWRAM : ORIGIN = 0x03000000, LENGTH = 32512
    CART : ORIGIN = 0x08000000, LENGTH = 32768K
}

/* fixed addresses */
__init_start        = ORIGIN(CART) + 0xC0;

/* heap */
__eheap_end			= ORIGIN(EWRAM) + LENGTH(EWRAM);

/* stack */
__sp_usr			= ORIGIN(IWRAM) + LENGTH(IWRAM) - 0x100;
__sp_irq			= ORIGIN(IWRAM) + LENGTH(IWRAM) - 0x60;
__intr_vector_buf	= ORIGIN(IWRAM) + LENGTH(IWRAM) - 0x4;

/* sections */
SECTIONS
{
    .header ORIGIN(CART) : AT(ORIGIN(CART))
    {
        KEEP(*(.header))
    } > CART

    .init __init_start :
    {
        KEEP(*(.init))
    } > CART

	/* text - code */
	.text : ALIGN(4)
	{
		*(.text.*)
		. = ALIGN(4);
	} > CART

	.rodata :
	{
		*(.rodata)
		*all.rodata*(*)
		*(.roda)
		*(.rodata.*)
		. = ALIGN(4);
	} > CART

	/* stuff always in internal work RAM */
	.iwram ORIGIN(IWRAM) :
	{
    	__iwram_start = ABSOLUTE(.) ;
    	*(.iwram)
		*iwram.*(.text)
		. = ALIGN(32);
        __iwram_end = ABSOLUTE(.);
	} > IWRAM AT > CART
    __iwram_lma = LOADADDR(.iwram);

    /* zero-initialized data in iwram */
    .iwrambss (NOLOAD): {
        __iwrambss_start = ABSOLUTE(.);
        *(.iwrambss)
        . = ALIGN(4);
        __iwrambss_end = .;
    } > IWRAM

	/* ewram - stuff always loaded in external work RAM */
	.ewram ORIGIN(EWRAM) :
	{
		__ewram_start = ABSOLUTE(.);
		*(.ewram)
		. = ALIGN(32);
        __ewram_end = .;
	} > EWRAM AT > CART
   	__ewram_lma = LOADADDR(.ewram);

	/* data - initialized global/static variables - in ewram*/
	.data ALIGN(4) :
	{
		__data_start = ABSOLUTE(.);
		*(.data)
		*(.data.*)
		*(.gnu.linkonce.d*)
		. = ALIGN(32);
        __data_end = .;
	} > EWRAM AT > CART
    __data_lma = LOADADDR(.data);

	/* bss - zero initialized global variables - in ewram*/
	.bss ALIGN(4) (NOLOAD):
	{
		__bss_start = ABSOLUTE(.);
        *(.bss.*)
		*(COMMON)
		. = ALIGN(32);
	} > EWRAM
	__bss_end = .;

	__eheap_start = .;

    /DISCARD/ :
    {
        *(.ARM.exidx.*)
        *(.ARM.extab.*)
        *(.note.gnu.build-id.*)
    }

	/*
		DWARF debug sections.
		Symbols in the DWARF debugging sections are relative to the beginning of the section so we begin them at 0.
	*/

	/* DWARF 1 */
	.debug				0 : { *(.debug) }
	.line				0 : { *(.line) }

	/* GNU DWARF 1 extensions */
	.debug_srcinfo		0 : { *(.debug_srcinfo) }
	.debug_sfnames		0 : { *(.debug_sfnames) }

	/* DWARF 1.1 and DWARF 2 */
	.debug_aranges		0 : { *(.debug_aranges) }
	.debug_pubnames		0 : { *(.debug_pubnames) }

	/* DWARF 2 */
	.debug_info			0 : { *(.debug_info) }
	.debug_abbrev		0 : { *(.debug_abbrev) }
	.debug_line			0 : { *(.debug_line) }
	.debug_frame		0 : { *(.debug_frame) }
	.debug_str			0 : { *(.debug_str) }
	.debug_loc			0 : { *(.debug_loc) }
	.debug_macinfo		0 : { *(.debug_macinfo) }

	/* SGI/MIPS DWARF 2 extensions */
	.debug_weaknames	0 : { *(.debug_weaknames) }
	.debug_funcnames	0 : { *(.debug_funcnames) }
	.debug_typenames	0 : { *(.debug_typenames) }
	.debug_varnames		0 : { *(.debug_varnames) }
}

/* ensure at least a 1K stack */
ASSERT((__iwram_end < __sp_usr - 0x400), \"Need at least 1k of stack\")
    ");

    File::create(Path::new(&out_dir).join("gba.ld"))
        .unwrap()
        .write_all(ld.as_bytes())
        .unwrap();

    println!("cargo:rustc-link-search={}", out_dir);
}