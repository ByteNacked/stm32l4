///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SRAM/NOR-Flash chip-select control register 1
    pub bcr1: BCR1,
    ///0x04 - SRAM/NOR-Flash chip-select timing register 1
    pub btr1: BTR,
    ///0x08 - SRAM/NOR-Flash chip-select control register 2
    pub bcr2: BCR,
    ///0x0c - SRAM/NOR-Flash chip-select timing register 1
    pub btr2: BTR,
    ///0x10 - SRAM/NOR-Flash chip-select control register 2
    pub bcr3: BCR,
    ///0x14 - SRAM/NOR-Flash chip-select timing register 1
    pub btr3: BTR,
    ///0x18 - SRAM/NOR-Flash chip-select control register 2
    pub bcr4: BCR,
    ///0x1c - SRAM/NOR-Flash chip-select timing register 1
    pub btr4: BTR,
    _reserved8: [u8; 96usize],
    ///0x80 - PC Card/NAND Flash control register 3
    pub pcr: PCR,
    ///0x84 - FIFO status and interrupt register 3
    pub sr: SR,
    ///0x88 - Common memory space timing register 3
    pub pmem: PMEM,
    ///0x8c - Attribute memory space timing register 3
    pub patt: PATT,
    _reserved12: [u8; 4usize],
    ///0x94 - ECC result register 3
    pub eccr: ECCR,
    _reserved13: [u8; 108usize],
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    pub bwtr1: BWTR,
    _reserved14: [u8; 4usize],
    ///0x10c - SRAM/NOR-Flash write timing registers 1
    pub bwtr2: BWTR,
    _reserved15: [u8; 4usize],
    ///0x114 - SRAM/NOR-Flash write timing registers 1
    pub bwtr3: BWTR,
    _reserved16: [u8; 4usize],
    ///0x11c - SRAM/NOR-Flash write timing registers 1
    pub bwtr4: BWTR,
}
///SRAM/NOR-Flash chip-select control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr1](bcr1) module
pub type BCR1 = crate::Reg<u32, _BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR1;
///`read()` method returns [bcr1::R](bcr1::R) reader structure
impl crate::Readable for BCR1 {}
///`write(|w| ..)` method takes [bcr1::W](bcr1::W) writer structure
impl crate::Writable for BCR1 {}
///SRAM/NOR-Flash chip-select control register 1
pub mod bcr1;
///SRAM/NOR-Flash chip-select timing register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btr](btr) module
pub type BTR = crate::Reg<u32, _BTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR;
///`read()` method returns [btr::R](btr::R) reader structure
impl crate::Readable for BTR {}
///`write(|w| ..)` method takes [btr::W](btr::W) writer structure
impl crate::Writable for BTR {}
///SRAM/NOR-Flash chip-select timing register 1
pub mod btr;
///SRAM/NOR-Flash chip-select control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr](bcr) module
pub type BCR = crate::Reg<u32, _BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR;
///`read()` method returns [bcr::R](bcr::R) reader structure
impl crate::Readable for BCR {}
///`write(|w| ..)` method takes [bcr::W](bcr::W) writer structure
impl crate::Writable for BCR {}
///SRAM/NOR-Flash chip-select control register 2
pub mod bcr;
///PC Card/NAND Flash control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcr](pcr) module
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
///`read()` method returns [pcr::R](pcr::R) reader structure
impl crate::Readable for PCR {}
///`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure
impl crate::Writable for PCR {}
///PC Card/NAND Flash control register 3
pub mod pcr;
///FIFO status and interrupt register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///`write(|w| ..)` method takes [sr::W](sr::W) writer structure
impl crate::Writable for SR {}
///FIFO status and interrupt register 3
pub mod sr;
///Common memory space timing register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pmem](pmem) module
pub type PMEM = crate::Reg<u32, _PMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMEM;
///`read()` method returns [pmem::R](pmem::R) reader structure
impl crate::Readable for PMEM {}
///`write(|w| ..)` method takes [pmem::W](pmem::W) writer structure
impl crate::Writable for PMEM {}
///Common memory space timing register 3
pub mod pmem;
///Attribute memory space timing register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [patt](patt) module
pub type PATT = crate::Reg<u32, _PATT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PATT;
///`read()` method returns [patt::R](patt::R) reader structure
impl crate::Readable for PATT {}
///`write(|w| ..)` method takes [patt::W](patt::W) writer structure
impl crate::Writable for PATT {}
///Attribute memory space timing register 3
pub mod patt;
///ECC result register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eccr](eccr) module
pub type ECCR = crate::Reg<u32, _ECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR;
///`read()` method returns [eccr::R](eccr::R) reader structure
impl crate::Readable for ECCR {}
///ECC result register 3
pub mod eccr;
///SRAM/NOR-Flash write timing registers 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bwtr](bwtr) module
pub type BWTR = crate::Reg<u32, _BWTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR;
///`read()` method returns [bwtr::R](bwtr::R) reader structure
impl crate::Readable for BWTR {}
///`write(|w| ..)` method takes [bwtr::W](bwtr::W) writer structure
impl crate::Writable for BWTR {}
///SRAM/NOR-Flash write timing registers 1
pub mod bwtr;
