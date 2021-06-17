///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - memory remap register
    pub memrmp: MEMRMP,
    ///0x04 - configuration register 1
    pub cfgr1: CFGR1,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
    ///0x18 - SCSR
    pub scsr: SCSR,
    ///0x1c - CFGR2
    pub cfgr2: CFGR2,
    ///0x20 - SWPR
    pub swpr: SWPR,
    ///0x24 - SKR
    pub skr: SKR,
}
///memory remap register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [memrmp](memrmp) module
pub type MEMRMP = crate::Reg<u32, _MEMRMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEMRMP;
///`read()` method returns [memrmp::R](memrmp::R) reader structure
impl crate::Readable for MEMRMP {}
///`write(|w| ..)` method takes [memrmp::W](memrmp::W) writer structure
impl crate::Writable for MEMRMP {}
///memory remap register
pub mod memrmp;
///configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](cfgr1) module
pub type CFGR1 = crate::Reg<u32, _CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR1;
///`read()` method returns [cfgr1::R](cfgr1::R) reader structure
impl crate::Readable for CFGR1 {}
///`write(|w| ..)` method takes [cfgr1::W](cfgr1::W) writer structure
impl crate::Writable for CFGR1 {}
///configuration register 1
pub mod cfgr1;
///external interrupt configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr1](exticr1) module
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
///`read()` method returns [exticr1::R](exticr1::R) reader structure
impl crate::Readable for EXTICR1 {}
///`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure
impl crate::Writable for EXTICR1 {}
///external interrupt configuration register 1
pub mod exticr1;
///external interrupt configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr2](exticr2) module
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
///`read()` method returns [exticr2::R](exticr2::R) reader structure
impl crate::Readable for EXTICR2 {}
///`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure
impl crate::Writable for EXTICR2 {}
///external interrupt configuration register 2
pub mod exticr2;
///external interrupt configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr3](exticr3) module
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
///`read()` method returns [exticr3::R](exticr3::R) reader structure
impl crate::Readable for EXTICR3 {}
///`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure
impl crate::Writable for EXTICR3 {}
///external interrupt configuration register 3
pub mod exticr3;
///external interrupt configuration register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr4](exticr4) module
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
///`read()` method returns [exticr4::R](exticr4::R) reader structure
impl crate::Readable for EXTICR4 {}
///`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure
impl crate::Writable for EXTICR4 {}
///external interrupt configuration register 4
pub mod exticr4;
///SCSR
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scsr](scsr) module
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
///`read()` method returns [scsr::R](scsr::R) reader structure
impl crate::Readable for SCSR {}
///`write(|w| ..)` method takes [scsr::W](scsr::W) writer structure
impl crate::Writable for SCSR {}
///SCSR
pub mod scsr;
///CFGR2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](cfgr2) module
pub type CFGR2 = crate::Reg<u32, _CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR2;
///`read()` method returns [cfgr2::R](cfgr2::R) reader structure
impl crate::Readable for CFGR2 {}
///`write(|w| ..)` method takes [cfgr2::W](cfgr2::W) writer structure
impl crate::Writable for CFGR2 {}
///CFGR2
pub mod cfgr2;
///SWPR
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swpr](swpr) module
pub type SWPR = crate::Reg<u32, _SWPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWPR;
///`write(|w| ..)` method takes [swpr::W](swpr::W) writer structure
impl crate::Writable for SWPR {}
///SWPR
pub mod swpr;
///SKR
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [skr](skr) module
pub type SKR = crate::Reg<u32, _SKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SKR;
///`write(|w| ..)` method takes [skr::W](skr::W) writer structure
impl crate::Writable for SKR {}
///SKR
pub mod skr;
