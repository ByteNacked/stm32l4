///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ADC Common status register
    pub csr: CSR,
    _reserved1: [u8; 4usize],
    ///0x08 - ADC common control register
    pub ccr: CCR,
    ///0x0c - ADC common regular data register for dual and triple modes
    pub cdr: CDR,
}
///ADC Common status register
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
///ADC Common status register
pub mod csr;
///ADC common control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](ccr) module
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
///`read()` method returns [ccr::R](ccr::R) reader structure
impl crate::Readable for CCR {}
///`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure
impl crate::Writable for CCR {}
///ADC common control register
pub mod ccr;
///ADC common regular data register for dual and triple modes
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cdr](cdr) module
pub type CDR = crate::Reg<u32, _CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDR;
///`read()` method returns [cdr::R](cdr::R) reader structure
impl crate::Readable for CDR {}
///ADC common regular data register for dual and triple modes
pub mod cdr;
