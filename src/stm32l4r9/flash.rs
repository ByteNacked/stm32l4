///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Access control register
    pub acr: ACR,
    ///0x04 - Power down key register
    pub pdkeyr: PDKEYR,
    ///0x08 - Flash key register
    pub keyr: KEYR,
    ///0x0c - Option byte key register
    pub optkeyr: OPTKEYR,
    ///0x10 - Status register
    pub sr: SR,
    ///0x14 - Flash control register
    pub cr: CR,
    ///0x18 - Flash ECC register
    pub eccr: ECCR,
    _reserved7: [u8; 4usize],
    ///0x20 - Flash option register
    pub optr: OPTR,
    ///0x24 - Flash Bank 1 PCROP Start address register
    pub pcrop1sr: PCROP1SR,
    ///0x28 - Flash Bank 1 PCROP End address register
    pub pcrop1er: PCROP1ER,
    ///0x2c - Flash Bank 1 WRP area A address register
    pub wrp1ar: WRP1AR,
    ///0x30 - Flash Bank 1 WRP area B address register
    pub wrp1br: WRP1BR,
    _reserved12: [u8; 16usize],
    ///0x44 - Flash Bank 2 PCROP Start address register
    pub pcrop2sr: PCROP2SR,
    ///0x48 - Flash Bank 2 PCROP End address register
    pub pcrop2er: PCROP2ER,
    ///0x4c - Flash Bank 2 WRP area A address register
    pub wrp2ar: WRP2AR,
    ///0x50 - Flash Bank 2 WRP area B address register
    pub wrp2br: WRP2BR,
}
///Access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [acr](acr) module
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
///`read()` method returns [acr::R](acr::R) reader structure
impl crate::Readable for ACR {}
///`write(|w| ..)` method takes [acr::W](acr::W) writer structure
impl crate::Writable for ACR {}
///Access control register
pub mod acr;
///Power down key register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdkeyr](pdkeyr) module
pub type PDKEYR = crate::Reg<u32, _PDKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDKEYR;
///`write(|w| ..)` method takes [pdkeyr::W](pdkeyr::W) writer structure
impl crate::Writable for PDKEYR {}
///Power down key register
pub mod pdkeyr;
///Flash key register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr](keyr) module
pub type KEYR = crate::Reg<u32, _KEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYR;
///`write(|w| ..)` method takes [keyr::W](keyr::W) writer structure
impl crate::Writable for KEYR {}
///Flash key register
pub mod keyr;
///Option byte key register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optkeyr](optkeyr) module
pub type OPTKEYR = crate::Reg<u32, _OPTKEYR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTKEYR;
///`write(|w| ..)` method takes [optkeyr::W](optkeyr::W) writer structure
impl crate::Writable for OPTKEYR {}
///Option byte key register
pub mod optkeyr;
///Status register
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
///Status register
pub mod sr;
///Flash control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](cr) module
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
///`read()` method returns [cr::R](cr::R) reader structure
impl crate::Readable for CR {}
///`write(|w| ..)` method takes [cr::W](cr::W) writer structure
impl crate::Writable for CR {}
///Flash control register
pub mod cr;
///Flash ECC register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eccr](eccr) module
pub type ECCR = crate::Reg<u32, _ECCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCR;
///`read()` method returns [eccr::R](eccr::R) reader structure
impl crate::Readable for ECCR {}
///`write(|w| ..)` method takes [eccr::W](eccr::W) writer structure
impl crate::Writable for ECCR {}
///Flash ECC register
pub mod eccr;
///Flash option register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optr](optr) module
pub type OPTR = crate::Reg<u32, _OPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPTR;
///`read()` method returns [optr::R](optr::R) reader structure
impl crate::Readable for OPTR {}
///`write(|w| ..)` method takes [optr::W](optr::W) writer structure
impl crate::Writable for OPTR {}
///Flash option register
pub mod optr;
///Flash Bank 1 PCROP Start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop1sr](pcrop1sr) module
pub type PCROP1SR = crate::Reg<u32, _PCROP1SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP1SR;
///`read()` method returns [pcrop1sr::R](pcrop1sr::R) reader structure
impl crate::Readable for PCROP1SR {}
///`write(|w| ..)` method takes [pcrop1sr::W](pcrop1sr::W) writer structure
impl crate::Writable for PCROP1SR {}
///Flash Bank 1 PCROP Start address register
pub mod pcrop1sr;
///Flash Bank 1 PCROP End address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop1er](pcrop1er) module
pub type PCROP1ER = crate::Reg<u32, _PCROP1ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP1ER;
///`read()` method returns [pcrop1er::R](pcrop1er::R) reader structure
impl crate::Readable for PCROP1ER {}
///`write(|w| ..)` method takes [pcrop1er::W](pcrop1er::W) writer structure
impl crate::Writable for PCROP1ER {}
///Flash Bank 1 PCROP End address register
pub mod pcrop1er;
///Flash Bank 1 WRP area A address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrp1ar](wrp1ar) module
pub type WRP1AR = crate::Reg<u32, _WRP1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP1AR;
///`read()` method returns [wrp1ar::R](wrp1ar::R) reader structure
impl crate::Readable for WRP1AR {}
///`write(|w| ..)` method takes [wrp1ar::W](wrp1ar::W) writer structure
impl crate::Writable for WRP1AR {}
///Flash Bank 1 WRP area A address register
pub mod wrp1ar;
///Flash Bank 1 WRP area B address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrp1br](wrp1br) module
pub type WRP1BR = crate::Reg<u32, _WRP1BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP1BR;
///`read()` method returns [wrp1br::R](wrp1br::R) reader structure
impl crate::Readable for WRP1BR {}
///`write(|w| ..)` method takes [wrp1br::W](wrp1br::W) writer structure
impl crate::Writable for WRP1BR {}
///Flash Bank 1 WRP area B address register
pub mod wrp1br;
///Flash Bank 2 PCROP Start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop2sr](pcrop2sr) module
pub type PCROP2SR = crate::Reg<u32, _PCROP2SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP2SR;
///`read()` method returns [pcrop2sr::R](pcrop2sr::R) reader structure
impl crate::Readable for PCROP2SR {}
///`write(|w| ..)` method takes [pcrop2sr::W](pcrop2sr::W) writer structure
impl crate::Writable for PCROP2SR {}
///Flash Bank 2 PCROP Start address register
pub mod pcrop2sr;
///Flash Bank 2 PCROP End address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop2er](pcrop2er) module
pub type PCROP2ER = crate::Reg<u32, _PCROP2ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCROP2ER;
///`read()` method returns [pcrop2er::R](pcrop2er::R) reader structure
impl crate::Readable for PCROP2ER {}
///`write(|w| ..)` method takes [pcrop2er::W](pcrop2er::W) writer structure
impl crate::Writable for PCROP2ER {}
///Flash Bank 2 PCROP End address register
pub mod pcrop2er;
///Flash Bank 2 WRP area A address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrp2ar](wrp2ar) module
pub type WRP2AR = crate::Reg<u32, _WRP2AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP2AR;
///`read()` method returns [wrp2ar::R](wrp2ar::R) reader structure
impl crate::Readable for WRP2AR {}
///`write(|w| ..)` method takes [wrp2ar::W](wrp2ar::W) writer structure
impl crate::Writable for WRP2AR {}
///Flash Bank 2 WRP area A address register
pub mod wrp2ar;
///Flash Bank 2 WRP area B address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrp2br](wrp2br) module
pub type WRP2BR = crate::Reg<u32, _WRP2BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRP2BR;
///`read()` method returns [wrp2br::R](wrp2br::R) reader structure
impl crate::Readable for WRP2BR {}
///`write(|w| ..)` method takes [wrp2br::W](wrp2br::W) writer structure
impl crate::Writable for WRP2BR {}
///Flash Bank 2 WRP area B address register
pub mod wrp2br;
