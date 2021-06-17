///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Power control register 1
    pub cr1: CR1,
    ///0x04 - Power control register 2
    pub cr2: CR2,
    ///0x08 - Power control register 3
    pub cr3: CR3,
    ///0x0c - Power control register 4
    pub cr4: CR4,
    ///0x10 - Power status register 1
    pub sr1: SR1,
    ///0x14 - Power status register 2
    pub sr2: SR2,
    ///0x18 - Power status clear register
    pub scr: SCR,
    _reserved7: [u8; 4usize],
    ///0x20 - Power Port A pull-up control register
    pub pucra: PUCRA,
    ///0x24 - Power Port A pull-down control register
    pub pdcra: PDCRA,
    ///0x28 - Power Port B pull-up control register
    pub pucrb: PUCRB,
    ///0x2c - Power Port B pull-down control register
    pub pdcrb: PDCRB,
    ///0x30 - Power Port C pull-up control register
    pub pucrc: PUCRC,
    ///0x34 - Power Port C pull-down control register
    pub pdcrc: PDCRC,
    ///0x38 - Power Port D pull-up control register
    pub pucrd: PUCRD,
    ///0x3c - Power Port D pull-down control register
    pub pdcrd: PDCRD,
    ///0x40 - Power Port E pull-up control register
    pub pucre: PUCRE,
    ///0x44 - Power Port E pull-down control register
    pub pdcre: PDCRE,
    ///0x48 - Power Port F pull-up control register
    pub pucrf: PUCRF,
    ///0x4c - Power Port F pull-down control register
    pub pdcrf: PDCRF,
    ///0x50 - Power Port G pull-up control register
    pub pucrg: PUCRG,
    ///0x54 - Power Port G pull-down control register
    pub pdcrg: PDCRG,
    ///0x58 - Power Port H pull-up control register
    pub pucrh: PUCRH,
    ///0x5c - Power Port H pull-down control register
    pub pdcrh: PDCRH,
}
///Power control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](cr1) module
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
///`read()` method returns [cr1::R](cr1::R) reader structure
impl crate::Readable for CR1 {}
///`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure
impl crate::Writable for CR1 {}
///Power control register 1
pub mod cr1;
///Power control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](cr2) module
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
///`read()` method returns [cr2::R](cr2::R) reader structure
impl crate::Readable for CR2 {}
///`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure
impl crate::Writable for CR2 {}
///Power control register 2
pub mod cr2;
///Power control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](cr3) module
pub type CR3 = crate::Reg<u32, _CR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR3;
///`read()` method returns [cr3::R](cr3::R) reader structure
impl crate::Readable for CR3 {}
///`write(|w| ..)` method takes [cr3::W](cr3::W) writer structure
impl crate::Writable for CR3 {}
///Power control register 3
pub mod cr3;
///Power control register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr4](cr4) module
pub type CR4 = crate::Reg<u32, _CR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR4;
///`read()` method returns [cr4::R](cr4::R) reader structure
impl crate::Readable for CR4 {}
///`write(|w| ..)` method takes [cr4::W](cr4::W) writer structure
impl crate::Writable for CR4 {}
///Power control register 4
pub mod cr4;
///Power status register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr1](sr1) module
pub type SR1 = crate::Reg<u32, _SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR1;
///`read()` method returns [sr1::R](sr1::R) reader structure
impl crate::Readable for SR1 {}
///Power status register 1
pub mod sr1;
///Power status register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr2](sr2) module
pub type SR2 = crate::Reg<u32, _SR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR2;
///`read()` method returns [sr2::R](sr2::R) reader structure
impl crate::Readable for SR2 {}
///Power status register 2
pub mod sr2;
///Power status clear register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scr](scr) module
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
///`write(|w| ..)` method takes [scr::W](scr::W) writer structure
impl crate::Writable for SCR {}
///Power status clear register
pub mod scr;
///Power Port A pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucra](pucra) module
pub type PUCRA = crate::Reg<u32, _PUCRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRA;
///`read()` method returns [pucra::R](pucra::R) reader structure
impl crate::Readable for PUCRA {}
///`write(|w| ..)` method takes [pucra::W](pucra::W) writer structure
impl crate::Writable for PUCRA {}
///Power Port A pull-up control register
pub mod pucra;
///Power Port A pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcra](pdcra) module
pub type PDCRA = crate::Reg<u32, _PDCRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRA;
///`read()` method returns [pdcra::R](pdcra::R) reader structure
impl crate::Readable for PDCRA {}
///`write(|w| ..)` method takes [pdcra::W](pdcra::W) writer structure
impl crate::Writable for PDCRA {}
///Power Port A pull-down control register
pub mod pdcra;
///Power Port B pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrb](pucrb) module
pub type PUCRB = crate::Reg<u32, _PUCRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRB;
///`read()` method returns [pucrb::R](pucrb::R) reader structure
impl crate::Readable for PUCRB {}
///`write(|w| ..)` method takes [pucrb::W](pucrb::W) writer structure
impl crate::Writable for PUCRB {}
///Power Port B pull-up control register
pub mod pucrb;
///Power Port B pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrb](pdcrb) module
pub type PDCRB = crate::Reg<u32, _PDCRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRB;
///`read()` method returns [pdcrb::R](pdcrb::R) reader structure
impl crate::Readable for PDCRB {}
///`write(|w| ..)` method takes [pdcrb::W](pdcrb::W) writer structure
impl crate::Writable for PDCRB {}
///Power Port B pull-down control register
pub mod pdcrb;
///Power Port C pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrc](pucrc) module
pub type PUCRC = crate::Reg<u32, _PUCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRC;
///`read()` method returns [pucrc::R](pucrc::R) reader structure
impl crate::Readable for PUCRC {}
///`write(|w| ..)` method takes [pucrc::W](pucrc::W) writer structure
impl crate::Writable for PUCRC {}
///Power Port C pull-up control register
pub mod pucrc;
///Power Port C pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrc](pdcrc) module
pub type PDCRC = crate::Reg<u32, _PDCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRC;
///`read()` method returns [pdcrc::R](pdcrc::R) reader structure
impl crate::Readable for PDCRC {}
///`write(|w| ..)` method takes [pdcrc::W](pdcrc::W) writer structure
impl crate::Writable for PDCRC {}
///Power Port C pull-down control register
pub mod pdcrc;
///Power Port D pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrd](pucrd) module
pub type PUCRD = crate::Reg<u32, _PUCRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRD;
///`read()` method returns [pucrd::R](pucrd::R) reader structure
impl crate::Readable for PUCRD {}
///`write(|w| ..)` method takes [pucrd::W](pucrd::W) writer structure
impl crate::Writable for PUCRD {}
///Power Port D pull-up control register
pub mod pucrd;
///Power Port D pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrd](pdcrd) module
pub type PDCRD = crate::Reg<u32, _PDCRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRD;
///`read()` method returns [pdcrd::R](pdcrd::R) reader structure
impl crate::Readable for PDCRD {}
///`write(|w| ..)` method takes [pdcrd::W](pdcrd::W) writer structure
impl crate::Writable for PDCRD {}
///Power Port D pull-down control register
pub mod pdcrd;
///Power Port E pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucre](pucre) module
pub type PUCRE = crate::Reg<u32, _PUCRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRE;
///`read()` method returns [pucre::R](pucre::R) reader structure
impl crate::Readable for PUCRE {}
///`write(|w| ..)` method takes [pucre::W](pucre::W) writer structure
impl crate::Writable for PUCRE {}
///Power Port E pull-up control register
pub mod pucre;
///Power Port E pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcre](pdcre) module
pub type PDCRE = crate::Reg<u32, _PDCRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRE;
///`read()` method returns [pdcre::R](pdcre::R) reader structure
impl crate::Readable for PDCRE {}
///`write(|w| ..)` method takes [pdcre::W](pdcre::W) writer structure
impl crate::Writable for PDCRE {}
///Power Port E pull-down control register
pub mod pdcre;
///Power Port F pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrf](pucrf) module
pub type PUCRF = crate::Reg<u32, _PUCRF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRF;
///`read()` method returns [pucrf::R](pucrf::R) reader structure
impl crate::Readable for PUCRF {}
///`write(|w| ..)` method takes [pucrf::W](pucrf::W) writer structure
impl crate::Writable for PUCRF {}
///Power Port F pull-up control register
pub mod pucrf;
///Power Port F pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrf](pdcrf) module
pub type PDCRF = crate::Reg<u32, _PDCRF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRF;
///`read()` method returns [pdcrf::R](pdcrf::R) reader structure
impl crate::Readable for PDCRF {}
///`write(|w| ..)` method takes [pdcrf::W](pdcrf::W) writer structure
impl crate::Writable for PDCRF {}
///Power Port F pull-down control register
pub mod pdcrf;
///Power Port G pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrg](pucrg) module
pub type PUCRG = crate::Reg<u32, _PUCRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRG;
///`read()` method returns [pucrg::R](pucrg::R) reader structure
impl crate::Readable for PUCRG {}
///`write(|w| ..)` method takes [pucrg::W](pucrg::W) writer structure
impl crate::Writable for PUCRG {}
///Power Port G pull-up control register
pub mod pucrg;
///Power Port G pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrg](pdcrg) module
pub type PDCRG = crate::Reg<u32, _PDCRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRG;
///`read()` method returns [pdcrg::R](pdcrg::R) reader structure
impl crate::Readable for PDCRG {}
///`write(|w| ..)` method takes [pdcrg::W](pdcrg::W) writer structure
impl crate::Writable for PDCRG {}
///Power Port G pull-down control register
pub mod pdcrg;
///Power Port H pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrh](pucrh) module
pub type PUCRH = crate::Reg<u32, _PUCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUCRH;
///`read()` method returns [pucrh::R](pucrh::R) reader structure
impl crate::Readable for PUCRH {}
///`write(|w| ..)` method takes [pucrh::W](pucrh::W) writer structure
impl crate::Writable for PUCRH {}
///Power Port H pull-up control register
pub mod pucrh;
///Power Port H pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrh](pdcrh) module
pub type PDCRH = crate::Reg<u32, _PDCRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDCRH;
///`read()` method returns [pdcrh::R](pdcrh::R) reader structure
impl crate::Readable for PDCRH {}
///`write(|w| ..)` method takes [pdcrh::W](pdcrh::W) writer structure
impl crate::Writable for PDCRH {}
///Power Port H pull-down control register
pub mod pdcrh;
