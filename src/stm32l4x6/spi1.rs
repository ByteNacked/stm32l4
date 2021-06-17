///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - status register
    pub sr: SR,
    ///0x0c - data register
    pub dr: DR,
    ///0x10 - CRC polynomial register
    pub crcpr: CRCPR,
    ///0x14 - RX CRC register
    pub rxcrcr: RXCRCR,
    ///0x18 - TX CRC register
    pub txcrcr: TXCRCR,
}
///control register 1
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
///control register 1
pub mod cr1;
///control register 2
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
///control register 2
pub mod cr2;
///status register
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
///status register
pub mod sr;
///data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](dr) module
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
///`read()` method returns [dr::R](dr::R) reader structure
impl crate::Readable for DR {}
///`write(|w| ..)` method takes [dr::W](dr::W) writer structure
impl crate::Writable for DR {}
///data register
pub mod dr;
///CRC polynomial register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crcpr](crcpr) module
pub type CRCPR = crate::Reg<u32, _CRCPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCPR;
///`read()` method returns [crcpr::R](crcpr::R) reader structure
impl crate::Readable for CRCPR {}
///`write(|w| ..)` method takes [crcpr::W](crcpr::W) writer structure
impl crate::Writable for CRCPR {}
///CRC polynomial register
pub mod crcpr;
///RX CRC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxcrcr](rxcrcr) module
pub type RXCRCR = crate::Reg<u32, _RXCRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCRCR;
///`read()` method returns [rxcrcr::R](rxcrcr::R) reader structure
impl crate::Readable for RXCRCR {}
///RX CRC register
pub mod rxcrcr;
///TX CRC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txcrcr](txcrcr) module
pub type TXCRCR = crate::Reg<u32, _TXCRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCRCR;
///`read()` method returns [txcrcr::R](txcrcr::R) reader structure
impl crate::Readable for TXCRCR {}
///TX CRC register
pub mod txcrcr;
