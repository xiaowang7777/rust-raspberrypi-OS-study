
_rpi_phy_binary_load_addr = 0x80000;

ENTRY(_rpi_phy_binary_load_addr)

PHDRS
{
    segment_code PT_LOAD FLAGS(5);
}

SECTIONS
{
    . = _rpi_phy_binary_load_addr;
    .text :
        {
            KEEP(*(.text._start))
        } :segment_code
}