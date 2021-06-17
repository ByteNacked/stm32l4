///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)
    pub fs_gotgctl: FS_GOTGCTL,
    ///0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)
    pub fs_gotgint: FS_GOTGINT,
    ///0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
    pub fs_gahbcfg: FS_GAHBCFG,
    ///0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)
    pub fs_gusbcfg: FS_GUSBCFG,
    ///0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)
    pub fs_grstctl: FS_GRSTCTL,
    ///0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)
    pub fs_gintsts: FS_GINTSTS,
    ///0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)
    pub fs_gintmsk: FS_GINTMSK,
    _reserved_7_fs_grxstsr: [u8; 4usize],
    _reserved8: [u8; 4usize],
    ///0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
    pub fs_grxfsiz: FS_GRXFSIZ,
    _reserved_9_fs_gnptxfsiz: [u8; 4usize],
    ///0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
    pub fs_gnptxsts: FS_GNPTXSTS,
    _reserved11: [u8; 8usize],
    ///0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)
    pub fs_gccfg: FS_GCCFG,
    ///0x3c - core ID register
    pub fs_cid: FS_CID,
    _reserved13: [u8; 192usize],
    ///0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
    pub fs_hptxfsiz: FS_HPTXFSIZ,
    ///0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    pub fs_dieptxf1: FS_DIEPTXF1,
    ///0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
    pub fs_dieptxf2: FS_DIEPTXF2,
    ///0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
    pub fs_dieptxf3: FS_DIEPTXF3,
}
impl RegisterBlock {
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub fn fs_grxstsr_host(&self) -> &FS_GRXSTSR_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const FS_GRXSTSR_HOST) }
    }
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub fn fs_grxstsr_host_mut(&self) -> &mut FS_GRXSTSR_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut FS_GRXSTSR_HOST) }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub fn fs_grxstsr_device(&self) -> &FS_GRXSTSR_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const FS_GRXSTSR_DEVICE) }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub fn fs_grxstsr_device_mut(&self) -> &mut FS_GRXSTSR_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut FS_GRXSTSR_DEVICE) }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)
    #[inline(always)]
    pub fn fs_gnptxfsiz_host(&self) -> &FS_GNPTXFSIZ_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const FS_GNPTXFSIZ_HOST) }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)
    #[inline(always)]
    pub fn fs_gnptxfsiz_host_mut(&self) -> &mut FS_GNPTXFSIZ_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut FS_GNPTXFSIZ_HOST) }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)
    #[inline(always)]
    pub fn fs_gnptxfsiz_device(&self) -> &FS_GNPTXFSIZ_DEVICE {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize) as *const FS_GNPTXFSIZ_DEVICE)
        }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)
    #[inline(always)]
    pub fn fs_gnptxfsiz_device_mut(&self) -> &mut FS_GNPTXFSIZ_DEVICE {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut FS_GNPTXFSIZ_DEVICE)
        }
    }
}
///OTG_FS control and status register (OTG_FS_GOTGCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gotgctl](fs_gotgctl) module
pub type FS_GOTGCTL = crate::Reg<u32, _FS_GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GOTGCTL;
///`read()` method returns [fs_gotgctl::R](fs_gotgctl::R) reader structure
impl crate::Readable for FS_GOTGCTL {}
///`write(|w| ..)` method takes [fs_gotgctl::W](fs_gotgctl::W) writer structure
impl crate::Writable for FS_GOTGCTL {}
///OTG_FS control and status register (OTG_FS_GOTGCTL)
pub mod fs_gotgctl;
///OTG_FS interrupt register (OTG_FS_GOTGINT)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gotgint](fs_gotgint) module
pub type FS_GOTGINT = crate::Reg<u32, _FS_GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GOTGINT;
///`read()` method returns [fs_gotgint::R](fs_gotgint::R) reader structure
impl crate::Readable for FS_GOTGINT {}
///`write(|w| ..)` method takes [fs_gotgint::W](fs_gotgint::W) writer structure
impl crate::Writable for FS_GOTGINT {}
///OTG_FS interrupt register (OTG_FS_GOTGINT)
pub mod fs_gotgint;
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gahbcfg](fs_gahbcfg) module
pub type FS_GAHBCFG = crate::Reg<u32, _FS_GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GAHBCFG;
///`read()` method returns [fs_gahbcfg::R](fs_gahbcfg::R) reader structure
impl crate::Readable for FS_GAHBCFG {}
///`write(|w| ..)` method takes [fs_gahbcfg::W](fs_gahbcfg::W) writer structure
impl crate::Writable for FS_GAHBCFG {}
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
pub mod fs_gahbcfg;
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gusbcfg](fs_gusbcfg) module
pub type FS_GUSBCFG = crate::Reg<u32, _FS_GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GUSBCFG;
///`read()` method returns [fs_gusbcfg::R](fs_gusbcfg::R) reader structure
impl crate::Readable for FS_GUSBCFG {}
///`write(|w| ..)` method takes [fs_gusbcfg::W](fs_gusbcfg::W) writer structure
impl crate::Writable for FS_GUSBCFG {}
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
pub mod fs_gusbcfg;
///OTG_FS reset register (OTG_FS_GRSTCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_grstctl](fs_grstctl) module
pub type FS_GRSTCTL = crate::Reg<u32, _FS_GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRSTCTL;
///`read()` method returns [fs_grstctl::R](fs_grstctl::R) reader structure
impl crate::Readable for FS_GRSTCTL {}
///`write(|w| ..)` method takes [fs_grstctl::W](fs_grstctl::W) writer structure
impl crate::Writable for FS_GRSTCTL {}
///OTG_FS reset register (OTG_FS_GRSTCTL)
pub mod fs_grstctl;
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gintsts](fs_gintsts) module
pub type FS_GINTSTS = crate::Reg<u32, _FS_GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GINTSTS;
///`read()` method returns [fs_gintsts::R](fs_gintsts::R) reader structure
impl crate::Readable for FS_GINTSTS {}
///`write(|w| ..)` method takes [fs_gintsts::W](fs_gintsts::W) writer structure
impl crate::Writable for FS_GINTSTS {}
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
pub mod fs_gintsts;
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gintmsk](fs_gintmsk) module
pub type FS_GINTMSK = crate::Reg<u32, _FS_GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GINTMSK;
///`read()` method returns [fs_gintmsk::R](fs_gintmsk::R) reader structure
impl crate::Readable for FS_GINTMSK {}
///`write(|w| ..)` method takes [fs_gintmsk::W](fs_gintmsk::W) writer structure
impl crate::Writable for FS_GINTMSK {}
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
pub mod fs_gintmsk;
///OTG_FS Receive status debug read(Device mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_grxstsr_device](fs_grxstsr_device) module
pub type FS_GRXSTSR_DEVICE = crate::Reg<u32, _FS_GRXSTSR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRXSTSR_DEVICE;
///`read()` method returns [fs_grxstsr_device::R](fs_grxstsr_device::R) reader structure
impl crate::Readable for FS_GRXSTSR_DEVICE {}
///OTG_FS Receive status debug read(Device mode)
pub mod fs_grxstsr_device;
///OTG_FS Receive status debug read(Host mode)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_grxstsr_host](fs_grxstsr_host) module
pub type FS_GRXSTSR_HOST = crate::Reg<u32, _FS_GRXSTSR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRXSTSR_HOST;
///`read()` method returns [fs_grxstsr_host::R](fs_grxstsr_host::R) reader structure
impl crate::Readable for FS_GRXSTSR_HOST {}
///OTG_FS Receive status debug read(Host mode)
pub mod fs_grxstsr_host;
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_grxfsiz](fs_grxfsiz) module
pub type FS_GRXFSIZ = crate::Reg<u32, _FS_GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GRXFSIZ;
///`read()` method returns [fs_grxfsiz::R](fs_grxfsiz::R) reader structure
impl crate::Readable for FS_GRXFSIZ {}
///`write(|w| ..)` method takes [fs_grxfsiz::W](fs_grxfsiz::W) writer structure
impl crate::Writable for FS_GRXFSIZ {}
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
pub mod fs_grxfsiz;
///OTG_FS non-periodic transmit FIFO size register (Device mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gnptxfsiz_device](fs_gnptxfsiz_device) module
pub type FS_GNPTXFSIZ_DEVICE = crate::Reg<u32, _FS_GNPTXFSIZ_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GNPTXFSIZ_DEVICE;
///`read()` method returns [fs_gnptxfsiz_device::R](fs_gnptxfsiz_device::R) reader structure
impl crate::Readable for FS_GNPTXFSIZ_DEVICE {}
///`write(|w| ..)` method takes [fs_gnptxfsiz_device::W](fs_gnptxfsiz_device::W) writer structure
impl crate::Writable for FS_GNPTXFSIZ_DEVICE {}
///OTG_FS non-periodic transmit FIFO size register (Device mode)
pub mod fs_gnptxfsiz_device;
///OTG_FS non-periodic transmit FIFO size register (Host mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gnptxfsiz_host](fs_gnptxfsiz_host) module
pub type FS_GNPTXFSIZ_HOST = crate::Reg<u32, _FS_GNPTXFSIZ_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GNPTXFSIZ_HOST;
///`read()` method returns [fs_gnptxfsiz_host::R](fs_gnptxfsiz_host::R) reader structure
impl crate::Readable for FS_GNPTXFSIZ_HOST {}
///`write(|w| ..)` method takes [fs_gnptxfsiz_host::W](fs_gnptxfsiz_host::W) writer structure
impl crate::Writable for FS_GNPTXFSIZ_HOST {}
///OTG_FS non-periodic transmit FIFO size register (Host mode)
pub mod fs_gnptxfsiz_host;
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gnptxsts](fs_gnptxsts) module
pub type FS_GNPTXSTS = crate::Reg<u32, _FS_GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GNPTXSTS;
///`read()` method returns [fs_gnptxsts::R](fs_gnptxsts::R) reader structure
impl crate::Readable for FS_GNPTXSTS {}
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
pub mod fs_gnptxsts;
///OTG_FS general core configuration register (OTG_FS_GCCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_gccfg](fs_gccfg) module
pub type FS_GCCFG = crate::Reg<u32, _FS_GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_GCCFG;
///`read()` method returns [fs_gccfg::R](fs_gccfg::R) reader structure
impl crate::Readable for FS_GCCFG {}
///`write(|w| ..)` method takes [fs_gccfg::W](fs_gccfg::W) writer structure
impl crate::Writable for FS_GCCFG {}
///OTG_FS general core configuration register (OTG_FS_GCCFG)
pub mod fs_gccfg;
///core ID register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_cid](fs_cid) module
pub type FS_CID = crate::Reg<u32, _FS_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_CID;
///`read()` method returns [fs_cid::R](fs_cid::R) reader structure
impl crate::Readable for FS_CID {}
///`write(|w| ..)` method takes [fs_cid::W](fs_cid::W) writer structure
impl crate::Writable for FS_CID {}
///core ID register
pub mod fs_cid;
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_hptxfsiz](fs_hptxfsiz) module
pub type FS_HPTXFSIZ = crate::Reg<u32, _FS_HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_HPTXFSIZ;
///`read()` method returns [fs_hptxfsiz::R](fs_hptxfsiz::R) reader structure
impl crate::Readable for FS_HPTXFSIZ {}
///`write(|w| ..)` method takes [fs_hptxfsiz::W](fs_hptxfsiz::W) writer structure
impl crate::Writable for FS_HPTXFSIZ {}
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
pub mod fs_hptxfsiz;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_dieptxf1](fs_dieptxf1) module
pub type FS_DIEPTXF1 = crate::Reg<u32, _FS_DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPTXF1;
///`read()` method returns [fs_dieptxf1::R](fs_dieptxf1::R) reader structure
impl crate::Readable for FS_DIEPTXF1 {}
///`write(|w| ..)` method takes [fs_dieptxf1::W](fs_dieptxf1::W) writer structure
impl crate::Writable for FS_DIEPTXF1 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
pub mod fs_dieptxf1;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_dieptxf2](fs_dieptxf2) module
pub type FS_DIEPTXF2 = crate::Reg<u32, _FS_DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPTXF2;
///`read()` method returns [fs_dieptxf2::R](fs_dieptxf2::R) reader structure
impl crate::Readable for FS_DIEPTXF2 {}
///`write(|w| ..)` method takes [fs_dieptxf2::W](fs_dieptxf2::W) writer structure
impl crate::Writable for FS_DIEPTXF2 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
pub mod fs_dieptxf2;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fs_dieptxf3](fs_dieptxf3) module
pub type FS_DIEPTXF3 = crate::Reg<u32, _FS_DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_DIEPTXF3;
///`read()` method returns [fs_dieptxf3::R](fs_dieptxf3::R) reader structure
impl crate::Readable for FS_DIEPTXF3 {}
///`write(|w| ..)` method takes [fs_dieptxf3::W](fs_dieptxf3::W) writer structure
impl crate::Writable for FS_DIEPTXF3 {}
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
pub mod fs_dieptxf3;
