///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    ///0x04 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    pub cha: CH,
    ///0x24 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    pub chb: CH,
}
///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - AConfiguration register 1
    pub cr1: self::ch::CR1,
    ///0x04 - AConfiguration register 2
    pub cr2: self::ch::CR2,
    ///0x08 - AFRCR
    pub frcr: self::ch::FRCR,
    ///0x0c - ASlot register
    pub slotr: self::ch::SLOTR,
    ///0x10 - AInterrupt mask register2
    pub im: self::ch::IM,
    ///0x14 - AStatus register
    pub sr: self::ch::SR,
    ///0x18 - AClear flag register
    pub clrfr: self::ch::CLRFR,
    ///0x1c - AData register
    pub dr: self::ch::DR,
}
///Register block
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub mod ch;
