ENTRY(_start)

SECTIONS
{
  . = 0x80200000;

  .text : ALIGN(4K)
  {
    *(.text.entry)
    *(.text*)
  }
  .rodata : ALIGN(4K)
  {
    *(.rodata*)
  }
  .data : ALIGN(4K)
  {
    _data_start = .;
    *(.data*)
    _data_end = .;
  }
  .bss : ALIGN(4K)
  {
    _bss_start = .;
    *(.bss*)
    *(COMMON)
    _bss_end = .;
  }
  .stack (NOLOAD) : ALIGN(4K)
  {
    _stack_bottom = .;
    . += 0x100000;
    _stack_top = .;
  }
  . = ALIGN(4K);
  _kernel_end = .;
}