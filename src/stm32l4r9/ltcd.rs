///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    ///0x08 - LTDC Synchronization Size Configuration Register
    pub sscr: SSCR,
    ///0x0c - LTDC Back Porch Configuration Register
    pub bpcr: BPCR,
    ///0x10 - LTDC Active Width Configuration Register
    pub awcr: AWCR,
    ///0x14 - LTDC Total Width Configuration Register
    pub twcr: TWCR,
    ///0x18 - LTDC Global Control Register
    pub gcr: GCR,
    _reserved5: [u8; 8usize],
    ///0x24 - LTDC Shadow Reload Configuration Register
    pub srcr: SRCR,
    _reserved6: [u8; 4usize],
    ///0x2c - LTDC Background Color Configuration Register
    pub bccr: BCCR,
    _reserved7: [u8; 4usize],
    ///0x34 - LTDC Interrupt Enable Register
    pub ier: IER,
    ///0x38 - LTDC Interrupt Status Register
    pub isr: ISR,
    ///0x3c - LTDC Interrupt Clear Register
    pub icr: ICR,
    ///0x40 - LTDC Line Interrupt Position Configuration Register
    pub lipcr: LIPCR,
    ///0x44 - LTDC Current Position Status Register
    pub cpsr: CPSR,
    ///0x48 - LTDC Current Display Status Register
    pub cdsr: CDSR,
    _reserved13: [u8; 56usize],
    ///0x84 - LTDC Layer Control Register
    pub l1cr: L1CR,
    ///0x88 - LTDC Layer Window Horizontal Position Configuration Register
    pub l1whpcr: L1WHPCR,
    ///0x8c - LTDC Layer Window Vertical Position Configuration Register
    pub l1wvpcr: L1WVPCR,
    ///0x90 - LTDC Layer Color Keying Configuration Register
    pub l1ckcr: L1CKCR,
    ///0x94 - LTDC Layer Pixel Format Configuration Register
    pub l1pfcr: L1PFCR,
    ///0x98 - LTDC Layer Constant Alpha Configuration Register
    pub l1cacr: L1CACR,
    ///0x9c - LTDC Layer Default Color Configuration Register
    pub l1dccr: L1DCCR,
    ///0xa0 - LTDC Layer Blending Factors Configuration Register
    pub l1bfcr: L1BFCR,
    _reserved21: [u8; 8usize],
    ///0xac - LTDC Layer Color Frame Buffer Address Register
    pub l1cfbar: L1CFBAR,
    ///0xb0 - LTDC Layer Color Frame Buffer Length Register
    pub l1cfblr: L1CFBLR,
    ///0xb4 - LTDC Layer ColorFrame Buffer Line Number Register
    pub l1cfblnr: L1CFBLNR,
    _reserved24: [u8; 12usize],
    ///0xc4 - LTDC Layerx CLUT Write Register
    pub l1clutwr: L1CLUTWR,
    _reserved25: [u8; 60usize],
    ///0x104 - LTDC Layer Control Register
    pub l2cr: L2CR,
    ///0x108 - LTDC Layerx Window Horizontal Position Configuration Register
    pub l2whpcr: L2WHPCR,
    ///0x10c - LTDC Layer Window Vertical Position Configuration Register
    pub l2wvpcr: L2WVPCR,
    ///0x110 - LTDC Layer Color Keying Configuration Register
    pub l2ckcr: L2CKCR,
    ///0x114 - LTDC Layer Pixel Format Configuration Register
    pub l2pfcr: L2PFCR,
    ///0x118 - LTDC Layer Constant Alpha Configuration Register
    pub l2cacr: L2CACR,
    ///0x11c - LTDC Layer Default Color Configuration Register
    pub l2dccr: L2DCCR,
    _reserved32: [u8; 4usize],
    ///0x124 - LTDC Layer Blending Factors Configuration Register
    pub l2bfcr: L2BFCR,
    _reserved33: [u8; 4usize],
    ///0x12c - LTDC Layer Color Frame Buffer Address Register
    pub l2cfbar: L2CFBAR,
    ///0x130 - LTDC Layer Color Frame Buffer Length Register
    pub l2cfblr: L2CFBLR,
    ///0x134 - LTDC Layer ColorFrame Buffer Line Number Register
    pub l2cfblnr: L2CFBLNR,
    _reserved36: [u8; 12usize],
    ///0x144 - LTDC Layerx CLUT Write Register
    pub l2clutwr: L2CLUTWR,
}
///LTDC Synchronization Size Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sscr](sscr) module
pub type SSCR = crate::Reg<u32, _SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCR;
///`read()` method returns [sscr::R](sscr::R) reader structure
impl crate::Readable for SSCR {}
///`write(|w| ..)` method takes [sscr::W](sscr::W) writer structure
impl crate::Writable for SSCR {}
///LTDC Synchronization Size Configuration Register
pub mod sscr;
///LTDC Back Porch Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bpcr](bpcr) module
pub type BPCR = crate::Reg<u32, _BPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPCR;
///`read()` method returns [bpcr::R](bpcr::R) reader structure
impl crate::Readable for BPCR {}
///`write(|w| ..)` method takes [bpcr::W](bpcr::W) writer structure
impl crate::Writable for BPCR {}
///LTDC Back Porch Configuration Register
pub mod bpcr;
///LTDC Active Width Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awcr](awcr) module
pub type AWCR = crate::Reg<u32, _AWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AWCR;
///`read()` method returns [awcr::R](awcr::R) reader structure
impl crate::Readable for AWCR {}
///`write(|w| ..)` method takes [awcr::W](awcr::W) writer structure
impl crate::Writable for AWCR {}
///LTDC Active Width Configuration Register
pub mod awcr;
///LTDC Total Width Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [twcr](twcr) module
pub type TWCR = crate::Reg<u32, _TWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWCR;
///`read()` method returns [twcr::R](twcr::R) reader structure
impl crate::Readable for TWCR {}
///`write(|w| ..)` method takes [twcr::W](twcr::W) writer structure
impl crate::Writable for TWCR {}
///LTDC Total Width Configuration Register
pub mod twcr;
///LTDC Global Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gcr](gcr) module
pub type GCR = crate::Reg<u32, _GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCR;
///`read()` method returns [gcr::R](gcr::R) reader structure
impl crate::Readable for GCR {}
///`write(|w| ..)` method takes [gcr::W](gcr::W) writer structure
impl crate::Writable for GCR {}
///LTDC Global Control Register
pub mod gcr;
///LTDC Shadow Reload Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [srcr](srcr) module
pub type SRCR = crate::Reg<u32, _SRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR;
///`read()` method returns [srcr::R](srcr::R) reader structure
impl crate::Readable for SRCR {}
///`write(|w| ..)` method takes [srcr::W](srcr::W) writer structure
impl crate::Writable for SRCR {}
///LTDC Shadow Reload Configuration Register
pub mod srcr;
///LTDC Background Color Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bccr](bccr) module
pub type BCCR = crate::Reg<u32, _BCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCCR;
///`read()` method returns [bccr::R](bccr::R) reader structure
impl crate::Readable for BCCR {}
///`write(|w| ..)` method takes [bccr::W](bccr::W) writer structure
impl crate::Writable for BCCR {}
///LTDC Background Color Configuration Register
pub mod bccr;
///LTDC Interrupt Enable Register
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
///LTDC Interrupt Enable Register
pub mod ier;
///LTDC Interrupt Status Register
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
///LTDC Interrupt Status Register
pub mod isr;
///LTDC Interrupt Clear Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](icr) module
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
///`write(|w| ..)` method takes [icr::W](icr::W) writer structure
impl crate::Writable for ICR {}
///LTDC Interrupt Clear Register
pub mod icr;
///LTDC Line Interrupt Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lipcr](lipcr) module
pub type LIPCR = crate::Reg<u32, _LIPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LIPCR;
///`read()` method returns [lipcr::R](lipcr::R) reader structure
impl crate::Readable for LIPCR {}
///`write(|w| ..)` method takes [lipcr::W](lipcr::W) writer structure
impl crate::Writable for LIPCR {}
///LTDC Line Interrupt Position Configuration Register
pub mod lipcr;
///LTDC Current Position Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpsr](cpsr) module
pub type CPSR = crate::Reg<u32, _CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSR;
///`read()` method returns [cpsr::R](cpsr::R) reader structure
impl crate::Readable for CPSR {}
///LTDC Current Position Status Register
pub mod cpsr;
///LTDC Current Display Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cdsr](cdsr) module
pub type CDSR = crate::Reg<u32, _CDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDSR;
///`read()` method returns [cdsr::R](cdsr::R) reader structure
impl crate::Readable for CDSR {}
///LTDC Current Display Status Register
pub mod cdsr;
///LTDC Layer Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1cr](l1cr) module
pub type L1CR = crate::Reg<u32, _L1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CR;
///`read()` method returns [l1cr::R](l1cr::R) reader structure
impl crate::Readable for L1CR {}
///`write(|w| ..)` method takes [l1cr::W](l1cr::W) writer structure
impl crate::Writable for L1CR {}
///LTDC Layer Control Register
pub mod l1cr;
///LTDC Layer Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2cr](l2cr) module
pub type L2CR = crate::Reg<u32, _L2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CR;
///`read()` method returns [l2cr::R](l2cr::R) reader structure
impl crate::Readable for L2CR {}
///`write(|w| ..)` method takes [l2cr::W](l2cr::W) writer structure
impl crate::Writable for L2CR {}
///LTDC Layer Control Register
pub mod l2cr;
///LTDC Layer Window Horizontal Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1whpcr](l1whpcr) module
pub type L1WHPCR = crate::Reg<u32, _L1WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1WHPCR;
///`read()` method returns [l1whpcr::R](l1whpcr::R) reader structure
impl crate::Readable for L1WHPCR {}
///`write(|w| ..)` method takes [l1whpcr::W](l1whpcr::W) writer structure
impl crate::Writable for L1WHPCR {}
///LTDC Layer Window Horizontal Position Configuration Register
pub mod l1whpcr;
///LTDC Layerx Window Horizontal Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2whpcr](l2whpcr) module
pub type L2WHPCR = crate::Reg<u32, _L2WHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2WHPCR;
///`read()` method returns [l2whpcr::R](l2whpcr::R) reader structure
impl crate::Readable for L2WHPCR {}
///`write(|w| ..)` method takes [l2whpcr::W](l2whpcr::W) writer structure
impl crate::Writable for L2WHPCR {}
///LTDC Layerx Window Horizontal Position Configuration Register
pub mod l2whpcr;
///LTDC Layer Window Vertical Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1wvpcr](l1wvpcr) module
pub type L1WVPCR = crate::Reg<u32, _L1WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1WVPCR;
///`read()` method returns [l1wvpcr::R](l1wvpcr::R) reader structure
impl crate::Readable for L1WVPCR {}
///`write(|w| ..)` method takes [l1wvpcr::W](l1wvpcr::W) writer structure
impl crate::Writable for L1WVPCR {}
///LTDC Layer Window Vertical Position Configuration Register
pub mod l1wvpcr;
///LTDC Layer Window Vertical Position Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2wvpcr](l2wvpcr) module
pub type L2WVPCR = crate::Reg<u32, _L2WVPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2WVPCR;
///`read()` method returns [l2wvpcr::R](l2wvpcr::R) reader structure
impl crate::Readable for L2WVPCR {}
///`write(|w| ..)` method takes [l2wvpcr::W](l2wvpcr::W) writer structure
impl crate::Writable for L2WVPCR {}
///LTDC Layer Window Vertical Position Configuration Register
pub mod l2wvpcr;
///LTDC Layer Color Keying Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1ckcr](l1ckcr) module
pub type L1CKCR = crate::Reg<u32, _L1CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CKCR;
///`read()` method returns [l1ckcr::R](l1ckcr::R) reader structure
impl crate::Readable for L1CKCR {}
///`write(|w| ..)` method takes [l1ckcr::W](l1ckcr::W) writer structure
impl crate::Writable for L1CKCR {}
///LTDC Layer Color Keying Configuration Register
pub mod l1ckcr;
///LTDC Layer Color Keying Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2ckcr](l2ckcr) module
pub type L2CKCR = crate::Reg<u32, _L2CKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CKCR;
///`read()` method returns [l2ckcr::R](l2ckcr::R) reader structure
impl crate::Readable for L2CKCR {}
///`write(|w| ..)` method takes [l2ckcr::W](l2ckcr::W) writer structure
impl crate::Writable for L2CKCR {}
///LTDC Layer Color Keying Configuration Register
pub mod l2ckcr;
///LTDC Layer Pixel Format Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1pfcr](l1pfcr) module
pub type L1PFCR = crate::Reg<u32, _L1PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1PFCR;
///`read()` method returns [l1pfcr::R](l1pfcr::R) reader structure
impl crate::Readable for L1PFCR {}
///`write(|w| ..)` method takes [l1pfcr::W](l1pfcr::W) writer structure
impl crate::Writable for L1PFCR {}
///LTDC Layer Pixel Format Configuration Register
pub mod l1pfcr;
///LTDC Layer Pixel Format Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2pfcr](l2pfcr) module
pub type L2PFCR = crate::Reg<u32, _L2PFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2PFCR;
///`read()` method returns [l2pfcr::R](l2pfcr::R) reader structure
impl crate::Readable for L2PFCR {}
///`write(|w| ..)` method takes [l2pfcr::W](l2pfcr::W) writer structure
impl crate::Writable for L2PFCR {}
///LTDC Layer Pixel Format Configuration Register
pub mod l2pfcr;
///LTDC Layer Constant Alpha Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1cacr](l1cacr) module
pub type L1CACR = crate::Reg<u32, _L1CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CACR;
///`read()` method returns [l1cacr::R](l1cacr::R) reader structure
impl crate::Readable for L1CACR {}
///`write(|w| ..)` method takes [l1cacr::W](l1cacr::W) writer structure
impl crate::Writable for L1CACR {}
///LTDC Layer Constant Alpha Configuration Register
pub mod l1cacr;
///LTDC Layer Constant Alpha Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2cacr](l2cacr) module
pub type L2CACR = crate::Reg<u32, _L2CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CACR;
///`read()` method returns [l2cacr::R](l2cacr::R) reader structure
impl crate::Readable for L2CACR {}
///`write(|w| ..)` method takes [l2cacr::W](l2cacr::W) writer structure
impl crate::Writable for L2CACR {}
///LTDC Layer Constant Alpha Configuration Register
pub mod l2cacr;
///LTDC Layer Default Color Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1dccr](l1dccr) module
pub type L1DCCR = crate::Reg<u32, _L1DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1DCCR;
///`read()` method returns [l1dccr::R](l1dccr::R) reader structure
impl crate::Readable for L1DCCR {}
///`write(|w| ..)` method takes [l1dccr::W](l1dccr::W) writer structure
impl crate::Writable for L1DCCR {}
///LTDC Layer Default Color Configuration Register
pub mod l1dccr;
///LTDC Layer Default Color Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2dccr](l2dccr) module
pub type L2DCCR = crate::Reg<u32, _L2DCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2DCCR;
///`read()` method returns [l2dccr::R](l2dccr::R) reader structure
impl crate::Readable for L2DCCR {}
///`write(|w| ..)` method takes [l2dccr::W](l2dccr::W) writer structure
impl crate::Writable for L2DCCR {}
///LTDC Layer Default Color Configuration Register
pub mod l2dccr;
///LTDC Layer Blending Factors Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1bfcr](l1bfcr) module
pub type L1BFCR = crate::Reg<u32, _L1BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1BFCR;
///`read()` method returns [l1bfcr::R](l1bfcr::R) reader structure
impl crate::Readable for L1BFCR {}
///`write(|w| ..)` method takes [l1bfcr::W](l1bfcr::W) writer structure
impl crate::Writable for L1BFCR {}
///LTDC Layer Blending Factors Configuration Register
pub mod l1bfcr;
///LTDC Layer Blending Factors Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2bfcr](l2bfcr) module
pub type L2BFCR = crate::Reg<u32, _L2BFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2BFCR;
///`read()` method returns [l2bfcr::R](l2bfcr::R) reader structure
impl crate::Readable for L2BFCR {}
///`write(|w| ..)` method takes [l2bfcr::W](l2bfcr::W) writer structure
impl crate::Writable for L2BFCR {}
///LTDC Layer Blending Factors Configuration Register
pub mod l2bfcr;
///LTDC Layer Color Frame Buffer Address Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1cfbar](l1cfbar) module
pub type L1CFBAR = crate::Reg<u32, _L1CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CFBAR;
///`read()` method returns [l1cfbar::R](l1cfbar::R) reader structure
impl crate::Readable for L1CFBAR {}
///`write(|w| ..)` method takes [l1cfbar::W](l1cfbar::W) writer structure
impl crate::Writable for L1CFBAR {}
///LTDC Layer Color Frame Buffer Address Register
pub mod l1cfbar;
///LTDC Layer Color Frame Buffer Address Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2cfbar](l2cfbar) module
pub type L2CFBAR = crate::Reg<u32, _L2CFBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CFBAR;
///`read()` method returns [l2cfbar::R](l2cfbar::R) reader structure
impl crate::Readable for L2CFBAR {}
///`write(|w| ..)` method takes [l2cfbar::W](l2cfbar::W) writer structure
impl crate::Writable for L2CFBAR {}
///LTDC Layer Color Frame Buffer Address Register
pub mod l2cfbar;
///LTDC Layer Color Frame Buffer Length Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1cfblr](l1cfblr) module
pub type L1CFBLR = crate::Reg<u32, _L1CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CFBLR;
///`read()` method returns [l1cfblr::R](l1cfblr::R) reader structure
impl crate::Readable for L1CFBLR {}
///`write(|w| ..)` method takes [l1cfblr::W](l1cfblr::W) writer structure
impl crate::Writable for L1CFBLR {}
///LTDC Layer Color Frame Buffer Length Register
pub mod l1cfblr;
///LTDC Layer Color Frame Buffer Length Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2cfblr](l2cfblr) module
pub type L2CFBLR = crate::Reg<u32, _L2CFBLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CFBLR;
///`read()` method returns [l2cfblr::R](l2cfblr::R) reader structure
impl crate::Readable for L2CFBLR {}
///`write(|w| ..)` method takes [l2cfblr::W](l2cfblr::W) writer structure
impl crate::Writable for L2CFBLR {}
///LTDC Layer Color Frame Buffer Length Register
pub mod l2cfblr;
///LTDC Layer ColorFrame Buffer Line Number Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1cfblnr](l1cfblnr) module
pub type L1CFBLNR = crate::Reg<u32, _L1CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CFBLNR;
///`read()` method returns [l1cfblnr::R](l1cfblnr::R) reader structure
impl crate::Readable for L1CFBLNR {}
///`write(|w| ..)` method takes [l1cfblnr::W](l1cfblnr::W) writer structure
impl crate::Writable for L1CFBLNR {}
///LTDC Layer ColorFrame Buffer Line Number Register
pub mod l1cfblnr;
///LTDC Layer ColorFrame Buffer Line Number Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2cfblnr](l2cfblnr) module
pub type L2CFBLNR = crate::Reg<u32, _L2CFBLNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CFBLNR;
///`read()` method returns [l2cfblnr::R](l2cfblnr::R) reader structure
impl crate::Readable for L2CFBLNR {}
///`write(|w| ..)` method takes [l2cfblnr::W](l2cfblnr::W) writer structure
impl crate::Writable for L2CFBLNR {}
///LTDC Layer ColorFrame Buffer Line Number Register
pub mod l2cfblnr;
///LTDC Layerx CLUT Write Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1clutwr](l1clutwr) module
pub type L1CLUTWR = crate::Reg<u32, _L1CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L1CLUTWR;
///`write(|w| ..)` method takes [l1clutwr::W](l1clutwr::W) writer structure
impl crate::Writable for L1CLUTWR {}
///LTDC Layerx CLUT Write Register
pub mod l1clutwr;
///LTDC Layerx CLUT Write Register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2clutwr](l2clutwr) module
pub type L2CLUTWR = crate::Reg<u32, _L2CLUTWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _L2CLUTWR;
///`write(|w| ..)` method takes [l2clutwr::W](l2clutwr::W) writer structure
impl crate::Writable for L2CLUTWR {}
///LTDC Layerx CLUT Write Register
pub mod l2clutwr;
