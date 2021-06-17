///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt and status register
    pub isr: ISR,
    ///0x04 - interrupt enable register
    pub ier: IER,
    ///0x08 - control register
    pub cr: CR,
    ///0x0c - configuration register
    pub cfgr: CFGR,
    ///0x10 - configuration register
    pub cfgr2: CFGR2,
    ///0x14 - sample time register 1
    pub smpr1: SMPR1,
    ///0x18 - sample time register 2
    pub smpr2: SMPR2,
    _reserved7: [u8; 4usize],
    ///0x20 - watchdog threshold register 1
    pub tr1: TR1,
    ///0x24 - watchdog threshold register
    pub tr2: TR2,
    ///0x28 - watchdog threshold register 3
    pub tr3: TR3,
    _reserved10: [u8; 4usize],
    ///0x30 - regular sequence register 1
    pub sqr1: SQR1,
    ///0x34 - regular sequence register 2
    pub sqr2: SQR2,
    ///0x38 - regular sequence register 3
    pub sqr3: SQR3,
    ///0x3c - regular sequence register 4
    pub sqr4: SQR4,
    ///0x40 - regular Data Register
    pub dr: DR,
    _reserved15: [u8; 8usize],
    ///0x4c - injected sequence register
    pub jsqr: JSQR,
    _reserved16: [u8; 16usize],
    ///0x60 - offset register 1
    pub ofr1: OFR1,
    ///0x64 - offset register 2
    pub ofr2: OFR2,
    ///0x68 - offset register 3
    pub ofr3: OFR3,
    ///0x6c - offset register 4
    pub ofr4: OFR4,
    _reserved20: [u8; 16usize],
    ///0x80 - injected data register 1
    pub jdr1: JDR1,
    ///0x84 - injected data register 2
    pub jdr2: JDR2,
    ///0x88 - injected data register 3
    pub jdr3: JDR3,
    ///0x8c - injected data register 4
    pub jdr4: JDR4,
    _reserved24: [u8; 16usize],
    ///0xa0 - Analog Watchdog 2 Configuration Register
    pub awd2cr: AWD2CR,
    ///0xa4 - Analog Watchdog 3 Configuration Register
    pub awd3cr: AWD3CR,
    _reserved26: [u8; 8usize],
    ///0xb0 - Differential Mode Selection Register 2
    pub difsel: DIFSEL,
    ///0xb4 - Calibration Factors
    pub calfact: CALFACT,
}
///interrupt and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](isr) module
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
///`read()` method returns [isr::R](isr::R) reader structure
impl crate::Readable for ISR {}
///`write(|w| ..)` method takes [isr::W](isr::W) writer structure
impl crate::Writable for ISR {}
///interrupt and status register
pub mod isr;
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](ier) module
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
///`read()` method returns [ier::R](ier::R) reader structure
impl crate::Readable for IER {}
///`write(|w| ..)` method takes [ier::W](ier::W) writer structure
impl crate::Writable for IER {}
///interrupt enable register
pub mod ier;
///control register
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
///control register
pub mod cr;
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](cfgr) module
pub type CFGR = crate::Reg<u32, _CFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGR;
///`read()` method returns [cfgr::R](cfgr::R) reader structure
impl crate::Readable for CFGR {}
///`write(|w| ..)` method takes [cfgr::W](cfgr::W) writer structure
impl crate::Writable for CFGR {}
///configuration register
pub mod cfgr;
///configuration register
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
///configuration register
pub mod cfgr2;
///sample time register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr1](smpr1) module
pub type SMPR1 = crate::Reg<u32, _SMPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR1;
///`read()` method returns [smpr1::R](smpr1::R) reader structure
impl crate::Readable for SMPR1 {}
///`write(|w| ..)` method takes [smpr1::W](smpr1::W) writer structure
impl crate::Writable for SMPR1 {}
///sample time register 1
pub mod smpr1;
///sample time register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr2](smpr2) module
pub type SMPR2 = crate::Reg<u32, _SMPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPR2;
///`read()` method returns [smpr2::R](smpr2::R) reader structure
impl crate::Readable for SMPR2 {}
///`write(|w| ..)` method takes [smpr2::W](smpr2::W) writer structure
impl crate::Writable for SMPR2 {}
///sample time register 2
pub mod smpr2;
///watchdog threshold register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr1](tr1) module
pub type TR1 = crate::Reg<u32, _TR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR1;
///`read()` method returns [tr1::R](tr1::R) reader structure
impl crate::Readable for TR1 {}
///`write(|w| ..)` method takes [tr1::W](tr1::W) writer structure
impl crate::Writable for TR1 {}
///watchdog threshold register 1
pub mod tr1;
///watchdog threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr2](tr2) module
pub type TR2 = crate::Reg<u32, _TR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR2;
///`read()` method returns [tr2::R](tr2::R) reader structure
impl crate::Readable for TR2 {}
///`write(|w| ..)` method takes [tr2::W](tr2::W) writer structure
impl crate::Writable for TR2 {}
///watchdog threshold register
pub mod tr2;
///watchdog threshold register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr3](tr3) module
pub type TR3 = crate::Reg<u32, _TR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR3;
///`read()` method returns [tr3::R](tr3::R) reader structure
impl crate::Readable for TR3 {}
///`write(|w| ..)` method takes [tr3::W](tr3::W) writer structure
impl crate::Writable for TR3 {}
///watchdog threshold register 3
pub mod tr3;
///regular sequence register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr1](sqr1) module
pub type SQR1 = crate::Reg<u32, _SQR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR1;
///`read()` method returns [sqr1::R](sqr1::R) reader structure
impl crate::Readable for SQR1 {}
///`write(|w| ..)` method takes [sqr1::W](sqr1::W) writer structure
impl crate::Writable for SQR1 {}
///regular sequence register 1
pub mod sqr1;
///regular sequence register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr2](sqr2) module
pub type SQR2 = crate::Reg<u32, _SQR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR2;
///`read()` method returns [sqr2::R](sqr2::R) reader structure
impl crate::Readable for SQR2 {}
///`write(|w| ..)` method takes [sqr2::W](sqr2::W) writer structure
impl crate::Writable for SQR2 {}
///regular sequence register 2
pub mod sqr2;
///regular sequence register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr3](sqr3) module
pub type SQR3 = crate::Reg<u32, _SQR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR3;
///`read()` method returns [sqr3::R](sqr3::R) reader structure
impl crate::Readable for SQR3 {}
///`write(|w| ..)` method takes [sqr3::W](sqr3::W) writer structure
impl crate::Writable for SQR3 {}
///regular sequence register 3
pub mod sqr3;
///regular sequence register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr4](sqr4) module
pub type SQR4 = crate::Reg<u32, _SQR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SQR4;
///`read()` method returns [sqr4::R](sqr4::R) reader structure
impl crate::Readable for SQR4 {}
///`write(|w| ..)` method takes [sqr4::W](sqr4::W) writer structure
impl crate::Writable for SQR4 {}
///regular sequence register 4
pub mod sqr4;
///regular Data Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](dr) module
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
///`read()` method returns [dr::R](dr::R) reader structure
impl crate::Readable for DR {}
///regular Data Register
pub mod dr;
///injected sequence register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jsqr](jsqr) module
pub type JSQR = crate::Reg<u32, _JSQR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JSQR;
///`read()` method returns [jsqr::R](jsqr::R) reader structure
impl crate::Readable for JSQR {}
///`write(|w| ..)` method takes [jsqr::W](jsqr::W) writer structure
impl crate::Writable for JSQR {}
///injected sequence register
pub mod jsqr;
///offset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr1](ofr1) module
pub type OFR1 = crate::Reg<u32, _OFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR1;
///`read()` method returns [ofr1::R](ofr1::R) reader structure
impl crate::Readable for OFR1 {}
///`write(|w| ..)` method takes [ofr1::W](ofr1::W) writer structure
impl crate::Writable for OFR1 {}
///offset register 1
pub mod ofr1;
///offset register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr2](ofr2) module
pub type OFR2 = crate::Reg<u32, _OFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR2;
///`read()` method returns [ofr2::R](ofr2::R) reader structure
impl crate::Readable for OFR2 {}
///`write(|w| ..)` method takes [ofr2::W](ofr2::W) writer structure
impl crate::Writable for OFR2 {}
///offset register 2
pub mod ofr2;
///offset register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr3](ofr3) module
pub type OFR3 = crate::Reg<u32, _OFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR3;
///`read()` method returns [ofr3::R](ofr3::R) reader structure
impl crate::Readable for OFR3 {}
///`write(|w| ..)` method takes [ofr3::W](ofr3::W) writer structure
impl crate::Writable for OFR3 {}
///offset register 3
pub mod ofr3;
///offset register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr4](ofr4) module
pub type OFR4 = crate::Reg<u32, _OFR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFR4;
///`read()` method returns [ofr4::R](ofr4::R) reader structure
impl crate::Readable for OFR4 {}
///`write(|w| ..)` method takes [ofr4::W](ofr4::W) writer structure
impl crate::Writable for OFR4 {}
///offset register 4
pub mod ofr4;
///injected data register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdr1](jdr1) module
pub type JDR1 = crate::Reg<u32, _JDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR1;
///`read()` method returns [jdr1::R](jdr1::R) reader structure
impl crate::Readable for JDR1 {}
///injected data register 1
pub mod jdr1;
///injected data register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdr2](jdr2) module
pub type JDR2 = crate::Reg<u32, _JDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR2;
///`read()` method returns [jdr2::R](jdr2::R) reader structure
impl crate::Readable for JDR2 {}
///injected data register 2
pub mod jdr2;
///injected data register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdr3](jdr3) module
pub type JDR3 = crate::Reg<u32, _JDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR3;
///`read()` method returns [jdr3::R](jdr3::R) reader structure
impl crate::Readable for JDR3 {}
///injected data register 3
pub mod jdr3;
///injected data register 4
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdr4](jdr4) module
pub type JDR4 = crate::Reg<u32, _JDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JDR4;
///`read()` method returns [jdr4::R](jdr4::R) reader structure
impl crate::Readable for JDR4 {}
///injected data register 4
pub mod jdr4;
///Analog Watchdog 2 Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awd2cr](awd2cr) module
pub type AWD2CR = crate::Reg<u32, _AWD2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD2CR;
///`read()` method returns [awd2cr::R](awd2cr::R) reader structure
impl crate::Readable for AWD2CR {}
///`write(|w| ..)` method takes [awd2cr::W](awd2cr::W) writer structure
impl crate::Writable for AWD2CR {}
///Analog Watchdog 2 Configuration Register
pub mod awd2cr;
///Analog Watchdog 3 Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awd3cr](awd3cr) module
pub type AWD3CR = crate::Reg<u32, _AWD3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWD3CR;
///`read()` method returns [awd3cr::R](awd3cr::R) reader structure
impl crate::Readable for AWD3CR {}
///`write(|w| ..)` method takes [awd3cr::W](awd3cr::W) writer structure
impl crate::Writable for AWD3CR {}
///Analog Watchdog 3 Configuration Register
pub mod awd3cr;
///Differential Mode Selection Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [difsel](difsel) module
pub type DIFSEL = crate::Reg<u32, _DIFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIFSEL;
///`read()` method returns [difsel::R](difsel::R) reader structure
impl crate::Readable for DIFSEL {}
///`write(|w| ..)` method takes [difsel::W](difsel::W) writer structure
impl crate::Writable for DIFSEL {}
///Differential Mode Selection Register 2
pub mod difsel;
///Calibration Factors
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calfact](calfact) module
pub type CALFACT = crate::Reg<u32, _CALFACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALFACT;
///`read()` method returns [calfact::R](calfact::R) reader structure
impl crate::Readable for CALFACT {}
///`write(|w| ..)` method takes [calfact::W](calfact::W) writer structure
impl crate::Writable for CALFACT {}
///Calibration Factors
pub mod calfact;
