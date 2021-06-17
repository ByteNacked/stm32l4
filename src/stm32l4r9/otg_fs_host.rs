///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS host configuration register (OTG_FS_HCFG)
    pub hcfg: HCFG,
    ///0x04 - OTG_FS Host frame interval register
    pub hfir: HFIR,
    ///0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
    pub hfnum: HFNUM,
    _reserved3: [u8; 4usize],
    ///0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
    pub hptxsts: HPTXSTS,
    ///0x14 - OTG_FS Host all channels interrupt register
    pub haint: HAINT,
    ///0x18 - OTG_FS host all channels interrupt mask register
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 36usize],
    ///0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)
    pub hprt: HPRT,
    _reserved7: [u8; 188usize],
    ///0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub hcchar0: HCCHAR0,
    _reserved8: [u8; 4usize],
    ///0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub hcint0: HCINT0,
    ///0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub hcintmsk0: HCINTMSK0,
    ///0x110 - OTG_FS host channel-0 transfer size register
    pub hctsiz0: HCTSIZ0,
    _reserved11: [u8; 12usize],
    ///0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
    pub hcchar1: HCCHAR1,
    _reserved12: [u8; 4usize],
    ///0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
    pub hcint1: HCINT1,
    ///0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
    pub hcintmsk1: HCINTMSK1,
    ///0x130 - OTG_FS host channel-1 transfer size register
    pub hctsiz1: HCTSIZ1,
    _reserved15: [u8; 12usize],
    ///0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
    pub hcchar2: HCCHAR2,
    _reserved16: [u8; 4usize],
    ///0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
    pub hcint2: HCINT2,
    ///0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
    pub hcintmsk2: HCINTMSK2,
    ///0x150 - OTG_FS host channel-2 transfer size register
    pub hctsiz2: HCTSIZ2,
    _reserved19: [u8; 12usize],
    ///0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
    pub hcchar3: HCCHAR3,
    _reserved20: [u8; 4usize],
    ///0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
    pub hcint3: HCINT3,
    ///0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
    pub hcintmsk3: HCINTMSK3,
    ///0x170 - OTG_FS host channel-3 transfer size register
    pub hctsiz3: HCTSIZ3,
    _reserved23: [u8; 12usize],
    ///0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
    pub hcchar4: HCCHAR4,
    _reserved24: [u8; 4usize],
    ///0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
    pub hcint4: HCINT4,
    ///0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
    pub hcintmsk4: HCINTMSK4,
    ///0x190 - OTG_FS host channel-x transfer size register
    pub hctsiz4: HCTSIZ4,
    _reserved27: [u8; 12usize],
    ///0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
    pub hcchar5: HCCHAR5,
    _reserved28: [u8; 4usize],
    ///0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
    pub hcint5: HCINT5,
    ///0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
    pub hcintmsk5: HCINTMSK5,
    ///0x1b0 - OTG_FS host channel-5 transfer size register
    pub hctsiz5: HCTSIZ5,
    _reserved31: [u8; 12usize],
    ///0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
    pub hcchar6: HCCHAR6,
    _reserved32: [u8; 4usize],
    ///0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
    pub hcint6: HCINT6,
    ///0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
    pub hcintmsk6: HCINTMSK6,
    ///0x1d0 - OTG_FS host channel-6 transfer size register
    pub hctsiz6: HCTSIZ6,
    _reserved35: [u8; 12usize],
    ///0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
    pub hcchar7: HCCHAR7,
    _reserved36: [u8; 4usize],
    ///0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
    pub hcint7: HCINT7,
    ///0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
    pub hcintmsk7: HCINTMSK7,
    ///0x1f0 - OTG_FS host channel-7 transfer size register
    pub hctsiz7: HCTSIZ7,
}
///OTG_FS host configuration register (OTG_FS_HCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcfg](hcfg) module
pub type HCFG = crate::Reg<u32, _HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCFG;
///`read()` method returns [hcfg::R](hcfg::R) reader structure
impl crate::Readable for HCFG {}
///`write(|w| ..)` method takes [hcfg::W](hcfg::W) writer structure
impl crate::Writable for HCFG {}
///OTG_FS host configuration register (OTG_FS_HCFG)
pub mod hcfg;
///OTG_FS Host frame interval register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hfir](hfir) module
pub type HFIR = crate::Reg<u32, _HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFIR;
///`read()` method returns [hfir::R](hfir::R) reader structure
impl crate::Readable for HFIR {}
///`write(|w| ..)` method takes [hfir::W](hfir::W) writer structure
impl crate::Writable for HFIR {}
///OTG_FS Host frame interval register
pub mod hfir;
///OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hfnum](hfnum) module
pub type HFNUM = crate::Reg<u32, _HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFNUM;
///`read()` method returns [hfnum::R](hfnum::R) reader structure
impl crate::Readable for HFNUM {}
///OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
pub mod hfnum;
///OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hptxsts](hptxsts) module
pub type HPTXSTS = crate::Reg<u32, _HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXSTS;
///`read()` method returns [hptxsts::R](hptxsts::R) reader structure
impl crate::Readable for HPTXSTS {}
///`write(|w| ..)` method takes [hptxsts::W](hptxsts::W) writer structure
impl crate::Writable for HPTXSTS {}
///OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
pub mod hptxsts;
///OTG_FS Host all channels interrupt register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [haint](haint) module
pub type HAINT = crate::Reg<u32, _HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINT;
///`read()` method returns [haint::R](haint::R) reader structure
impl crate::Readable for HAINT {}
///OTG_FS Host all channels interrupt register
pub mod haint;
///OTG_FS host all channels interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [haintmsk](haintmsk) module
pub type HAINTMSK = crate::Reg<u32, _HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINTMSK;
///`read()` method returns [haintmsk::R](haintmsk::R) reader structure
impl crate::Readable for HAINTMSK {}
///`write(|w| ..)` method takes [haintmsk::W](haintmsk::W) writer structure
impl crate::Writable for HAINTMSK {}
///OTG_FS host all channels interrupt mask register
pub mod haintmsk;
///OTG_FS host port control and status register (OTG_FS_HPRT)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hprt](hprt) module
pub type HPRT = crate::Reg<u32, _HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPRT;
///`read()` method returns [hprt::R](hprt::R) reader structure
impl crate::Readable for HPRT {}
///`write(|w| ..)` method takes [hprt::W](hprt::W) writer structure
impl crate::Writable for HPRT {}
///OTG_FS host port control and status register (OTG_FS_HPRT)
pub mod hprt;
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar0](hcchar0) module
pub type HCCHAR0 = crate::Reg<u32, _HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR0;
///`read()` method returns [hcchar0::R](hcchar0::R) reader structure
impl crate::Readable for HCCHAR0 {}
///`write(|w| ..)` method takes [hcchar0::W](hcchar0::W) writer structure
impl crate::Writable for HCCHAR0 {}
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod hcchar0;
///OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar1](hcchar1) module
pub type HCCHAR1 = crate::Reg<u32, _HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR1;
///`read()` method returns [hcchar1::R](hcchar1::R) reader structure
impl crate::Readable for HCCHAR1 {}
///`write(|w| ..)` method takes [hcchar1::W](hcchar1::W) writer structure
impl crate::Writable for HCCHAR1 {}
///OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
pub mod hcchar1;
///OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar2](hcchar2) module
pub type HCCHAR2 = crate::Reg<u32, _HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR2;
///`read()` method returns [hcchar2::R](hcchar2::R) reader structure
impl crate::Readable for HCCHAR2 {}
///`write(|w| ..)` method takes [hcchar2::W](hcchar2::W) writer structure
impl crate::Writable for HCCHAR2 {}
///OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
pub mod hcchar2;
///OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar3](hcchar3) module
pub type HCCHAR3 = crate::Reg<u32, _HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR3;
///`read()` method returns [hcchar3::R](hcchar3::R) reader structure
impl crate::Readable for HCCHAR3 {}
///`write(|w| ..)` method takes [hcchar3::W](hcchar3::W) writer structure
impl crate::Writable for HCCHAR3 {}
///OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
pub mod hcchar3;
///OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar4](hcchar4) module
pub type HCCHAR4 = crate::Reg<u32, _HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR4;
///`read()` method returns [hcchar4::R](hcchar4::R) reader structure
impl crate::Readable for HCCHAR4 {}
///`write(|w| ..)` method takes [hcchar4::W](hcchar4::W) writer structure
impl crate::Writable for HCCHAR4 {}
///OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
pub mod hcchar4;
///OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar5](hcchar5) module
pub type HCCHAR5 = crate::Reg<u32, _HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR5;
///`read()` method returns [hcchar5::R](hcchar5::R) reader structure
impl crate::Readable for HCCHAR5 {}
///`write(|w| ..)` method takes [hcchar5::W](hcchar5::W) writer structure
impl crate::Writable for HCCHAR5 {}
///OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
pub mod hcchar5;
///OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar6](hcchar6) module
pub type HCCHAR6 = crate::Reg<u32, _HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR6;
///`read()` method returns [hcchar6::R](hcchar6::R) reader structure
impl crate::Readable for HCCHAR6 {}
///`write(|w| ..)` method takes [hcchar6::W](hcchar6::W) writer structure
impl crate::Writable for HCCHAR6 {}
///OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
pub mod hcchar6;
///OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar7](hcchar7) module
pub type HCCHAR7 = crate::Reg<u32, _HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR7;
///`read()` method returns [hcchar7::R](hcchar7::R) reader structure
impl crate::Readable for HCCHAR7 {}
///`write(|w| ..)` method takes [hcchar7::W](hcchar7::W) writer structure
impl crate::Writable for HCCHAR7 {}
///OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
pub mod hcchar7;
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint0](hcint0) module
pub type HCINT0 = crate::Reg<u32, _HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT0;
///`read()` method returns [hcint0::R](hcint0::R) reader structure
impl crate::Readable for HCINT0 {}
///`write(|w| ..)` method takes [hcint0::W](hcint0::W) writer structure
impl crate::Writable for HCINT0 {}
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod hcint0;
///OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint1](hcint1) module
pub type HCINT1 = crate::Reg<u32, _HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT1;
///`read()` method returns [hcint1::R](hcint1::R) reader structure
impl crate::Readable for HCINT1 {}
///`write(|w| ..)` method takes [hcint1::W](hcint1::W) writer structure
impl crate::Writable for HCINT1 {}
///OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
pub mod hcint1;
///OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint2](hcint2) module
pub type HCINT2 = crate::Reg<u32, _HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT2;
///`read()` method returns [hcint2::R](hcint2::R) reader structure
impl crate::Readable for HCINT2 {}
///`write(|w| ..)` method takes [hcint2::W](hcint2::W) writer structure
impl crate::Writable for HCINT2 {}
///OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
pub mod hcint2;
///OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint3](hcint3) module
pub type HCINT3 = crate::Reg<u32, _HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT3;
///`read()` method returns [hcint3::R](hcint3::R) reader structure
impl crate::Readable for HCINT3 {}
///`write(|w| ..)` method takes [hcint3::W](hcint3::W) writer structure
impl crate::Writable for HCINT3 {}
///OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
pub mod hcint3;
///OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint4](hcint4) module
pub type HCINT4 = crate::Reg<u32, _HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT4;
///`read()` method returns [hcint4::R](hcint4::R) reader structure
impl crate::Readable for HCINT4 {}
///`write(|w| ..)` method takes [hcint4::W](hcint4::W) writer structure
impl crate::Writable for HCINT4 {}
///OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
pub mod hcint4;
///OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint5](hcint5) module
pub type HCINT5 = crate::Reg<u32, _HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT5;
///`read()` method returns [hcint5::R](hcint5::R) reader structure
impl crate::Readable for HCINT5 {}
///`write(|w| ..)` method takes [hcint5::W](hcint5::W) writer structure
impl crate::Writable for HCINT5 {}
///OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
pub mod hcint5;
///OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint6](hcint6) module
pub type HCINT6 = crate::Reg<u32, _HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT6;
///`read()` method returns [hcint6::R](hcint6::R) reader structure
impl crate::Readable for HCINT6 {}
///`write(|w| ..)` method takes [hcint6::W](hcint6::W) writer structure
impl crate::Writable for HCINT6 {}
///OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
pub mod hcint6;
///OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcint7](hcint7) module
pub type HCINT7 = crate::Reg<u32, _HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT7;
///`read()` method returns [hcint7::R](hcint7::R) reader structure
impl crate::Readable for HCINT7 {}
///`write(|w| ..)` method takes [hcint7::W](hcint7::W) writer structure
impl crate::Writable for HCINT7 {}
///OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
pub mod hcint7;
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk0](hcintmsk0) module
pub type HCINTMSK0 = crate::Reg<u32, _HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK0;
///`read()` method returns [hcintmsk0::R](hcintmsk0::R) reader structure
impl crate::Readable for HCINTMSK0 {}
///`write(|w| ..)` method takes [hcintmsk0::W](hcintmsk0::W) writer structure
impl crate::Writable for HCINTMSK0 {}
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod hcintmsk0;
///OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk1](hcintmsk1) module
pub type HCINTMSK1 = crate::Reg<u32, _HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK1;
///`read()` method returns [hcintmsk1::R](hcintmsk1::R) reader structure
impl crate::Readable for HCINTMSK1 {}
///`write(|w| ..)` method takes [hcintmsk1::W](hcintmsk1::W) writer structure
impl crate::Writable for HCINTMSK1 {}
///OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
pub mod hcintmsk1;
///OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk2](hcintmsk2) module
pub type HCINTMSK2 = crate::Reg<u32, _HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK2;
///`read()` method returns [hcintmsk2::R](hcintmsk2::R) reader structure
impl crate::Readable for HCINTMSK2 {}
///`write(|w| ..)` method takes [hcintmsk2::W](hcintmsk2::W) writer structure
impl crate::Writable for HCINTMSK2 {}
///OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
pub mod hcintmsk2;
///OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk3](hcintmsk3) module
pub type HCINTMSK3 = crate::Reg<u32, _HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK3;
///`read()` method returns [hcintmsk3::R](hcintmsk3::R) reader structure
impl crate::Readable for HCINTMSK3 {}
///`write(|w| ..)` method takes [hcintmsk3::W](hcintmsk3::W) writer structure
impl crate::Writable for HCINTMSK3 {}
///OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
pub mod hcintmsk3;
///OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk4](hcintmsk4) module
pub type HCINTMSK4 = crate::Reg<u32, _HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK4;
///`read()` method returns [hcintmsk4::R](hcintmsk4::R) reader structure
impl crate::Readable for HCINTMSK4 {}
///`write(|w| ..)` method takes [hcintmsk4::W](hcintmsk4::W) writer structure
impl crate::Writable for HCINTMSK4 {}
///OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
pub mod hcintmsk4;
///OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk5](hcintmsk5) module
pub type HCINTMSK5 = crate::Reg<u32, _HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK5;
///`read()` method returns [hcintmsk5::R](hcintmsk5::R) reader structure
impl crate::Readable for HCINTMSK5 {}
///`write(|w| ..)` method takes [hcintmsk5::W](hcintmsk5::W) writer structure
impl crate::Writable for HCINTMSK5 {}
///OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
pub mod hcintmsk5;
///OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk6](hcintmsk6) module
pub type HCINTMSK6 = crate::Reg<u32, _HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK6;
///`read()` method returns [hcintmsk6::R](hcintmsk6::R) reader structure
impl crate::Readable for HCINTMSK6 {}
///`write(|w| ..)` method takes [hcintmsk6::W](hcintmsk6::W) writer structure
impl crate::Writable for HCINTMSK6 {}
///OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
pub mod hcintmsk6;
///OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk7](hcintmsk7) module
pub type HCINTMSK7 = crate::Reg<u32, _HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK7;
///`read()` method returns [hcintmsk7::R](hcintmsk7::R) reader structure
impl crate::Readable for HCINTMSK7 {}
///`write(|w| ..)` method takes [hcintmsk7::W](hcintmsk7::W) writer structure
impl crate::Writable for HCINTMSK7 {}
///OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
pub mod hcintmsk7;
///OTG_FS host channel-0 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz0](hctsiz0) module
pub type HCTSIZ0 = crate::Reg<u32, _HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ0;
///`read()` method returns [hctsiz0::R](hctsiz0::R) reader structure
impl crate::Readable for HCTSIZ0 {}
///`write(|w| ..)` method takes [hctsiz0::W](hctsiz0::W) writer structure
impl crate::Writable for HCTSIZ0 {}
///OTG_FS host channel-0 transfer size register
pub mod hctsiz0;
///OTG_FS host channel-1 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz1](hctsiz1) module
pub type HCTSIZ1 = crate::Reg<u32, _HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ1;
///`read()` method returns [hctsiz1::R](hctsiz1::R) reader structure
impl crate::Readable for HCTSIZ1 {}
///`write(|w| ..)` method takes [hctsiz1::W](hctsiz1::W) writer structure
impl crate::Writable for HCTSIZ1 {}
///OTG_FS host channel-1 transfer size register
pub mod hctsiz1;
///OTG_FS host channel-2 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz2](hctsiz2) module
pub type HCTSIZ2 = crate::Reg<u32, _HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ2;
///`read()` method returns [hctsiz2::R](hctsiz2::R) reader structure
impl crate::Readable for HCTSIZ2 {}
///`write(|w| ..)` method takes [hctsiz2::W](hctsiz2::W) writer structure
impl crate::Writable for HCTSIZ2 {}
///OTG_FS host channel-2 transfer size register
pub mod hctsiz2;
///OTG_FS host channel-3 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz3](hctsiz3) module
pub type HCTSIZ3 = crate::Reg<u32, _HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ3;
///`read()` method returns [hctsiz3::R](hctsiz3::R) reader structure
impl crate::Readable for HCTSIZ3 {}
///`write(|w| ..)` method takes [hctsiz3::W](hctsiz3::W) writer structure
impl crate::Writable for HCTSIZ3 {}
///OTG_FS host channel-3 transfer size register
pub mod hctsiz3;
///OTG_FS host channel-x transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz4](hctsiz4) module
pub type HCTSIZ4 = crate::Reg<u32, _HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ4;
///`read()` method returns [hctsiz4::R](hctsiz4::R) reader structure
impl crate::Readable for HCTSIZ4 {}
///`write(|w| ..)` method takes [hctsiz4::W](hctsiz4::W) writer structure
impl crate::Writable for HCTSIZ4 {}
///OTG_FS host channel-x transfer size register
pub mod hctsiz4;
///OTG_FS host channel-5 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz5](hctsiz5) module
pub type HCTSIZ5 = crate::Reg<u32, _HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ5;
///`read()` method returns [hctsiz5::R](hctsiz5::R) reader structure
impl crate::Readable for HCTSIZ5 {}
///`write(|w| ..)` method takes [hctsiz5::W](hctsiz5::W) writer structure
impl crate::Writable for HCTSIZ5 {}
///OTG_FS host channel-5 transfer size register
pub mod hctsiz5;
///OTG_FS host channel-6 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz6](hctsiz6) module
pub type HCTSIZ6 = crate::Reg<u32, _HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ6;
///`read()` method returns [hctsiz6::R](hctsiz6::R) reader structure
impl crate::Readable for HCTSIZ6 {}
///`write(|w| ..)` method takes [hctsiz6::W](hctsiz6::W) writer structure
impl crate::Writable for HCTSIZ6 {}
///OTG_FS host channel-6 transfer size register
pub mod hctsiz6;
///OTG_FS host channel-7 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hctsiz7](hctsiz7) module
pub type HCTSIZ7 = crate::Reg<u32, _HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ7;
///`read()` method returns [hctsiz7::R](hctsiz7::R) reader structure
impl crate::Readable for HCTSIZ7 {}
///`write(|w| ..)` method takes [hctsiz7::W](hctsiz7::W) writer structure
impl crate::Writable for HCTSIZ7 {}
///OTG_FS host channel-7 transfer size register
pub mod hctsiz7;
