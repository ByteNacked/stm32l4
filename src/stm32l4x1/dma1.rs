///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt status register
    pub isr: ISR,
    ///0x04 - interrupt flag clear register
    pub ifcr: IFCR,
    ///0x08 - channel x configuration register
    pub ccr1: CCR1,
    ///0x0c - channel x number of data register
    pub cndtr1: CNDTR1,
    ///0x10 - channel x peripheral address register
    pub cpar1: CPAR1,
    ///0x14 - channel x memory address register
    pub cmar1: CMAR1,
    _reserved6: [u8; 4usize],
    ///0x1c - channel x configuration register
    pub ccr2: CCR2,
    ///0x20 - channel x number of data register
    pub cndtr2: CNDTR2,
    ///0x24 - channel x peripheral address register
    pub cpar2: CPAR2,
    ///0x28 - channel x memory address register
    pub cmar2: CMAR2,
    _reserved10: [u8; 4usize],
    ///0x30 - channel x configuration register
    pub ccr3: CCR3,
    ///0x34 - channel x number of data register
    pub cndtr3: CNDTR3,
    ///0x38 - channel x peripheral address register
    pub cpar3: CPAR3,
    ///0x3c - channel x memory address register
    pub cmar3: CMAR3,
    _reserved14: [u8; 4usize],
    ///0x44 - channel x configuration register
    pub ccr4: CCR4,
    ///0x48 - channel x number of data register
    pub cndtr4: CNDTR4,
    ///0x4c - channel x peripheral address register
    pub cpar4: CPAR4,
    ///0x50 - channel x memory address register
    pub cmar4: CMAR4,
    _reserved18: [u8; 4usize],
    ///0x58 - channel x configuration register
    pub ccr5: CCR5,
    ///0x5c - channel x number of data register
    pub cndtr5: CNDTR5,
    ///0x60 - channel x peripheral address register
    pub cpar5: CPAR5,
    ///0x64 - channel x memory address register
    pub cmar5: CMAR5,
    _reserved22: [u8; 4usize],
    ///0x6c - channel x configuration register
    pub ccr6: CCR6,
    ///0x70 - channel x number of data register
    pub cndtr6: CNDTR6,
    ///0x74 - channel x peripheral address register
    pub cpar6: CPAR6,
    ///0x78 - channel x memory address register
    pub cmar6: CMAR6,
    _reserved26: [u8; 4usize],
    ///0x80 - channel x configuration register
    pub ccr7: CCR7,
    ///0x84 - channel x number of data register
    pub cndtr7: CNDTR7,
    ///0x88 - channel x peripheral address register
    pub cpar7: CPAR7,
    ///0x8c - channel x memory address register
    pub cmar7: CMAR7,
    _reserved30: [u8; 24usize],
    ///0xa8 - channel selection register
    pub cselr: CSELR,
}
///interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](isr) module
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
///`read()` method returns [isr::R](isr::R) reader structure
impl crate::Readable for ISR {}
///interrupt status register
pub mod isr;
///interrupt flag clear register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ifcr](ifcr) module
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
///`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure
impl crate::Writable for IFCR {}
///interrupt flag clear register
pub mod ifcr;
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr1](ccr1) module
pub type CCR1 = crate::Reg<u32, _CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR1;
///`read()` method returns [ccr1::R](ccr1::R) reader structure
impl crate::Readable for CCR1 {}
///`write(|w| ..)` method takes [ccr1::W](ccr1::W) writer structure
impl crate::Writable for CCR1 {}
///channel x configuration register
pub mod ccr1;
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr1](cndtr1) module
pub type CNDTR1 = crate::Reg<u32, _CNDTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR1;
///`read()` method returns [cndtr1::R](cndtr1::R) reader structure
impl crate::Readable for CNDTR1 {}
///`write(|w| ..)` method takes [cndtr1::W](cndtr1::W) writer structure
impl crate::Writable for CNDTR1 {}
///channel x number of data register
pub mod cndtr1;
///channel x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpar1](cpar1) module
pub type CPAR1 = crate::Reg<u32, _CPAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR1;
///`read()` method returns [cpar1::R](cpar1::R) reader structure
impl crate::Readable for CPAR1 {}
///`write(|w| ..)` method takes [cpar1::W](cpar1::W) writer structure
impl crate::Writable for CPAR1 {}
///channel x peripheral address register
pub mod cpar1;
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar1](cmar1) module
pub type CMAR1 = crate::Reg<u32, _CMAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR1;
///`read()` method returns [cmar1::R](cmar1::R) reader structure
impl crate::Readable for CMAR1 {}
///`write(|w| ..)` method takes [cmar1::W](cmar1::W) writer structure
impl crate::Writable for CMAR1 {}
///channel x memory address register
pub mod cmar1;
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr2](ccr2) module
pub type CCR2 = crate::Reg<u32, _CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR2;
///`read()` method returns [ccr2::R](ccr2::R) reader structure
impl crate::Readable for CCR2 {}
///`write(|w| ..)` method takes [ccr2::W](ccr2::W) writer structure
impl crate::Writable for CCR2 {}
///channel x configuration register
pub mod ccr2;
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr2](cndtr2) module
pub type CNDTR2 = crate::Reg<u32, _CNDTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR2;
///`read()` method returns [cndtr2::R](cndtr2::R) reader structure
impl crate::Readable for CNDTR2 {}
///`write(|w| ..)` method takes [cndtr2::W](cndtr2::W) writer structure
impl crate::Writable for CNDTR2 {}
///channel x number of data register
pub mod cndtr2;
///channel x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpar2](cpar2) module
pub type CPAR2 = crate::Reg<u32, _CPAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR2;
///`read()` method returns [cpar2::R](cpar2::R) reader structure
impl crate::Readable for CPAR2 {}
///`write(|w| ..)` method takes [cpar2::W](cpar2::W) writer structure
impl crate::Writable for CPAR2 {}
///channel x peripheral address register
pub mod cpar2;
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar2](cmar2) module
pub type CMAR2 = crate::Reg<u32, _CMAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR2;
///`read()` method returns [cmar2::R](cmar2::R) reader structure
impl crate::Readable for CMAR2 {}
///`write(|w| ..)` method takes [cmar2::W](cmar2::W) writer structure
impl crate::Writable for CMAR2 {}
///channel x memory address register
pub mod cmar2;
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr3](ccr3) module
pub type CCR3 = crate::Reg<u32, _CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR3;
///`read()` method returns [ccr3::R](ccr3::R) reader structure
impl crate::Readable for CCR3 {}
///`write(|w| ..)` method takes [ccr3::W](ccr3::W) writer structure
impl crate::Writable for CCR3 {}
///channel x configuration register
pub mod ccr3;
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr3](cndtr3) module
pub type CNDTR3 = crate::Reg<u32, _CNDTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR3;
///`read()` method returns [cndtr3::R](cndtr3::R) reader structure
impl crate::Readable for CNDTR3 {}
///`write(|w| ..)` method takes [cndtr3::W](cndtr3::W) writer structure
impl crate::Writable for CNDTR3 {}
///channel x number of data register
pub mod cndtr3;
///channel x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpar3](cpar3) module
pub type CPAR3 = crate::Reg<u32, _CPAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR3;
///`read()` method returns [cpar3::R](cpar3::R) reader structure
impl crate::Readable for CPAR3 {}
///`write(|w| ..)` method takes [cpar3::W](cpar3::W) writer structure
impl crate::Writable for CPAR3 {}
///channel x peripheral address register
pub mod cpar3;
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar3](cmar3) module
pub type CMAR3 = crate::Reg<u32, _CMAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR3;
///`read()` method returns [cmar3::R](cmar3::R) reader structure
impl crate::Readable for CMAR3 {}
///`write(|w| ..)` method takes [cmar3::W](cmar3::W) writer structure
impl crate::Writable for CMAR3 {}
///channel x memory address register
pub mod cmar3;
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr4](ccr4) module
pub type CCR4 = crate::Reg<u32, _CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR4;
///`read()` method returns [ccr4::R](ccr4::R) reader structure
impl crate::Readable for CCR4 {}
///`write(|w| ..)` method takes [ccr4::W](ccr4::W) writer structure
impl crate::Writable for CCR4 {}
///channel x configuration register
pub mod ccr4;
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr4](cndtr4) module
pub type CNDTR4 = crate::Reg<u32, _CNDTR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR4;
///`read()` method returns [cndtr4::R](cndtr4::R) reader structure
impl crate::Readable for CNDTR4 {}
///`write(|w| ..)` method takes [cndtr4::W](cndtr4::W) writer structure
impl crate::Writable for CNDTR4 {}
///channel x number of data register
pub mod cndtr4;
///channel x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpar4](cpar4) module
pub type CPAR4 = crate::Reg<u32, _CPAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR4;
///`read()` method returns [cpar4::R](cpar4::R) reader structure
impl crate::Readable for CPAR4 {}
///`write(|w| ..)` method takes [cpar4::W](cpar4::W) writer structure
impl crate::Writable for CPAR4 {}
///channel x peripheral address register
pub mod cpar4;
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar4](cmar4) module
pub type CMAR4 = crate::Reg<u32, _CMAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR4;
///`read()` method returns [cmar4::R](cmar4::R) reader structure
impl crate::Readable for CMAR4 {}
///`write(|w| ..)` method takes [cmar4::W](cmar4::W) writer structure
impl crate::Writable for CMAR4 {}
///channel x memory address register
pub mod cmar4;
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr5](ccr5) module
pub type CCR5 = crate::Reg<u32, _CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR5;
///`read()` method returns [ccr5::R](ccr5::R) reader structure
impl crate::Readable for CCR5 {}
///`write(|w| ..)` method takes [ccr5::W](ccr5::W) writer structure
impl crate::Writable for CCR5 {}
///channel x configuration register
pub mod ccr5;
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr5](cndtr5) module
pub type CNDTR5 = crate::Reg<u32, _CNDTR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR5;
///`read()` method returns [cndtr5::R](cndtr5::R) reader structure
impl crate::Readable for CNDTR5 {}
///`write(|w| ..)` method takes [cndtr5::W](cndtr5::W) writer structure
impl crate::Writable for CNDTR5 {}
///channel x number of data register
pub mod cndtr5;
///channel x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpar5](cpar5) module
pub type CPAR5 = crate::Reg<u32, _CPAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR5;
///`read()` method returns [cpar5::R](cpar5::R) reader structure
impl crate::Readable for CPAR5 {}
///`write(|w| ..)` method takes [cpar5::W](cpar5::W) writer structure
impl crate::Writable for CPAR5 {}
///channel x peripheral address register
pub mod cpar5;
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar5](cmar5) module
pub type CMAR5 = crate::Reg<u32, _CMAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR5;
///`read()` method returns [cmar5::R](cmar5::R) reader structure
impl crate::Readable for CMAR5 {}
///`write(|w| ..)` method takes [cmar5::W](cmar5::W) writer structure
impl crate::Writable for CMAR5 {}
///channel x memory address register
pub mod cmar5;
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr6](ccr6) module
pub type CCR6 = crate::Reg<u32, _CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR6;
///`read()` method returns [ccr6::R](ccr6::R) reader structure
impl crate::Readable for CCR6 {}
///`write(|w| ..)` method takes [ccr6::W](ccr6::W) writer structure
impl crate::Writable for CCR6 {}
///channel x configuration register
pub mod ccr6;
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr6](cndtr6) module
pub type CNDTR6 = crate::Reg<u32, _CNDTR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR6;
///`read()` method returns [cndtr6::R](cndtr6::R) reader structure
impl crate::Readable for CNDTR6 {}
///`write(|w| ..)` method takes [cndtr6::W](cndtr6::W) writer structure
impl crate::Writable for CNDTR6 {}
///channel x number of data register
pub mod cndtr6;
///channel x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpar6](cpar6) module
pub type CPAR6 = crate::Reg<u32, _CPAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR6;
///`read()` method returns [cpar6::R](cpar6::R) reader structure
impl crate::Readable for CPAR6 {}
///`write(|w| ..)` method takes [cpar6::W](cpar6::W) writer structure
impl crate::Writable for CPAR6 {}
///channel x peripheral address register
pub mod cpar6;
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar6](cmar6) module
pub type CMAR6 = crate::Reg<u32, _CMAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR6;
///`read()` method returns [cmar6::R](cmar6::R) reader structure
impl crate::Readable for CMAR6 {}
///`write(|w| ..)` method takes [cmar6::W](cmar6::W) writer structure
impl crate::Writable for CMAR6 {}
///channel x memory address register
pub mod cmar6;
///channel x configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr7](ccr7) module
pub type CCR7 = crate::Reg<u32, _CCR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR7;
///`read()` method returns [ccr7::R](ccr7::R) reader structure
impl crate::Readable for CCR7 {}
///`write(|w| ..)` method takes [ccr7::W](ccr7::W) writer structure
impl crate::Writable for CCR7 {}
///channel x configuration register
pub mod ccr7;
///channel x number of data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cndtr7](cndtr7) module
pub type CNDTR7 = crate::Reg<u32, _CNDTR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNDTR7;
///`read()` method returns [cndtr7::R](cndtr7::R) reader structure
impl crate::Readable for CNDTR7 {}
///`write(|w| ..)` method takes [cndtr7::W](cndtr7::W) writer structure
impl crate::Writable for CNDTR7 {}
///channel x number of data register
pub mod cndtr7;
///channel x peripheral address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpar7](cpar7) module
pub type CPAR7 = crate::Reg<u32, _CPAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPAR7;
///`read()` method returns [cpar7::R](cpar7::R) reader structure
impl crate::Readable for CPAR7 {}
///`write(|w| ..)` method takes [cpar7::W](cpar7::W) writer structure
impl crate::Writable for CPAR7 {}
///channel x peripheral address register
pub mod cpar7;
///channel x memory address register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmar7](cmar7) module
pub type CMAR7 = crate::Reg<u32, _CMAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMAR7;
///`read()` method returns [cmar7::R](cmar7::R) reader structure
impl crate::Readable for CMAR7 {}
///`write(|w| ..)` method takes [cmar7::W](cmar7::W) writer structure
impl crate::Writable for CMAR7 {}
///channel x memory address register
pub mod cmar7;
///channel selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cselr](cselr) module
pub type CSELR = crate::Reg<u32, _CSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSELR;
///`read()` method returns [cselr::R](cselr::R) reader structure
impl crate::Readable for CSELR {}
///`write(|w| ..)` method takes [cselr::W](cselr::W) writer structure
impl crate::Writable for CSELR {}
///channel selection register
pub mod cselr;
