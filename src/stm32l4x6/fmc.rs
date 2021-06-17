///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SRAM/NOR-Flash chip-select control register 1
    pub bcr1: BCR1,
    ///0x04 - SRAM/NOR-Flash chip-select timing register 1
    pub btr1: BTR1,
    ///0x08 - SRAM/NOR-Flash chip-select control register 2
    pub bcr2: BCR2,
    ///0x0c - SRAM/NOR-Flash chip-select timing register 2
    pub btr2: BTR2,
    ///0x10 - SRAM/NOR-Flash chip-select control register 3
    pub bcr3: BCR3,
    ///0x14 - SRAM/NOR-Flash chip-select timing register 3
    pub btr3: BTR3,
    ///0x18 - SRAM/NOR-Flash chip-select control register 4
    pub bcr4: BCR4,
    ///0x1c - SRAM/NOR-Flash chip-select timing register 4
    pub btr4: BTR4,
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
    pub bwtr1: BWTR1,
    _reserved14: [u8; 4usize],
    ///0x10c - SRAM/NOR-Flash write timing registers 2
    pub bwtr2: BWTR2,
    _reserved15: [u8; 4usize],
    ///0x114 - SRAM/NOR-Flash write timing registers 3
    pub bwtr3: BWTR3,
    _reserved16: [u8; 4usize],
    ///0x11c - SRAM/NOR-Flash write timing registers 4
    pub bwtr4: BWTR4,
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
///For information about available fields see [btr1](btr1) module
pub type BTR1 = crate::Reg<u32, _BTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR1;
///`read()` method returns [btr1::R](btr1::R) reader structure
impl crate::Readable for BTR1 {}
///`write(|w| ..)` method takes [btr1::W](btr1::W) writer structure
impl crate::Writable for BTR1 {}
///SRAM/NOR-Flash chip-select timing register 1
pub mod btr1;
///SRAM/NOR-Flash chip-select control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr2](bcr2) module
pub type BCR2 = crate::Reg<u32, _BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR2;
///`read()` method returns [bcr2::R](bcr2::R) reader structure
impl crate::Readable for BCR2 {}
///`write(|w| ..)` method takes [bcr2::W](bcr2::W) writer structure
impl crate::Writable for BCR2 {}
///SRAM/NOR-Flash chip-select control register 2
pub mod bcr2;
///SRAM/NOR-Flash chip-select timing register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btr2](btr2) module
pub type BTR2 = crate::Reg<u32, _BTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR2;
///`read()` method returns [btr2::R](btr2::R) reader structure
impl crate::Readable for BTR2 {}
///`write(|w| ..)` method takes [btr2::W](btr2::W) writer structure
impl crate::Writable for BTR2 {}
///SRAM/NOR-Flash chip-select timing register 2
pub mod btr2;
///SRAM/NOR-Flash chip-select control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr3](bcr3) module
pub type BCR3 = crate::Reg<u32, _BCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR3;
///`read()` method returns [bcr3::R](bcr3::R) reader structure
impl crate::Readable for BCR3 {}
///`write(|w| ..)` method takes [bcr3::W](bcr3::W) writer structure
impl crate::Writable for BCR3 {}
///SRAM/NOR-Flash chip-select control register 3
pub mod bcr3;
///SRAM/NOR-Flash chip-select timing register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btr3](btr3) module
pub type BTR3 = crate::Reg<u32, _BTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR3;
///`read()` method returns [btr3::R](btr3::R) reader structure
impl crate::Readable for BTR3 {}
///`write(|w| ..)` method takes [btr3::W](btr3::W) writer structure
impl crate::Writable for BTR3 {}
///SRAM/NOR-Flash chip-select timing register 3
pub mod btr3;
///SRAM/NOR-Flash chip-select control register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr4](bcr4) module
pub type BCR4 = crate::Reg<u32, _BCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR4;
///`read()` method returns [bcr4::R](bcr4::R) reader structure
impl crate::Readable for BCR4 {}
///`write(|w| ..)` method takes [bcr4::W](bcr4::W) writer structure
impl crate::Writable for BCR4 {}
///SRAM/NOR-Flash chip-select control register 4
pub mod bcr4;
///SRAM/NOR-Flash chip-select timing register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btr4](btr4) module
pub type BTR4 = crate::Reg<u32, _BTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTR4;
///`read()` method returns [btr4::R](btr4::R) reader structure
impl crate::Readable for BTR4 {}
///`write(|w| ..)` method takes [btr4::W](btr4::W) writer structure
impl crate::Writable for BTR4 {}
///SRAM/NOR-Flash chip-select timing register 4
pub mod btr4;
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
///For information about available fields see [bwtr1](bwtr1) module
pub type BWTR1 = crate::Reg<u32, _BWTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR1;
///`read()` method returns [bwtr1::R](bwtr1::R) reader structure
impl crate::Readable for BWTR1 {}
///`write(|w| ..)` method takes [bwtr1::W](bwtr1::W) writer structure
impl crate::Writable for BWTR1 {}
///SRAM/NOR-Flash write timing registers 1
pub mod bwtr1;
///SRAM/NOR-Flash write timing registers 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bwtr2](bwtr2) module
pub type BWTR2 = crate::Reg<u32, _BWTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR2;
///`read()` method returns [bwtr2::R](bwtr2::R) reader structure
impl crate::Readable for BWTR2 {}
///`write(|w| ..)` method takes [bwtr2::W](bwtr2::W) writer structure
impl crate::Writable for BWTR2 {}
///SRAM/NOR-Flash write timing registers 2
pub mod bwtr2;
///SRAM/NOR-Flash write timing registers 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bwtr3](bwtr3) module
pub type BWTR3 = crate::Reg<u32, _BWTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR3;
///`read()` method returns [bwtr3::R](bwtr3::R) reader structure
impl crate::Readable for BWTR3 {}
///`write(|w| ..)` method takes [bwtr3::W](bwtr3::W) writer structure
impl crate::Writable for BWTR3 {}
///SRAM/NOR-Flash write timing registers 3
pub mod bwtr3;
///SRAM/NOR-Flash write timing registers 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bwtr4](bwtr4) module
pub type BWTR4 = crate::Reg<u32, _BWTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BWTR4;
///`read()` method returns [bwtr4::R](bwtr4::R) reader structure
impl crate::Readable for BWTR4 {}
///`write(|w| ..)` method takes [bwtr4::W](bwtr4::W) writer structure
impl crate::Writable for BWTR4 {}
///SRAM/NOR-Flash write timing registers 4
pub mod bwtr4;
