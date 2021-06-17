///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - channel 0 configuration register
    pub c0cr: C0CR,
    ///0x04 - channel 1 configuration register
    pub c1cr: C1CR,
    ///0x08 - channel 2 configuration register
    pub c2cr: C2CR,
    ///0x0c - channel 3 configuration register
    pub c3cr: C3CR,
    ///0x10 - channel 4 configuration register
    pub c4cr: C4CR,
    ///0x14 - channel 5 configuration register
    pub c5cr: C5CR,
    ///0x18 - channel 6 configuration register
    pub c6cr: C6CR,
    ///0x1c - channel 7 configuration register
    pub c7cr: C7CR,
    ///0x20 - channel 8 configuration register
    pub c8cr: C8CR,
    ///0x24 - channel 9 configuration register
    pub c9cr: C9CR,
    ///0x28 - channel 10 configuration register
    pub c10cr: C10CR,
    ///0x2c - channel 11 configuration register
    pub c11cr: C11CR,
    ///0x30 - channel 12 configuration register
    pub c12cr: C12CR,
    ///0x34 - channel 13 configuration register
    pub c13cr: C13CR,
    _reserved14: [u8; 72usize],
    ///0x80 - channel status register
    pub csr: CSR,
    ///0x84 - clear flag register
    pub cfr: CFR,
    _reserved16: [u8; 120usize],
    ///0x100 - request generator channel 0 configuration register
    pub rg0cr: RG0CR,
    ///0x104 - request generator channel 1 configuration register
    pub rg1cr: RG1CR,
    ///0x108 - request generator channel 2 configuration register
    pub rg2cr: RG2CR,
    ///0x10c - request generator channel 3 configuration register
    pub rg3cr: RG3CR,
    _reserved20: [u8; 48usize],
    ///0x140 - request generator interrupt status register
    pub rgsr: RGSR,
    ///0x144 - request generator interrupt clear flag register
    pub rgcfr: RGCFR,
}
///channel 0 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c0cr](c0cr) module
pub type C0CR = crate::Reg<u32, _C0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0CR;
///`read()` method returns [c0cr::R](c0cr::R) reader structure
impl crate::Readable for C0CR {}
///`write(|w| ..)` method takes [c0cr::W](c0cr::W) writer structure
impl crate::Writable for C0CR {}
///channel 0 configuration register
pub mod c0cr;
///channel 1 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1cr](c1cr) module
pub type C1CR = crate::Reg<u32, _C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1CR;
///`read()` method returns [c1cr::R](c1cr::R) reader structure
impl crate::Readable for C1CR {}
///`write(|w| ..)` method takes [c1cr::W](c1cr::W) writer structure
impl crate::Writable for C1CR {}
///channel 1 configuration register
pub mod c1cr;
///channel 2 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2cr](c2cr) module
pub type C2CR = crate::Reg<u32, _C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2CR;
///`read()` method returns [c2cr::R](c2cr::R) reader structure
impl crate::Readable for C2CR {}
///`write(|w| ..)` method takes [c2cr::W](c2cr::W) writer structure
impl crate::Writable for C2CR {}
///channel 2 configuration register
pub mod c2cr;
///channel 3 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c3cr](c3cr) module
pub type C3CR = crate::Reg<u32, _C3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3CR;
///`read()` method returns [c3cr::R](c3cr::R) reader structure
impl crate::Readable for C3CR {}
///`write(|w| ..)` method takes [c3cr::W](c3cr::W) writer structure
impl crate::Writable for C3CR {}
///channel 3 configuration register
pub mod c3cr;
///channel 4 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c4cr](c4cr) module
pub type C4CR = crate::Reg<u32, _C4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4CR;
///`read()` method returns [c4cr::R](c4cr::R) reader structure
impl crate::Readable for C4CR {}
///`write(|w| ..)` method takes [c4cr::W](c4cr::W) writer structure
impl crate::Writable for C4CR {}
///channel 4 configuration register
pub mod c4cr;
///channel 5 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c5cr](c5cr) module
pub type C5CR = crate::Reg<u32, _C5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5CR;
///`read()` method returns [c5cr::R](c5cr::R) reader structure
impl crate::Readable for C5CR {}
///`write(|w| ..)` method takes [c5cr::W](c5cr::W) writer structure
impl crate::Writable for C5CR {}
///channel 5 configuration register
pub mod c5cr;
///channel 6 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c6cr](c6cr) module
pub type C6CR = crate::Reg<u32, _C6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6CR;
///`read()` method returns [c6cr::R](c6cr::R) reader structure
impl crate::Readable for C6CR {}
///`write(|w| ..)` method takes [c6cr::W](c6cr::W) writer structure
impl crate::Writable for C6CR {}
///channel 6 configuration register
pub mod c6cr;
///channel 7 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c7cr](c7cr) module
pub type C7CR = crate::Reg<u32, _C7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7CR;
///`read()` method returns [c7cr::R](c7cr::R) reader structure
impl crate::Readable for C7CR {}
///`write(|w| ..)` method takes [c7cr::W](c7cr::W) writer structure
impl crate::Writable for C7CR {}
///channel 7 configuration register
pub mod c7cr;
///channel 8 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c8cr](c8cr) module
pub type C8CR = crate::Reg<u32, _C8CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C8CR;
///`read()` method returns [c8cr::R](c8cr::R) reader structure
impl crate::Readable for C8CR {}
///`write(|w| ..)` method takes [c8cr::W](c8cr::W) writer structure
impl crate::Writable for C8CR {}
///channel 8 configuration register
pub mod c8cr;
///channel 9 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c9cr](c9cr) module
pub type C9CR = crate::Reg<u32, _C9CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C9CR;
///`read()` method returns [c9cr::R](c9cr::R) reader structure
impl crate::Readable for C9CR {}
///`write(|w| ..)` method takes [c9cr::W](c9cr::W) writer structure
impl crate::Writable for C9CR {}
///channel 9 configuration register
pub mod c9cr;
///channel 10 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c10cr](c10cr) module
pub type C10CR = crate::Reg<u32, _C10CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C10CR;
///`read()` method returns [c10cr::R](c10cr::R) reader structure
impl crate::Readable for C10CR {}
///`write(|w| ..)` method takes [c10cr::W](c10cr::W) writer structure
impl crate::Writable for C10CR {}
///channel 10 configuration register
pub mod c10cr;
///channel 11 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c11cr](c11cr) module
pub type C11CR = crate::Reg<u32, _C11CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C11CR;
///`read()` method returns [c11cr::R](c11cr::R) reader structure
impl crate::Readable for C11CR {}
///`write(|w| ..)` method takes [c11cr::W](c11cr::W) writer structure
impl crate::Writable for C11CR {}
///channel 11 configuration register
pub mod c11cr;
///channel 12 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c12cr](c12cr) module
pub type C12CR = crate::Reg<u32, _C12CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C12CR;
///`read()` method returns [c12cr::R](c12cr::R) reader structure
impl crate::Readable for C12CR {}
///`write(|w| ..)` method takes [c12cr::W](c12cr::W) writer structure
impl crate::Writable for C12CR {}
///channel 12 configuration register
pub mod c12cr;
///channel 13 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c13cr](c13cr) module
pub type C13CR = crate::Reg<u32, _C13CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C13CR;
///`read()` method returns [c13cr::R](c13cr::R) reader structure
impl crate::Readable for C13CR {}
///`write(|w| ..)` method takes [c13cr::W](c13cr::W) writer structure
impl crate::Writable for C13CR {}
///channel 13 configuration register
pub mod c13cr;
///channel status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](csr) module
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
///`read()` method returns [csr::R](csr::R) reader structure
impl crate::Readable for CSR {}
///channel status register
pub mod csr;
///clear flag register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfr](cfr) module
pub type CFR = crate::Reg<u32, _CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFR;
///`write(|w| ..)` method takes [cfr::W](cfr::W) writer structure
impl crate::Writable for CFR {}
///clear flag register
pub mod cfr;
///request generator channel 0 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rg0cr](rg0cr) module
pub type RG0CR = crate::Reg<u32, _RG0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG0CR;
///`read()` method returns [rg0cr::R](rg0cr::R) reader structure
impl crate::Readable for RG0CR {}
///`write(|w| ..)` method takes [rg0cr::W](rg0cr::W) writer structure
impl crate::Writable for RG0CR {}
///request generator channel 0 configuration register
pub mod rg0cr;
///request generator channel 1 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rg1cr](rg1cr) module
pub type RG1CR = crate::Reg<u32, _RG1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG1CR;
///`read()` method returns [rg1cr::R](rg1cr::R) reader structure
impl crate::Readable for RG1CR {}
///`write(|w| ..)` method takes [rg1cr::W](rg1cr::W) writer structure
impl crate::Writable for RG1CR {}
///request generator channel 1 configuration register
pub mod rg1cr;
///request generator channel 2 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rg2cr](rg2cr) module
pub type RG2CR = crate::Reg<u32, _RG2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG2CR;
///`read()` method returns [rg2cr::R](rg2cr::R) reader structure
impl crate::Readable for RG2CR {}
///`write(|w| ..)` method takes [rg2cr::W](rg2cr::W) writer structure
impl crate::Writable for RG2CR {}
///request generator channel 2 configuration register
pub mod rg2cr;
///request generator channel 3 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rg3cr](rg3cr) module
pub type RG3CR = crate::Reg<u32, _RG3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RG3CR;
///`read()` method returns [rg3cr::R](rg3cr::R) reader structure
impl crate::Readable for RG3CR {}
///`write(|w| ..)` method takes [rg3cr::W](rg3cr::W) writer structure
impl crate::Writable for RG3CR {}
///request generator channel 3 configuration register
pub mod rg3cr;
///request generator interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rgsr](rgsr) module
pub type RGSR = crate::Reg<u32, _RGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGSR;
///`read()` method returns [rgsr::R](rgsr::R) reader structure
impl crate::Readable for RGSR {}
///request generator interrupt status register
pub mod rgsr;
///request generator interrupt clear flag register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rgcfr](rgcfr) module
pub type RGCFR = crate::Reg<u32, _RGCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGCFR;
///`write(|w| ..)` method takes [rgcfr::W](rgcfr::W) writer structure
impl crate::Writable for RGCFR {}
///request generator interrupt clear flag register
pub mod rgcfr;
