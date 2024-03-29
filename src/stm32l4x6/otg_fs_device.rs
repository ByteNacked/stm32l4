///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS device configuration register (OTG_FS_DCFG)
    pub fs_dcfg: FS_DCFG,
    ///0x04 - OTG_FS device control register (OTG_FS_DCTL)
    pub fs_dctl: FS_DCTL,
    ///0x08 - OTG_FS device status register (OTG_FS_DSTS)
    pub fs_dsts: FS_DSTS,
    _reserved3: [u8; 4usize],
    ///0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
    pub fs_diepmsk: FS_DIEPMSK,
    ///0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
    pub fs_doepmsk: FS_DOEPMSK,
    ///0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
    pub fs_daint: FS_DAINT,
    ///0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
    pub fs_daintmsk: FS_DAINTMSK,
    _reserved7: [u8; 8usize],
    ///0x28 - OTG_FS device VBUS discharge time register
    pub dvbusdis: DVBUSDIS,
    ///0x2c - OTG_FS device VBUS pulsing time register
    pub dvbuspulse: DVBUSPULSE,
    _reserved9: [u8; 4usize],
    ///0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register
    pub diepempmsk: DIEPEMPMSK,
    _reserved10: [u8; 200usize],
    ///0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
    pub fs_diepctl0: FS_DIEPCTL0,
    _reserved11: [u8; 4usize],
    ///0x108 - device endpoint-x interrupt register
    pub diepint0: DIEPINT0,
    _reserved12: [u8; 4usize],
    ///0x110 - device endpoint-0 transfer size register
    pub dieptsiz0: DIEPTSIZ0,
    _reserved13: [u8; 4usize],
    ///0x118 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts0: DTXFSTS0,
    _reserved14: [u8; 4usize],
    ///0x120 - OTG device endpoint-1 control register
    pub diepctl1: DIEPCTL1,
    _reserved15: [u8; 4usize],
    ///0x128 - device endpoint-1 interrupt register
    pub diepint1: DIEPINT1,
    _reserved16: [u8; 4usize],
    ///0x130 - device endpoint-1 transfer size register
    pub dieptsiz1: DIEPTSIZ1,
    _reserved17: [u8; 4usize],
    ///0x138 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts1: DTXFSTS1,
    _reserved18: [u8; 4usize],
    ///0x140 - OTG device endpoint-2 control register
    pub diepctl2: DIEPCTL2,
    _reserved19: [u8; 4usize],
    ///0x148 - device endpoint-2 interrupt register
    pub diepint2: DIEPINT2,
    _reserved20: [u8; 4usize],
    ///0x150 - device endpoint-2 transfer size register
    pub dieptsiz2: DIEPTSIZ2,
    _reserved21: [u8; 4usize],
    ///0x158 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts2: DTXFSTS2,
    _reserved22: [u8; 4usize],
    ///0x160 - OTG device endpoint-3 control register
    pub diepctl3: DIEPCTL3,
    _reserved23: [u8; 4usize],
    ///0x168 - device endpoint-3 interrupt register
    pub diepint3: DIEPINT3,
    _reserved24: [u8; 4usize],
    ///0x170 - device endpoint-3 transfer size register
    pub dieptsiz3: DIEPTSIZ3,
    _reserved25: [u8; 4usize],
    ///0x178 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts3: DTXFSTS3,
    _reserved26: [u8; 388usize],
    ///0x300 - device endpoint-0 control register
    pub doepctl0: DOEPCTL0,
    _reserved27: [u8; 4usize],
    ///0x308 - device endpoint-0 interrupt register
    pub doepint0: DOEPINT0,
    _reserved28: [u8; 4usize],
    ///0x310 - device OUT endpoint-0 transfer size register
    pub doeptsiz0: DOEPTSIZ0,
    _reserved29: [u8; 12usize],
    ///0x320 - device endpoint-1 control register
    pub doepctl1: DOEPCTL1,
    _reserved30: [u8; 4usize],
    ///0x328 - device endpoint-1 interrupt register
    pub doepint1: DOEPINT1,
    _reserved31: [u8; 4usize],
    ///0x330 - device OUT endpoint-1 transfer size register
    pub doeptsiz1: DOEPTSIZ1,
    _reserved32: [u8; 12usize],
    ///0x340 - device endpoint-2 control register
    pub doepctl2: DOEPCTL2,
    _reserved33: [u8; 4usize],
    ///0x348 - device endpoint-2 interrupt register
    pub doepint2: DOEPINT2,
    _reserved34: [u8; 4usize],
    ///0x350 - device OUT endpoint-2 transfer size register
    pub doeptsiz2: DOEPTSIZ2,
    _reserved35: [u8; 12usize],
    ///0x360 - device endpoint-3 control register
    pub doepctl3: DOEPCTL3,
    _reserved36: [u8; 4usize],
    ///0x368 - device endpoint-3 interrupt register
    pub doepint3: DOEPINT3,
    _reserved37: [u8; 4usize],
    ///0x370 - device OUT endpoint-3 transfer size register
    pub doeptsiz3: DOEPTSIZ3,
}
///OTG_FS device configuration register (OTG_FS_DCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_dcfg](fs_dcfg) module
pub type FS_DCFG = crate::Reg<u32, _FS_DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DCFG;
///`read()` method returns [fs_dcfg::R](fs_dcfg::R) reader structure
impl crate::Readable for FS_DCFG {}
///`write(|w| ..)` method takes [fs_dcfg::W](fs_dcfg::W) writer structure
impl crate::Writable for FS_DCFG {}
///OTG_FS device configuration register (OTG_FS_DCFG)
pub mod fs_dcfg;
///OTG_FS device control register (OTG_FS_DCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_dctl](fs_dctl) module
pub type FS_DCTL = crate::Reg<u32, _FS_DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DCTL;
///`read()` method returns [fs_dctl::R](fs_dctl::R) reader structure
impl crate::Readable for FS_DCTL {}
///`write(|w| ..)` method takes [fs_dctl::W](fs_dctl::W) writer structure
impl crate::Writable for FS_DCTL {}
///OTG_FS device control register (OTG_FS_DCTL)
pub mod fs_dctl;
///OTG_FS device status register (OTG_FS_DSTS)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_dsts](fs_dsts) module
pub type FS_DSTS = crate::Reg<u32, _FS_DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DSTS;
///`read()` method returns [fs_dsts::R](fs_dsts::R) reader structure
impl crate::Readable for FS_DSTS {}
///OTG_FS device status register (OTG_FS_DSTS)
pub mod fs_dsts;
///OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_diepmsk](fs_diepmsk) module
pub type FS_DIEPMSK = crate::Reg<u32, _FS_DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPMSK;
///`read()` method returns [fs_diepmsk::R](fs_diepmsk::R) reader structure
impl crate::Readable for FS_DIEPMSK {}
///`write(|w| ..)` method takes [fs_diepmsk::W](fs_diepmsk::W) writer structure
impl crate::Writable for FS_DIEPMSK {}
///OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
pub mod fs_diepmsk;
///OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_doepmsk](fs_doepmsk) module
pub type FS_DOEPMSK = crate::Reg<u32, _FS_DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DOEPMSK;
///`read()` method returns [fs_doepmsk::R](fs_doepmsk::R) reader structure
impl crate::Readable for FS_DOEPMSK {}
///`write(|w| ..)` method takes [fs_doepmsk::W](fs_doepmsk::W) writer structure
impl crate::Writable for FS_DOEPMSK {}
///OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
pub mod fs_doepmsk;
///OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_daint](fs_daint) module
pub type FS_DAINT = crate::Reg<u32, _FS_DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DAINT;
///`read()` method returns [fs_daint::R](fs_daint::R) reader structure
impl crate::Readable for FS_DAINT {}
///OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
pub mod fs_daint;
///OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_daintmsk](fs_daintmsk) module
pub type FS_DAINTMSK = crate::Reg<u32, _FS_DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DAINTMSK;
///`read()` method returns [fs_daintmsk::R](fs_daintmsk::R) reader structure
impl crate::Readable for FS_DAINTMSK {}
///`write(|w| ..)` method takes [fs_daintmsk::W](fs_daintmsk::W) writer structure
impl crate::Writable for FS_DAINTMSK {}
///OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
pub mod fs_daintmsk;
///OTG_FS device VBUS discharge time register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dvbusdis](dvbusdis) module
pub type DVBUSDIS = crate::Reg<u32, _DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSDIS;
///`read()` method returns [dvbusdis::R](dvbusdis::R) reader structure
impl crate::Readable for DVBUSDIS {}
///`write(|w| ..)` method takes [dvbusdis::W](dvbusdis::W) writer structure
impl crate::Writable for DVBUSDIS {}
///OTG_FS device VBUS discharge time register
pub mod dvbusdis;
///OTG_FS device VBUS pulsing time register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dvbuspulse](dvbuspulse) module
pub type DVBUSPULSE = crate::Reg<u32, _DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSPULSE;
///`read()` method returns [dvbuspulse::R](dvbuspulse::R) reader structure
impl crate::Readable for DVBUSPULSE {}
///`write(|w| ..)` method takes [dvbuspulse::W](dvbuspulse::W) writer structure
impl crate::Writable for DVBUSPULSE {}
///OTG_FS device VBUS pulsing time register
pub mod dvbuspulse;
///OTG_FS device IN endpoint FIFO empty interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepempmsk](diepempmsk) module
pub type DIEPEMPMSK = crate::Reg<u32, _DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEMPMSK;
///`read()` method returns [diepempmsk::R](diepempmsk::R) reader structure
impl crate::Readable for DIEPEMPMSK {}
///`write(|w| ..)` method takes [diepempmsk::W](diepempmsk::W) writer structure
impl crate::Writable for DIEPEMPMSK {}
///OTG_FS device IN endpoint FIFO empty interrupt mask register
pub mod diepempmsk;
///OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_diepctl0](fs_diepctl0) module
pub type FS_DIEPCTL0 = crate::Reg<u32, _FS_DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPCTL0;
///`read()` method returns [fs_diepctl0::R](fs_diepctl0::R) reader structure
impl crate::Readable for FS_DIEPCTL0 {}
///`write(|w| ..)` method takes [fs_diepctl0::W](fs_diepctl0::W) writer structure
impl crate::Writable for FS_DIEPCTL0 {}
///OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
pub mod fs_diepctl0;
///OTG device endpoint-1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepctl1](diepctl1) module
pub type DIEPCTL1 = crate::Reg<u32, _DIEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL1;
///`read()` method returns [diepctl1::R](diepctl1::R) reader structure
impl crate::Readable for DIEPCTL1 {}
///`write(|w| ..)` method takes [diepctl1::W](diepctl1::W) writer structure
impl crate::Writable for DIEPCTL1 {}
///OTG device endpoint-1 control register
pub mod diepctl1;
///OTG device endpoint-2 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepctl2](diepctl2) module
pub type DIEPCTL2 = crate::Reg<u32, _DIEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL2;
///`read()` method returns [diepctl2::R](diepctl2::R) reader structure
impl crate::Readable for DIEPCTL2 {}
///`write(|w| ..)` method takes [diepctl2::W](diepctl2::W) writer structure
impl crate::Writable for DIEPCTL2 {}
///OTG device endpoint-2 control register
pub mod diepctl2;
///OTG device endpoint-3 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepctl3](diepctl3) module
pub type DIEPCTL3 = crate::Reg<u32, _DIEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL3;
///`read()` method returns [diepctl3::R](diepctl3::R) reader structure
impl crate::Readable for DIEPCTL3 {}
///`write(|w| ..)` method takes [diepctl3::W](diepctl3::W) writer structure
impl crate::Writable for DIEPCTL3 {}
///OTG device endpoint-3 control register
pub mod diepctl3;
///device endpoint-0 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepctl0](doepctl0) module
pub type DOEPCTL0 = crate::Reg<u32, _DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL0;
///`read()` method returns [doepctl0::R](doepctl0::R) reader structure
impl crate::Readable for DOEPCTL0 {}
///`write(|w| ..)` method takes [doepctl0::W](doepctl0::W) writer structure
impl crate::Writable for DOEPCTL0 {}
///device endpoint-0 control register
pub mod doepctl0;
///device endpoint-1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepctl1](doepctl1) module
pub type DOEPCTL1 = crate::Reg<u32, _DOEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL1;
///`read()` method returns [doepctl1::R](doepctl1::R) reader structure
impl crate::Readable for DOEPCTL1 {}
///`write(|w| ..)` method takes [doepctl1::W](doepctl1::W) writer structure
impl crate::Writable for DOEPCTL1 {}
///device endpoint-1 control register
pub mod doepctl1;
///device endpoint-2 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepctl2](doepctl2) module
pub type DOEPCTL2 = crate::Reg<u32, _DOEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL2;
///`read()` method returns [doepctl2::R](doepctl2::R) reader structure
impl crate::Readable for DOEPCTL2 {}
///`write(|w| ..)` method takes [doepctl2::W](doepctl2::W) writer structure
impl crate::Writable for DOEPCTL2 {}
///device endpoint-2 control register
pub mod doepctl2;
///device endpoint-3 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepctl3](doepctl3) module
pub type DOEPCTL3 = crate::Reg<u32, _DOEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL3;
///`read()` method returns [doepctl3::R](doepctl3::R) reader structure
impl crate::Readable for DOEPCTL3 {}
///`write(|w| ..)` method takes [doepctl3::W](doepctl3::W) writer structure
impl crate::Writable for DOEPCTL3 {}
///device endpoint-3 control register
pub mod doepctl3;
///device endpoint-x interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepint0](diepint0) module
pub type DIEPINT0 = crate::Reg<u32, _DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT0;
///`read()` method returns [diepint0::R](diepint0::R) reader structure
impl crate::Readable for DIEPINT0 {}
///`write(|w| ..)` method takes [diepint0::W](diepint0::W) writer structure
impl crate::Writable for DIEPINT0 {}
///device endpoint-x interrupt register
pub mod diepint0;
///device endpoint-1 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepint1](diepint1) module
pub type DIEPINT1 = crate::Reg<u32, _DIEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT1;
///`read()` method returns [diepint1::R](diepint1::R) reader structure
impl crate::Readable for DIEPINT1 {}
///`write(|w| ..)` method takes [diepint1::W](diepint1::W) writer structure
impl crate::Writable for DIEPINT1 {}
///device endpoint-1 interrupt register
pub mod diepint1;
///device endpoint-2 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepint2](diepint2) module
pub type DIEPINT2 = crate::Reg<u32, _DIEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT2;
///`read()` method returns [diepint2::R](diepint2::R) reader structure
impl crate::Readable for DIEPINT2 {}
///`write(|w| ..)` method takes [diepint2::W](diepint2::W) writer structure
impl crate::Writable for DIEPINT2 {}
///device endpoint-2 interrupt register
pub mod diepint2;
///device endpoint-3 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepint3](diepint3) module
pub type DIEPINT3 = crate::Reg<u32, _DIEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINT3;
///`read()` method returns [diepint3::R](diepint3::R) reader structure
impl crate::Readable for DIEPINT3 {}
///`write(|w| ..)` method takes [diepint3::W](diepint3::W) writer structure
impl crate::Writable for DIEPINT3 {}
///device endpoint-3 interrupt register
pub mod diepint3;
///device endpoint-0 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepint0](doepint0) module
pub type DOEPINT0 = crate::Reg<u32, _DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT0;
///`read()` method returns [doepint0::R](doepint0::R) reader structure
impl crate::Readable for DOEPINT0 {}
///`write(|w| ..)` method takes [doepint0::W](doepint0::W) writer structure
impl crate::Writable for DOEPINT0 {}
///device endpoint-0 interrupt register
pub mod doepint0;
///device endpoint-1 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepint1](doepint1) module
pub type DOEPINT1 = crate::Reg<u32, _DOEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT1;
///`read()` method returns [doepint1::R](doepint1::R) reader structure
impl crate::Readable for DOEPINT1 {}
///`write(|w| ..)` method takes [doepint1::W](doepint1::W) writer structure
impl crate::Writable for DOEPINT1 {}
///device endpoint-1 interrupt register
pub mod doepint1;
///device endpoint-2 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepint2](doepint2) module
pub type DOEPINT2 = crate::Reg<u32, _DOEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT2;
///`read()` method returns [doepint2::R](doepint2::R) reader structure
impl crate::Readable for DOEPINT2 {}
///`write(|w| ..)` method takes [doepint2::W](doepint2::W) writer structure
impl crate::Writable for DOEPINT2 {}
///device endpoint-2 interrupt register
pub mod doepint2;
///device endpoint-3 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepint3](doepint3) module
pub type DOEPINT3 = crate::Reg<u32, _DOEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINT3;
///`read()` method returns [doepint3::R](doepint3::R) reader structure
impl crate::Readable for DOEPINT3 {}
///`write(|w| ..)` method takes [doepint3::W](doepint3::W) writer structure
impl crate::Writable for DOEPINT3 {}
///device endpoint-3 interrupt register
pub mod doepint3;
///device endpoint-0 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptsiz0](dieptsiz0) module
pub type DIEPTSIZ0 = crate::Reg<u32, _DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ0;
///`read()` method returns [dieptsiz0::R](dieptsiz0::R) reader structure
impl crate::Readable for DIEPTSIZ0 {}
///`write(|w| ..)` method takes [dieptsiz0::W](dieptsiz0::W) writer structure
impl crate::Writable for DIEPTSIZ0 {}
///device endpoint-0 transfer size register
pub mod dieptsiz0;
///device OUT endpoint-0 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doeptsiz0](doeptsiz0) module
pub type DOEPTSIZ0 = crate::Reg<u32, _DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ0;
///`read()` method returns [doeptsiz0::R](doeptsiz0::R) reader structure
impl crate::Readable for DOEPTSIZ0 {}
///`write(|w| ..)` method takes [doeptsiz0::W](doeptsiz0::W) writer structure
impl crate::Writable for DOEPTSIZ0 {}
///device OUT endpoint-0 transfer size register
pub mod doeptsiz0;
///device endpoint-1 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptsiz1](dieptsiz1) module
pub type DIEPTSIZ1 = crate::Reg<u32, _DIEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ1;
///`read()` method returns [dieptsiz1::R](dieptsiz1::R) reader structure
impl crate::Readable for DIEPTSIZ1 {}
///`write(|w| ..)` method takes [dieptsiz1::W](dieptsiz1::W) writer structure
impl crate::Writable for DIEPTSIZ1 {}
///device endpoint-1 transfer size register
pub mod dieptsiz1;
///device endpoint-2 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptsiz2](dieptsiz2) module
pub type DIEPTSIZ2 = crate::Reg<u32, _DIEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ2;
///`read()` method returns [dieptsiz2::R](dieptsiz2::R) reader structure
impl crate::Readable for DIEPTSIZ2 {}
///`write(|w| ..)` method takes [dieptsiz2::W](dieptsiz2::W) writer structure
impl crate::Writable for DIEPTSIZ2 {}
///device endpoint-2 transfer size register
pub mod dieptsiz2;
///device endpoint-3 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptsiz3](dieptsiz3) module
pub type DIEPTSIZ3 = crate::Reg<u32, _DIEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTSIZ3;
///`read()` method returns [dieptsiz3::R](dieptsiz3::R) reader structure
impl crate::Readable for DIEPTSIZ3 {}
///`write(|w| ..)` method takes [dieptsiz3::W](dieptsiz3::W) writer structure
impl crate::Writable for DIEPTSIZ3 {}
///device endpoint-3 transfer size register
pub mod dieptsiz3;
///OTG_FS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtxfsts0](dtxfsts0) module
pub type DTXFSTS0 = crate::Reg<u32, _DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS0;
///`read()` method returns [dtxfsts0::R](dtxfsts0::R) reader structure
impl crate::Readable for DTXFSTS0 {}
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts0;
///OTG_FS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtxfsts1](dtxfsts1) module
pub type DTXFSTS1 = crate::Reg<u32, _DTXFSTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS1;
///`read()` method returns [dtxfsts1::R](dtxfsts1::R) reader structure
impl crate::Readable for DTXFSTS1 {}
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts1;
///OTG_FS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtxfsts2](dtxfsts2) module
pub type DTXFSTS2 = crate::Reg<u32, _DTXFSTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS2;
///`read()` method returns [dtxfsts2::R](dtxfsts2::R) reader structure
impl crate::Readable for DTXFSTS2 {}
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts2;
///OTG_FS device IN endpoint transmit FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtxfsts3](dtxfsts3) module
pub type DTXFSTS3 = crate::Reg<u32, _DTXFSTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTXFSTS3;
///`read()` method returns [dtxfsts3::R](dtxfsts3::R) reader structure
impl crate::Readable for DTXFSTS3 {}
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts3;
///device OUT endpoint-1 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doeptsiz1](doeptsiz1) module
pub type DOEPTSIZ1 = crate::Reg<u32, _DOEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ1;
///`read()` method returns [doeptsiz1::R](doeptsiz1::R) reader structure
impl crate::Readable for DOEPTSIZ1 {}
///`write(|w| ..)` method takes [doeptsiz1::W](doeptsiz1::W) writer structure
impl crate::Writable for DOEPTSIZ1 {}
///device OUT endpoint-1 transfer size register
pub mod doeptsiz1;
///device OUT endpoint-2 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doeptsiz2](doeptsiz2) module
pub type DOEPTSIZ2 = crate::Reg<u32, _DOEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ2;
///`read()` method returns [doeptsiz2::R](doeptsiz2::R) reader structure
impl crate::Readable for DOEPTSIZ2 {}
///`write(|w| ..)` method takes [doeptsiz2::W](doeptsiz2::W) writer structure
impl crate::Writable for DOEPTSIZ2 {}
///device OUT endpoint-2 transfer size register
pub mod doeptsiz2;
///device OUT endpoint-3 transfer size register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doeptsiz3](doeptsiz3) module
pub type DOEPTSIZ3 = crate::Reg<u32, _DOEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPTSIZ3;
///`read()` method returns [doeptsiz3::R](doeptsiz3::R) reader structure
impl crate::Readable for DOEPTSIZ3 {}
///`write(|w| ..)` method takes [doeptsiz3::W](doeptsiz3::W) writer structure
impl crate::Writable for DOEPTSIZ3 {}
///device OUT endpoint-3 transfer size register
pub mod doeptsiz3;
