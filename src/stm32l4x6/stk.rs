///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SysTick control and status register
    pub ctrl: CTRL,
    ///0x04 - SysTick reload value register
    pub load: LOAD,
    ///0x08 - SysTick current value register
    pub val: VAL,
    ///0x0c - SysTick calibration value register
    pub calib: CALIB,
}
///SysTick control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctrl](ctrl) module
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
///`read()` method returns [ctrl::R](ctrl::R) reader structure
impl crate::Readable for CTRL {}
///`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure
impl crate::Writable for CTRL {}
///SysTick control and status register
pub mod ctrl;
///SysTick reload value register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [load](load) module
pub type LOAD = crate::Reg<u32, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
///`read()` method returns [load::R](load::R) reader structure
impl crate::Readable for LOAD {}
///`write(|w| ..)` method takes [load::W](load::W) writer structure
impl crate::Writable for LOAD {}
///SysTick reload value register
pub mod load;
///SysTick current value register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [val](val) module
pub type VAL = crate::Reg<u32, _VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VAL;
///`read()` method returns [val::R](val::R) reader structure
impl crate::Readable for VAL {}
///`write(|w| ..)` method takes [val::W](val::W) writer structure
impl crate::Writable for VAL {}
///SysTick current value register
pub mod val;
///SysTick calibration value register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calib](calib) module
pub type CALIB = crate::Reg<u32, _CALIB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALIB;
///`read()` method returns [calib::R](calib::R) reader structure
impl crate::Readable for CALIB {}
///`write(|w| ..)` method takes [calib::W](calib::W) writer structure
impl crate::Writable for CALIB {}
///SysTick calibration value register
pub mod calib;
