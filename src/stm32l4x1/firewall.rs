///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Code segment start address
    pub cssa: CSSA,
    ///0x04 - Code segment length
    pub csl: CSL,
    ///0x08 - Non-volatile data segment start address
    pub nvdssa: NVDSSA,
    ///0x0c - Non-volatile data segment length
    pub nvdsl: NVDSL,
    ///0x10 - Volatile data segment start address
    pub vdssa: VDSSA,
    ///0x14 - Volatile data segment length
    pub vdsl: VDSL,
    _reserved6: [u8; 8usize],
    ///0x20 - Configuration register
    pub cr: CR,
}
///Code segment start address
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cssa](cssa) module
pub type CSSA = crate::Reg<u32, _CSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSSA;
///`read()` method returns [cssa::R](cssa::R) reader structure
impl crate::Readable for CSSA {}
///`write(|w| ..)` method takes [cssa::W](cssa::W) writer structure
impl crate::Writable for CSSA {}
///Code segment start address
pub mod cssa;
///Code segment length
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csl](csl) module
pub type CSL = crate::Reg<u32, _CSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSL;
///`read()` method returns [csl::R](csl::R) reader structure
impl crate::Readable for CSL {}
///`write(|w| ..)` method takes [csl::W](csl::W) writer structure
impl crate::Writable for CSL {}
///Code segment length
pub mod csl;
///Non-volatile data segment start address
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [nvdssa](nvdssa) module
pub type NVDSSA = crate::Reg<u32, _NVDSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVDSSA;
///`read()` method returns [nvdssa::R](nvdssa::R) reader structure
impl crate::Readable for NVDSSA {}
///`write(|w| ..)` method takes [nvdssa::W](nvdssa::W) writer structure
impl crate::Writable for NVDSSA {}
///Non-volatile data segment start address
pub mod nvdssa;
///Non-volatile data segment length
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [nvdsl](nvdsl) module
pub type NVDSL = crate::Reg<u32, _NVDSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVDSL;
///`read()` method returns [nvdsl::R](nvdsl::R) reader structure
impl crate::Readable for NVDSL {}
///`write(|w| ..)` method takes [nvdsl::W](nvdsl::W) writer structure
impl crate::Writable for NVDSL {}
///Non-volatile data segment length
pub mod nvdsl;
///Volatile data segment start address
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vdssa](vdssa) module
pub type VDSSA = crate::Reg<u32, _VDSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDSSA;
///`read()` method returns [vdssa::R](vdssa::R) reader structure
impl crate::Readable for VDSSA {}
///`write(|w| ..)` method takes [vdssa::W](vdssa::W) writer structure
impl crate::Writable for VDSSA {}
///Volatile data segment start address
pub mod vdssa;
///Volatile data segment length
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vdsl](vdsl) module
pub type VDSL = crate::Reg<u32, _VDSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDSL;
///`read()` method returns [vdsl::R](vdsl::R) reader structure
impl crate::Readable for VDSL {}
///`write(|w| ..)` method takes [vdsl::W](vdsl::W) writer structure
impl crate::Writable for VDSL {}
///Volatile data segment length
pub mod vdsl;
///Configuration register
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
///Configuration register
pub mod cr;
