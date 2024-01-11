ENTRY(module_start)
SECTIONS
{
  /* PRX format requires text to begin at 0 */
  .text 0 : { KEEP(*(.text .text.*)) }
  PROVIDE(__text = ADDR(.text));

  /* Sort stubs for convenient ordering */
  .sceStub.text : { *(.sceStub.text) *(SORT(.sceStub.text.*)) }

  /* Keep these sections around, even though they may appear unused to the linker */
  .lib.ent.top :  { KEEP(*(.lib.ent.top)) }
  .lib.ent :      { KEEP(*(.lib.ent)) }
  .lib.ent.btm :  { KEEP(*(.lib.ent.btm)) }
  .lib.stub.top : { KEEP(*(.lib.stub.top)) }
  .lib.stub :     { KEEP(*(.lib.stub)) }
  .lib.stub.btm : { KEEP(*(.lib.stub.btm)) }

  /* Exception support */
  .eh_frame_hdr : { KEEP(*(.eh_frame_hdr .eh_frame_hdr.*)) }
  PROVIDE(__eh_frame_hdr = ADDR(.eh_frame_hdr));
  .eh_frame : { KEEP(*(.eh_frame .eh_frame.*)) }
  PROVIDE(__eh_frame = ADDR(.eh_frame));

  /* These are explicitly listed to avoid being merged into .rodata */
  .rodata.sceResident : { *(.rodata.sceResident) *(.rodata.sceResident.*) }
  .rodata.sceModuleInfo : { *(.rodata.sceModuleInfo) }
  /* Sort NIDs for convenient ordering */
  .rodata.sceNid : { *(.rodata.sceNid) *(SORT(.rodata.sceNid.*)) }

  .rodata : { *(.rodata .rodata.*) }
  .data : { *(.data .data.*) }
  .gcc_except_table : { *(.gcc_except_table .gcc_except_table.*) }
  .bss : { *(.bss .bss.*) }

  /DISCARD/ : { *(.rel.sceStub.text .MIPS.abiflags .reginfo) }
}