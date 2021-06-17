///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)
    pub gotgctl: GOTGCTL,
    ///0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)
    pub gotgint: GOTGINT,
    ///0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
    pub gahbcfg: GAHBCFG,
    ///0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)
    pub gusbcfg: GUSBCFG,
    ///0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)
    pub grstctl: GRSTCTL,
    ///0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)
    pub gintsts: GINTSTS,
    ///0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 4usize],
    _reserved_8_grxstsp: [u8; 4usize],
    ///0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
    pub grxfsiz: GRXFSIZ,
    _reserved_10_dieptxf0: [u8; 4usize],
    ///0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
    pub gnptxsts: GNPTXSTS,
    _reserved12: [u8; 8usize],
    ///0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)
    pub gccfg: GCCFG,
    ///0x3c - core ID register
    pub cid: CID,
    _reserved14: [u8; 192usize],
    ///0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
    pub hptxfsiz: HPTXFSIZ,
    ///0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    pub dieptxf1: DIEPTXF,
    ///0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    pub dieptxf2: DIEPTXF,
    ///0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    pub dieptxf3: DIEPTXF,
}
impl RegisterBlock {
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRXSTSR_HOST) }
    }
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub fn grxstsr_host_mut(&self) -> &mut GRXSTSR_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRXSTSR_HOST) }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRXSTSR_DEVICE) }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub fn grxstsr_device_mut(&self) -> &mut GRXSTSR_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRXSTSR_DEVICE) }
    }
    ///0x20 - OTG status read and pop (host mode)
    #[inline(always)]
    pub fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRXSTSP_HOST) }
    }
    ///0x20 - OTG status read and pop (host mode)
    #[inline(always)]
    pub fn grxstsp_host_mut(&self) -> &mut GRXSTSP_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRXSTSP_HOST) }
    }
    ///0x20 - OTG status read and pop (device mode)
    #[inline(always)]
    pub fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRXSTSP_DEVICE) }
    }
    ///0x20 - OTG status read and pop (device mode)
    #[inline(always)]
    pub fn grxstsp_device_mut(&self) -> &mut GRXSTSP_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRXSTSP_DEVICE) }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)
    #[inline(always)]
    pub fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const HNPTXFSIZ) }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)
    #[inline(always)]
    pub fn hnptxfsiz_mut(&self) -> &mut HNPTXFSIZ {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut HNPTXFSIZ) }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)
    #[inline(always)]
    pub fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const DIEPTXF0) }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)
    #[inline(always)]
    pub fn dieptxf0_mut(&self) -> &mut DIEPTXF0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut DIEPTXF0) }
    }
}
///OTG_FS control and status register (OTG_FS_GOTGCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gotgctl](gotgctl) module
pub type GOTGCTL = crate::Reg<u32, _GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGCTL;
///`read()` method returns [gotgctl::R](gotgctl::R) reader structure
impl crate::Readable for GOTGCTL {}
///`write(|w| ..)` method takes [gotgctl::W](gotgctl::W) writer structure
impl crate::Writable for GOTGCTL {}
///OTG_FS control and status register (OTG_FS_GOTGCTL)
pub mod gotgctl;
///OTG_FS interrupt register (OTG_FS_GOTGINT)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gotgint](gotgint) module
pub type GOTGINT = crate::Reg<u32, _GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGINT;
///`read()` method returns [gotgint::R](gotgint::R) reader structure
impl crate::Readable for GOTGINT {}
///`write(|w| ..)` method takes [gotgint::W](gotgint::W) writer structure
impl crate::Writable for GOTGINT {}
///OTG_FS interrupt register (OTG_FS_GOTGINT)
pub mod gotgint;
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gahbcfg](gahbcfg) module
pub type GAHBCFG = crate::Reg<u32, _GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAHBCFG;
///`read()` method returns [gahbcfg::R](gahbcfg::R) reader structure
impl crate::Readable for GAHBCFG {}
///`write(|w| ..)` method takes [gahbcfg::W](gahbcfg::W) writer structure
impl crate::Writable for GAHBCFG {}
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
pub mod gahbcfg;
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gusbcfg](gusbcfg) module
pub type GUSBCFG = crate::Reg<u32, _GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUSBCFG;
///`read()` method returns [gusbcfg::R](gusbcfg::R) reader structure
impl crate::Readable for GUSBCFG {}
///`write(|w| ..)` method takes [gusbcfg::W](gusbcfg::W) writer structure
impl crate::Writable for GUSBCFG {}
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
pub mod gusbcfg;
///OTG_FS reset register (OTG_FS_GRSTCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grstctl](grstctl) module
pub type GRSTCTL = crate::Reg<u32, _GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTCTL;
///`read()` method returns [grstctl::R](grstctl::R) reader structure
impl crate::Readable for GRSTCTL {}
///`write(|w| ..)` method takes [grstctl::W](grstctl::W) writer structure
impl crate::Writable for GRSTCTL {}
///OTG_FS reset register (OTG_FS_GRSTCTL)
pub mod grstctl;
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gintsts](gintsts) module
pub type GINTSTS = crate::Reg<u32, _GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTSTS;
///`read()` method returns [gintsts::R](gintsts::R) reader structure
impl crate::Readable for GINTSTS {}
///`write(|w| ..)` method takes [gintsts::W](gintsts::W) writer structure
impl crate::Writable for GINTSTS {}
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
pub mod gintsts;
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gintmsk](gintmsk) module
pub type GINTMSK = crate::Reg<u32, _GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTMSK;
///`read()` method returns [gintmsk::R](gintmsk::R) reader structure
impl crate::Readable for GINTMSK {}
///`write(|w| ..)` method takes [gintmsk::W](gintmsk::W) writer structure
impl crate::Writable for GINTMSK {}
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
pub mod gintmsk;
///OTG_FS Receive status debug read(Device mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxstsr_device](grxstsr_device) module
pub type GRXSTSR_DEVICE = crate::Reg<u32, _GRXSTSR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR_DEVICE;
///`read()` method returns [grxstsr_device::R](grxstsr_device::R) reader structure
impl crate::Readable for GRXSTSR_DEVICE {}
///OTG_FS Receive status debug read(Device mode)
pub mod grxstsr_device;
///OTG_FS Receive status debug read(Host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxstsr_host](grxstsr_host) module
pub type GRXSTSR_HOST = crate::Reg<u32, _GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR_HOST;
///`read()` method returns [grxstsr_host::R](grxstsr_host::R) reader structure
impl crate::Readable for GRXSTSR_HOST {}
///OTG_FS Receive status debug read(Host mode)
pub mod grxstsr_host;
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxfsiz](grxfsiz) module
pub type GRXFSIZ = crate::Reg<u32, _GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXFSIZ;
///`read()` method returns [grxfsiz::R](grxfsiz::R) reader structure
impl crate::Readable for GRXFSIZ {}
///`write(|w| ..)` method takes [grxfsiz::W](grxfsiz::W) writer structure
impl crate::Writable for GRXFSIZ {}
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
pub mod grxfsiz;
///OTG_FS non-periodic transmit FIFO size register (Device mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptxf0](dieptxf0) module
pub type DIEPTXF0 = crate::Reg<u32, _DIEPTXF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF0;
///`read()` method returns [dieptxf0::R](dieptxf0::R) reader structure
impl crate::Readable for DIEPTXF0 {}
///`write(|w| ..)` method takes [dieptxf0::W](dieptxf0::W) writer structure
impl crate::Writable for DIEPTXF0 {}
///OTG_FS non-periodic transmit FIFO size register (Device mode)
pub mod dieptxf0;
///OTG_FS non-periodic transmit FIFO size register (Host mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hnptxfsiz](hnptxfsiz) module
pub type HNPTXFSIZ = crate::Reg<u32, _HNPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HNPTXFSIZ;
///`read()` method returns [hnptxfsiz::R](hnptxfsiz::R) reader structure
impl crate::Readable for HNPTXFSIZ {}
///`write(|w| ..)` method takes [hnptxfsiz::W](hnptxfsiz::W) writer structure
impl crate::Writable for HNPTXFSIZ {}
///OTG_FS non-periodic transmit FIFO size register (Host mode)
pub mod hnptxfsiz;
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gnptxsts](gnptxsts) module
pub type GNPTXSTS = crate::Reg<u32, _GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXSTS;
///`read()` method returns [gnptxsts::R](gnptxsts::R) reader structure
impl crate::Readable for GNPTXSTS {}
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
pub mod gnptxsts;
///OTG_FS general core configuration register (OTG_FS_GCCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gccfg](gccfg) module
pub type GCCFG = crate::Reg<u32, _GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCCFG;
///`read()` method returns [gccfg::R](gccfg::R) reader structure
impl crate::Readable for GCCFG {}
///`write(|w| ..)` method takes [gccfg::W](gccfg::W) writer structure
impl crate::Writable for GCCFG {}
///OTG_FS general core configuration register (OTG_FS_GCCFG)
pub mod gccfg;
///core ID register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cid](cid) module
pub type CID = crate::Reg<u32, _CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID;
///`read()` method returns [cid::R](cid::R) reader structure
impl crate::Readable for CID {}
///`write(|w| ..)` method takes [cid::W](cid::W) writer structure
impl crate::Writable for CID {}
///core ID register
pub mod cid;
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hptxfsiz](hptxfsiz) module
pub type HPTXFSIZ = crate::Reg<u32, _HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXFSIZ;
///`read()` method returns [hptxfsiz::R](hptxfsiz::R) reader structure
impl crate::Readable for HPTXFSIZ {}
///`write(|w| ..)` method takes [hptxfsiz::W](hptxfsiz::W) writer structure
impl crate::Writable for HPTXFSIZ {}
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
pub mod hptxfsiz;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dieptxf](dieptxf) module
pub type DIEPTXF = crate::Reg<u32, _DIEPTXF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF;
///`read()` method returns [dieptxf::R](dieptxf::R) reader structure
impl crate::Readable for DIEPTXF {}
///`write(|w| ..)` method takes [dieptxf::W](dieptxf::W) writer structure
impl crate::Writable for DIEPTXF {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
pub mod dieptxf;
///OTG status read and pop (device mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxstsp_device](grxstsp_device) module
pub type GRXSTSP_DEVICE = crate::Reg<u32, _GRXSTSP_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP_DEVICE;
///`read()` method returns [grxstsp_device::R](grxstsp_device::R) reader structure
impl crate::Readable for GRXSTSP_DEVICE {}
///OTG status read and pop (device mode)
pub mod grxstsp_device;
///OTG status read and pop (host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [grxstsp_host](grxstsp_host) module
pub type GRXSTSP_HOST = crate::Reg<u32, _GRXSTSP_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP_HOST;
///`read()` method returns [grxstsp_host::R](grxstsp_host::R) reader structure
impl crate::Readable for GRXSTSP_HOST {}
///OTG status read and pop (host mode)
pub mod grxstsp_host;
