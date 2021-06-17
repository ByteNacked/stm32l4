///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS device configuration register (OTG_FS_DCFG)
    pub dcfg: DCFG,
    ///0x04 - OTG_FS device control register (OTG_FS_DCTL)
    pub dctl: DCTL,
    ///0x08 - OTG_FS device status register (OTG_FS_DSTS)
    pub dsts: DSTS,
    _reserved3: [u8; 4usize],
    ///0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
    pub diepmsk: DIEPMSK,
    ///0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
    pub doepmsk: DOEPMSK,
    ///0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
    pub daint: DAINT,
    ///0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
    pub daintmsk: DAINTMSK,
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
    pub diepctl0: DIEPCTL0,
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
    pub diepctl1: DIEPCTL,
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
    ///0x140 - OTG device endpoint-1 control register
    pub diepctl2: DIEPCTL,
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
    ///0x160 - OTG device endpoint-1 control register
    pub diepctl3: DIEPCTL,
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
    pub doepctl1: DOEPCTL,
    _reserved30: [u8; 4usize],
    ///0x328 - device endpoint-1 interrupt register
    pub doepint1: DOEPINT1,
    _reserved31: [u8; 4usize],
    ///0x330 - device OUT endpoint-1 transfer size register
    pub doeptsiz1: DOEPTSIZ1,
    _reserved32: [u8; 12usize],
    ///0x340 - device endpoint-1 control register
    pub doepctl2: DOEPCTL,
    _reserved33: [u8; 4usize],
    ///0x348 - device endpoint-2 interrupt register
    pub doepint2: DOEPINT2,
    _reserved34: [u8; 4usize],
    ///0x350 - device OUT endpoint-2 transfer size register
    pub doeptsiz2: DOEPTSIZ2,
    _reserved35: [u8; 12usize],
    ///0x360 - device endpoint-1 control register
    pub doepctl3: DOEPCTL,
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
///For information about available fields see [dcfg](dcfg) module
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
///`read()` method returns [dcfg::R](dcfg::R) reader structure
impl crate::Readable for DCFG {}
///`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure
impl crate::Writable for DCFG {}
///OTG_FS device configuration register (OTG_FS_DCFG)
pub mod dcfg;
///OTG_FS device control register (OTG_FS_DCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dctl](dctl) module
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
///`read()` method returns [dctl::R](dctl::R) reader structure
impl crate::Readable for DCTL {}
///`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure
impl crate::Writable for DCTL {}
///OTG_FS device control register (OTG_FS_DCTL)
pub mod dctl;
///OTG_FS device status register (OTG_FS_DSTS)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsts](dsts) module
pub type DSTS = crate::Reg<u32, _DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTS;
///`read()` method returns [dsts::R](dsts::R) reader structure
impl crate::Readable for DSTS {}
///OTG_FS device status register (OTG_FS_DSTS)
pub mod dsts;
///OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepmsk](diepmsk) module
pub type DIEPMSK = crate::Reg<u32, _DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPMSK;
///`read()` method returns [diepmsk::R](diepmsk::R) reader structure
impl crate::Readable for DIEPMSK {}
///`write(|w| ..)` method takes [diepmsk::W](diepmsk::W) writer structure
impl crate::Writable for DIEPMSK {}
///OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
pub mod diepmsk;
///OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepmsk](doepmsk) module
pub type DOEPMSK = crate::Reg<u32, _DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPMSK;
///`read()` method returns [doepmsk::R](doepmsk::R) reader structure
impl crate::Readable for DOEPMSK {}
///`write(|w| ..)` method takes [doepmsk::W](doepmsk::W) writer structure
impl crate::Writable for DOEPMSK {}
///OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
pub mod doepmsk;
///OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daint](daint) module
pub type DAINT = crate::Reg<u32, _DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINT;
///`read()` method returns [daint::R](daint::R) reader structure
impl crate::Readable for DAINT {}
///OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
pub mod daint;
///OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daintmsk](daintmsk) module
pub type DAINTMSK = crate::Reg<u32, _DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINTMSK;
///`read()` method returns [daintmsk::R](daintmsk::R) reader structure
impl crate::Readable for DAINTMSK {}
///`write(|w| ..)` method takes [daintmsk::W](daintmsk::W) writer structure
impl crate::Writable for DAINTMSK {}
///OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
pub mod daintmsk;
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
///For information about available fields see [diepctl0](diepctl0) module
pub type DIEPCTL0 = crate::Reg<u32, _DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL0;
///`read()` method returns [diepctl0::R](diepctl0::R) reader structure
impl crate::Readable for DIEPCTL0 {}
///`write(|w| ..)` method takes [diepctl0::W](diepctl0::W) writer structure
impl crate::Writable for DIEPCTL0 {}
///OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
pub mod diepctl0;
///OTG device endpoint-1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepctl](diepctl) module
pub type DIEPCTL = crate::Reg<u32, _DIEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPCTL;
///`read()` method returns [diepctl::R](diepctl::R) reader structure
impl crate::Readable for DIEPCTL {}
///`write(|w| ..)` method takes [diepctl::W](diepctl::W) writer structure
impl crate::Writable for DIEPCTL {}
///OTG device endpoint-1 control register
pub mod diepctl;
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
///For information about available fields see [doepctl](doepctl) module
pub type DOEPCTL = crate::Reg<u32, _DOEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPCTL;
///`read()` method returns [doepctl::R](doepctl::R) reader structure
impl crate::Readable for DOEPCTL {}
///`write(|w| ..)` method takes [doepctl::W](doepctl::W) writer structure
impl crate::Writable for DOEPCTL {}
///device endpoint-1 control register
pub mod doepctl;
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
