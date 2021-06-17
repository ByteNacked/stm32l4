///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DSI Host Version Register
    pub dsi_vr: DSI_VR,
    ///0x04 - DSI Host Control Register
    pub dsi_cr: DSI_CR,
    ///0x08 - DSI HOST Clock Control Register
    pub dsi_ccr: DSI_CCR,
    ///0x0c - DSI Host LTDC VCID Register
    pub dsi_lvcidr: DSI_LVCIDR,
    ///0x10 - DSI Host LTDC Color Coding Register
    pub dsi_lcolcr: DSI_LCOLCR,
    ///0x14 - DSI Host LTDC Polarity Configuration Register
    pub dsi_lpcr: DSI_LPCR,
    ///0x18 - DSI Host Low-Power mode Configuration Register
    pub dsi_lpmcr: DSI_LPMCR,
    _reserved7: [u8; 16usize],
    ///0x2c - DSI Host Protocol Configuration Register
    pub dsi_pcr: DSI_PCR,
    ///0x30 - DSI Host Generic VCID Register
    pub dsi_gvcidr: DSI_GVCIDR,
    ///0x34 - DSI Host mode Configuration Register
    pub dsi_mcr: DSI_MCR,
    ///0x38 - DSI Host Video mode Configuration Register
    pub dsi_vmcr: DSI_VMCR,
    ///0x3c - DSI Host Video Packet Configuration Register
    pub dsi_vpcr: DSI_VPCR,
    ///0x40 - DSI Host Video Chunks Configuration Register
    pub dsi_vccr: DSI_VCCR,
    ///0x44 - DSI Host Video Null Packet Configuration Register
    pub dsi_vnpcr: DSI_VNPCR,
    ///0x48 - DSI Host Video HSA Configuration Register
    pub dsi_vhsacr: DSI_VHSACR,
    ///0x4c - DSI Host Video HBP Configuration Register
    pub dsi_vhbpcr: DSI_VHBPCR,
    ///0x50 - DSI Host Video Line Configuration Register
    pub dsi_vlcr: DSI_VLCR,
    ///0x54 - DSI Host Video VSA Configuration Register
    pub dsi_vvsacr: DSI_VVSACR,
    ///0x58 - DSI Host Video VBP Configuration Register
    pub dsi_vvbpcr: DSI_VVBPCR,
    ///0x5c - DSI Host Video VFP Configuration Register
    pub dsi_vvfpcr: DSI_VVFPCR,
    ///0x60 - DSI Host Video VA Configuration Register
    pub dsi_vvacr: DSI_VVACR,
    ///0x64 - DSI Host LTDC Command Configuration Register
    pub dsi_lccr: DSI_LCCR,
    ///0x68 - DSI Host Command mode Configuration Register
    pub dsi_cmcr: DSI_CMCR,
    ///0x6c - DSI Host Generic Header Configuration Register
    pub dsi_ghcr: DSI_GHCR,
    ///0x70 - DSI Host Generic Payload Data Register
    pub dsi_gpdr: DSI_GPDR,
    ///0x74 - DSI Host Generic Packet Status Register
    pub dsi_gpsr: DSI_GPSR,
    ///0x78 - DSI Host Timeout Counter Configuration Register 0
    pub dsi_tccr0: DSI_TCCR0,
    ///0x7c - DSI Host Timeout Counter Configuration Register 1
    pub dsi_tccr1: DSI_TCCR1,
    ///0x80 - DSI Host Timeout Counter Configuration Register 2
    pub dsi_tccr2: DSI_TCCR2,
    ///0x84 - DSI Host Timeout Counter Configuration Register 3
    pub dsi_tccr3: DSI_TCCR3,
    ///0x88 - DSI Host Timeout Counter Configuration Register 4
    pub dsi_tccr4: DSI_TCCR4,
    ///0x8c - DSI Host Timeout Counter Configuration Register 5
    pub dsi_tccr5: DSI_TCCR5,
    _reserved32: [u8; 4usize],
    ///0x94 - DSI Host Clock Lane Configuration Register
    pub dsi_clcr: DSI_CLCR,
    ///0x98 - DSI Host Clock Lane Timer Configuration Register
    pub dsi_cltcr: DSI_CLTCR,
    ///0x9c - DSI Host Data Lane Timer Configuration Register
    pub dsi_dltrc: DSI_DLTRC,
    ///0xa0 - DSI Host PHY Control Register
    pub dsi_pctlr: DSI_PCTLR,
    ///0xa4 - DSI Host PHY Configuration Register
    pub dsi_pconfr: DSI_PCONFR,
    ///0xa8 - DSI Host PHY ULPS Control Register
    pub dsi_pucr: DSI_PUCR,
    ///0xac - DSI Host PHY TX Triggers Configuration Register
    pub dsi_pttcr: DSI_PTTCR,
    ///0xb0 - DSI Host PHY Status Register
    pub dsi_psr: DSI_PSR,
    _reserved40: [u8; 8usize],
    ///0xbc - DSI Host Interrupt & Status Register 0
    pub dsi_isr0: DSI_ISR0,
    ///0xc0 - DSI Host Interrupt & Status Register 1
    pub dsi_isr1: DSI_ISR1,
    ///0xc4 - DSI Host Interrupt Enable Register 0
    pub dsi_ier0: DSI_IER0,
    ///0xc8 - DSI Host Interrupt Enable Register 1
    pub dsi_ier1: DSI_IER1,
    _reserved44: [u8; 12usize],
    ///0xd8 - DSI Host Force Interrupt Register 0
    pub dsi_fir0: DSI_FIR0,
    ///0xdc - DSI Host Force Interrupt Register 1
    pub dsi_fir1: DSI_FIR1,
    _reserved46: [u8; 32usize],
    ///0x100 - DSI Host Video Shadow Control Register
    pub dsi_vscr: DSI_VSCR,
    _reserved47: [u8; 8usize],
    ///0x10c - DSI Host LTDC Current VCID Register
    pub dsi_lcvcidr: DSI_LCVCIDR,
    ///0x110 - DSI Host LTDC Current Color Coding Register
    pub dsi_lcccr: DSI_LCCCR,
    _reserved49: [u8; 4usize],
    ///0x118 - DSI Host Low-Power mode Current Configuration Register
    pub dsi_lpmccr: DSI_LPMCCR,
    _reserved50: [u8; 28usize],
    ///0x138 - DSI Host Video mode Current Configuration Register
    pub dsi_vmccr: DSI_VMCCR,
    ///0x13c - DSI Host Video Packet Current Configuration Register
    pub dsi_vpccr: DSI_VPCCR,
    ///0x140 - DSI Host Video Chunks Current Configuration Register
    pub dsi_vcccr: DSI_VCCCR,
    ///0x144 - DSI Host Video Null Packet Current Configuration Register
    pub dsi_vnpccr: DSI_VNPCCR,
    ///0x148 - DSI Host Video HSA Current Configuration Register
    pub dsi_vhsaccr: DSI_VHSACCR,
    ///0x14c - DSI Host Video HBP Current Configuration Register
    pub dsi_vhbpccr: DSI_VHBPCCR,
    ///0x150 - DSI Host Video Line Current Configuration Register
    pub dsi_vlccr: DSI_VLCCR,
    ///0x154 - DSI Host Video VSA Current Configuration Register
    pub dsi_vvsaccr: DSI_VVSACCR,
    ///0x158 - DSI Host Video VBP Current Configuration Register
    pub dsi_vvbpccr: DSI_VVBPCCR,
    ///0x15c - DSI Host Video VFP Current Configuration Register
    pub dsi_vvfpccr: DSI_VVFPCCR,
    ///0x160 - DSI Host Video VA Current Configuration Register
    pub dsi_vvaccr: DSI_VVACCR,
    _reserved61: [u8; 668usize],
    ///0x400 - DSI Wrapper Configuration Register
    pub dsi_wcfgr: DSI_WCFGR,
    ///0x404 - DSI Wrapper Control Register
    pub dsi_wcr: DSI_WCR,
    ///0x408 - DSI Wrapper Interrupt Enable Register
    pub dsi_wier: DSI_WIER,
    ///0x40c - DSI Wrapper Interrupt & Status Register
    pub dsi_wisr: DSI_WISR,
    ///0x410 - DSI Wrapper Interrupt Flag Clear Register
    pub dsi_wifcr: DSI_WIFCR,
    _reserved66: [u8; 4usize],
    ///0x418 - DSI Wrapper PHY Configuration Register 0
    pub dsi_wpcr0: DSI_WPCR0,
    ///0x41c - DSI Wrapper PHY Configuration Register 1
    pub dsi_wpcr1: DSI_WPCR1,
    ///0x420 - DSI Wrapper PHY Configuration Register 2
    pub dsi_wpcr2: DSI_WPCR2,
    ///0x424 - DSI_WPCR3
    pub dsi_wpcr3: DSI_WPCR3,
    ///0x428 - DSI Wrapper PHY Configuration Register 4
    pub dsi_wpcr4: DSI_WPCR4,
    _reserved71: [u8; 4usize],
    ///0x430 - DSI Wrapper Regulator and PLL Control Register
    pub dsi_wrpcr: DSI_WRPCR,
}
///DSI Host Version Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vr](dsi_vr) module
pub type DSI_VR = crate::Reg<u32, _DSI_VR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VR;
///`read()` method returns [dsi_vr::R](dsi_vr::R) reader structure
impl crate::Readable for DSI_VR {}
///DSI Host Version Register
pub mod dsi_vr;
///DSI Host Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_cr](dsi_cr) module
pub type DSI_CR = crate::Reg<u32, _DSI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CR;
///`read()` method returns [dsi_cr::R](dsi_cr::R) reader structure
impl crate::Readable for DSI_CR {}
///`write(|w| ..)` method takes [dsi_cr::W](dsi_cr::W) writer structure
impl crate::Writable for DSI_CR {}
///DSI Host Control Register
pub mod dsi_cr;
///DSI HOST Clock Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_ccr](dsi_ccr) module
pub type DSI_CCR = crate::Reg<u32, _DSI_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CCR;
///`read()` method returns [dsi_ccr::R](dsi_ccr::R) reader structure
impl crate::Readable for DSI_CCR {}
///`write(|w| ..)` method takes [dsi_ccr::W](dsi_ccr::W) writer structure
impl crate::Writable for DSI_CCR {}
///DSI HOST Clock Control Register
pub mod dsi_ccr;
///DSI Host LTDC VCID Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lvcidr](dsi_lvcidr) module
pub type DSI_LVCIDR = crate::Reg<u32, _DSI_LVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LVCIDR;
///`read()` method returns [dsi_lvcidr::R](dsi_lvcidr::R) reader structure
impl crate::Readable for DSI_LVCIDR {}
///`write(|w| ..)` method takes [dsi_lvcidr::W](dsi_lvcidr::W) writer structure
impl crate::Writable for DSI_LVCIDR {}
///DSI Host LTDC VCID Register
pub mod dsi_lvcidr;
///DSI Host LTDC Color Coding Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lcolcr](dsi_lcolcr) module
pub type DSI_LCOLCR = crate::Reg<u32, _DSI_LCOLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCOLCR;
///`read()` method returns [dsi_lcolcr::R](dsi_lcolcr::R) reader structure
impl crate::Readable for DSI_LCOLCR {}
///`write(|w| ..)` method takes [dsi_lcolcr::W](dsi_lcolcr::W) writer structure
impl crate::Writable for DSI_LCOLCR {}
///DSI Host LTDC Color Coding Register
pub mod dsi_lcolcr;
///DSI Host LTDC Polarity Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lpcr](dsi_lpcr) module
pub type DSI_LPCR = crate::Reg<u32, _DSI_LPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LPCR;
///`read()` method returns [dsi_lpcr::R](dsi_lpcr::R) reader structure
impl crate::Readable for DSI_LPCR {}
///`write(|w| ..)` method takes [dsi_lpcr::W](dsi_lpcr::W) writer structure
impl crate::Writable for DSI_LPCR {}
///DSI Host LTDC Polarity Configuration Register
pub mod dsi_lpcr;
///DSI Host Low-Power mode Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lpmcr](dsi_lpmcr) module
pub type DSI_LPMCR = crate::Reg<u32, _DSI_LPMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LPMCR;
///`read()` method returns [dsi_lpmcr::R](dsi_lpmcr::R) reader structure
impl crate::Readable for DSI_LPMCR {}
///`write(|w| ..)` method takes [dsi_lpmcr::W](dsi_lpmcr::W) writer structure
impl crate::Writable for DSI_LPMCR {}
///DSI Host Low-Power mode Configuration Register
pub mod dsi_lpmcr;
///DSI Host Protocol Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_pcr](dsi_pcr) module
pub type DSI_PCR = crate::Reg<u32, _DSI_PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PCR;
///`read()` method returns [dsi_pcr::R](dsi_pcr::R) reader structure
impl crate::Readable for DSI_PCR {}
///`write(|w| ..)` method takes [dsi_pcr::W](dsi_pcr::W) writer structure
impl crate::Writable for DSI_PCR {}
///DSI Host Protocol Configuration Register
pub mod dsi_pcr;
///DSI Host Generic VCID Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_gvcidr](dsi_gvcidr) module
pub type DSI_GVCIDR = crate::Reg<u32, _DSI_GVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GVCIDR;
///`read()` method returns [dsi_gvcidr::R](dsi_gvcidr::R) reader structure
impl crate::Readable for DSI_GVCIDR {}
///`write(|w| ..)` method takes [dsi_gvcidr::W](dsi_gvcidr::W) writer structure
impl crate::Writable for DSI_GVCIDR {}
///DSI Host Generic VCID Register
pub mod dsi_gvcidr;
///DSI Host mode Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_mcr](dsi_mcr) module
pub type DSI_MCR = crate::Reg<u32, _DSI_MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_MCR;
///`read()` method returns [dsi_mcr::R](dsi_mcr::R) reader structure
impl crate::Readable for DSI_MCR {}
///`write(|w| ..)` method takes [dsi_mcr::W](dsi_mcr::W) writer structure
impl crate::Writable for DSI_MCR {}
///DSI Host mode Configuration Register
pub mod dsi_mcr;
///DSI Host Video mode Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vmcr](dsi_vmcr) module
pub type DSI_VMCR = crate::Reg<u32, _DSI_VMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VMCR;
///`read()` method returns [dsi_vmcr::R](dsi_vmcr::R) reader structure
impl crate::Readable for DSI_VMCR {}
///`write(|w| ..)` method takes [dsi_vmcr::W](dsi_vmcr::W) writer structure
impl crate::Writable for DSI_VMCR {}
///DSI Host Video mode Configuration Register
pub mod dsi_vmcr;
///DSI Host Video Packet Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vpcr](dsi_vpcr) module
pub type DSI_VPCR = crate::Reg<u32, _DSI_VPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VPCR;
///`read()` method returns [dsi_vpcr::R](dsi_vpcr::R) reader structure
impl crate::Readable for DSI_VPCR {}
///`write(|w| ..)` method takes [dsi_vpcr::W](dsi_vpcr::W) writer structure
impl crate::Writable for DSI_VPCR {}
///DSI Host Video Packet Configuration Register
pub mod dsi_vpcr;
///DSI Host Video Chunks Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vccr](dsi_vccr) module
pub type DSI_VCCR = crate::Reg<u32, _DSI_VCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VCCR;
///`read()` method returns [dsi_vccr::R](dsi_vccr::R) reader structure
impl crate::Readable for DSI_VCCR {}
///`write(|w| ..)` method takes [dsi_vccr::W](dsi_vccr::W) writer structure
impl crate::Writable for DSI_VCCR {}
///DSI Host Video Chunks Configuration Register
pub mod dsi_vccr;
///DSI Host Video Null Packet Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vnpcr](dsi_vnpcr) module
pub type DSI_VNPCR = crate::Reg<u32, _DSI_VNPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VNPCR;
///`read()` method returns [dsi_vnpcr::R](dsi_vnpcr::R) reader structure
impl crate::Readable for DSI_VNPCR {}
///`write(|w| ..)` method takes [dsi_vnpcr::W](dsi_vnpcr::W) writer structure
impl crate::Writable for DSI_VNPCR {}
///DSI Host Video Null Packet Configuration Register
pub mod dsi_vnpcr;
///DSI Host Video HSA Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vhsacr](dsi_vhsacr) module
pub type DSI_VHSACR = crate::Reg<u32, _DSI_VHSACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHSACR;
///`read()` method returns [dsi_vhsacr::R](dsi_vhsacr::R) reader structure
impl crate::Readable for DSI_VHSACR {}
///`write(|w| ..)` method takes [dsi_vhsacr::W](dsi_vhsacr::W) writer structure
impl crate::Writable for DSI_VHSACR {}
///DSI Host Video HSA Configuration Register
pub mod dsi_vhsacr;
///DSI Host Video HBP Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vhbpcr](dsi_vhbpcr) module
pub type DSI_VHBPCR = crate::Reg<u32, _DSI_VHBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHBPCR;
///`read()` method returns [dsi_vhbpcr::R](dsi_vhbpcr::R) reader structure
impl crate::Readable for DSI_VHBPCR {}
///`write(|w| ..)` method takes [dsi_vhbpcr::W](dsi_vhbpcr::W) writer structure
impl crate::Writable for DSI_VHBPCR {}
///DSI Host Video HBP Configuration Register
pub mod dsi_vhbpcr;
///DSI Host Video Line Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vlcr](dsi_vlcr) module
pub type DSI_VLCR = crate::Reg<u32, _DSI_VLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VLCR;
///`read()` method returns [dsi_vlcr::R](dsi_vlcr::R) reader structure
impl crate::Readable for DSI_VLCR {}
///`write(|w| ..)` method takes [dsi_vlcr::W](dsi_vlcr::W) writer structure
impl crate::Writable for DSI_VLCR {}
///DSI Host Video Line Configuration Register
pub mod dsi_vlcr;
///DSI Host Video VSA Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvsacr](dsi_vvsacr) module
pub type DSI_VVSACR = crate::Reg<u32, _DSI_VVSACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVSACR;
///`read()` method returns [dsi_vvsacr::R](dsi_vvsacr::R) reader structure
impl crate::Readable for DSI_VVSACR {}
///`write(|w| ..)` method takes [dsi_vvsacr::W](dsi_vvsacr::W) writer structure
impl crate::Writable for DSI_VVSACR {}
///DSI Host Video VSA Configuration Register
pub mod dsi_vvsacr;
///DSI Host Video VBP Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvbpcr](dsi_vvbpcr) module
pub type DSI_VVBPCR = crate::Reg<u32, _DSI_VVBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVBPCR;
///`read()` method returns [dsi_vvbpcr::R](dsi_vvbpcr::R) reader structure
impl crate::Readable for DSI_VVBPCR {}
///`write(|w| ..)` method takes [dsi_vvbpcr::W](dsi_vvbpcr::W) writer structure
impl crate::Writable for DSI_VVBPCR {}
///DSI Host Video VBP Configuration Register
pub mod dsi_vvbpcr;
///DSI Host Video VFP Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvfpcr](dsi_vvfpcr) module
pub type DSI_VVFPCR = crate::Reg<u32, _DSI_VVFPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVFPCR;
///`read()` method returns [dsi_vvfpcr::R](dsi_vvfpcr::R) reader structure
impl crate::Readable for DSI_VVFPCR {}
///`write(|w| ..)` method takes [dsi_vvfpcr::W](dsi_vvfpcr::W) writer structure
impl crate::Writable for DSI_VVFPCR {}
///DSI Host Video VFP Configuration Register
pub mod dsi_vvfpcr;
///DSI Host Video VA Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvacr](dsi_vvacr) module
pub type DSI_VVACR = crate::Reg<u32, _DSI_VVACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVACR;
///`read()` method returns [dsi_vvacr::R](dsi_vvacr::R) reader structure
impl crate::Readable for DSI_VVACR {}
///`write(|w| ..)` method takes [dsi_vvacr::W](dsi_vvacr::W) writer structure
impl crate::Writable for DSI_VVACR {}
///DSI Host Video VA Configuration Register
pub mod dsi_vvacr;
///DSI Host LTDC Command Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lccr](dsi_lccr) module
pub type DSI_LCCR = crate::Reg<u32, _DSI_LCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCCR;
///`read()` method returns [dsi_lccr::R](dsi_lccr::R) reader structure
impl crate::Readable for DSI_LCCR {}
///`write(|w| ..)` method takes [dsi_lccr::W](dsi_lccr::W) writer structure
impl crate::Writable for DSI_LCCR {}
///DSI Host LTDC Command Configuration Register
pub mod dsi_lccr;
///DSI Host Command mode Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_cmcr](dsi_cmcr) module
pub type DSI_CMCR = crate::Reg<u32, _DSI_CMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CMCR;
///`read()` method returns [dsi_cmcr::R](dsi_cmcr::R) reader structure
impl crate::Readable for DSI_CMCR {}
///`write(|w| ..)` method takes [dsi_cmcr::W](dsi_cmcr::W) writer structure
impl crate::Writable for DSI_CMCR {}
///DSI Host Command mode Configuration Register
pub mod dsi_cmcr;
///DSI Host Generic Header Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_ghcr](dsi_ghcr) module
pub type DSI_GHCR = crate::Reg<u32, _DSI_GHCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GHCR;
///`read()` method returns [dsi_ghcr::R](dsi_ghcr::R) reader structure
impl crate::Readable for DSI_GHCR {}
///`write(|w| ..)` method takes [dsi_ghcr::W](dsi_ghcr::W) writer structure
impl crate::Writable for DSI_GHCR {}
///DSI Host Generic Header Configuration Register
pub mod dsi_ghcr;
///DSI Host Generic Payload Data Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_gpdr](dsi_gpdr) module
pub type DSI_GPDR = crate::Reg<u32, _DSI_GPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GPDR;
///`read()` method returns [dsi_gpdr::R](dsi_gpdr::R) reader structure
impl crate::Readable for DSI_GPDR {}
///`write(|w| ..)` method takes [dsi_gpdr::W](dsi_gpdr::W) writer structure
impl crate::Writable for DSI_GPDR {}
///DSI Host Generic Payload Data Register
pub mod dsi_gpdr;
///DSI Host Generic Packet Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_gpsr](dsi_gpsr) module
pub type DSI_GPSR = crate::Reg<u32, _DSI_GPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_GPSR;
///`read()` method returns [dsi_gpsr::R](dsi_gpsr::R) reader structure
impl crate::Readable for DSI_GPSR {}
///DSI Host Generic Packet Status Register
pub mod dsi_gpsr;
///DSI Host Timeout Counter Configuration Register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_tccr0](dsi_tccr0) module
pub type DSI_TCCR0 = crate::Reg<u32, _DSI_TCCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR0;
///`read()` method returns [dsi_tccr0::R](dsi_tccr0::R) reader structure
impl crate::Readable for DSI_TCCR0 {}
///`write(|w| ..)` method takes [dsi_tccr0::W](dsi_tccr0::W) writer structure
impl crate::Writable for DSI_TCCR0 {}
///DSI Host Timeout Counter Configuration Register 0
pub mod dsi_tccr0;
///DSI Host Timeout Counter Configuration Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_tccr1](dsi_tccr1) module
pub type DSI_TCCR1 = crate::Reg<u32, _DSI_TCCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR1;
///`read()` method returns [dsi_tccr1::R](dsi_tccr1::R) reader structure
impl crate::Readable for DSI_TCCR1 {}
///`write(|w| ..)` method takes [dsi_tccr1::W](dsi_tccr1::W) writer structure
impl crate::Writable for DSI_TCCR1 {}
///DSI Host Timeout Counter Configuration Register 1
pub mod dsi_tccr1;
///DSI Host Timeout Counter Configuration Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_tccr2](dsi_tccr2) module
pub type DSI_TCCR2 = crate::Reg<u32, _DSI_TCCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR2;
///`read()` method returns [dsi_tccr2::R](dsi_tccr2::R) reader structure
impl crate::Readable for DSI_TCCR2 {}
///`write(|w| ..)` method takes [dsi_tccr2::W](dsi_tccr2::W) writer structure
impl crate::Writable for DSI_TCCR2 {}
///DSI Host Timeout Counter Configuration Register 2
pub mod dsi_tccr2;
///DSI Host Timeout Counter Configuration Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_tccr3](dsi_tccr3) module
pub type DSI_TCCR3 = crate::Reg<u32, _DSI_TCCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR3;
///`read()` method returns [dsi_tccr3::R](dsi_tccr3::R) reader structure
impl crate::Readable for DSI_TCCR3 {}
///`write(|w| ..)` method takes [dsi_tccr3::W](dsi_tccr3::W) writer structure
impl crate::Writable for DSI_TCCR3 {}
///DSI Host Timeout Counter Configuration Register 3
pub mod dsi_tccr3;
///DSI Host Timeout Counter Configuration Register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_tccr4](dsi_tccr4) module
pub type DSI_TCCR4 = crate::Reg<u32, _DSI_TCCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR4;
///`read()` method returns [dsi_tccr4::R](dsi_tccr4::R) reader structure
impl crate::Readable for DSI_TCCR4 {}
///`write(|w| ..)` method takes [dsi_tccr4::W](dsi_tccr4::W) writer structure
impl crate::Writable for DSI_TCCR4 {}
///DSI Host Timeout Counter Configuration Register 4
pub mod dsi_tccr4;
///DSI Host Timeout Counter Configuration Register 5
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_tccr5](dsi_tccr5) module
pub type DSI_TCCR5 = crate::Reg<u32, _DSI_TCCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_TCCR5;
///`read()` method returns [dsi_tccr5::R](dsi_tccr5::R) reader structure
impl crate::Readable for DSI_TCCR5 {}
///`write(|w| ..)` method takes [dsi_tccr5::W](dsi_tccr5::W) writer structure
impl crate::Writable for DSI_TCCR5 {}
///DSI Host Timeout Counter Configuration Register 5
pub mod dsi_tccr5;
///DSI Host Clock Lane Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_clcr](dsi_clcr) module
pub type DSI_CLCR = crate::Reg<u32, _DSI_CLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CLCR;
///`read()` method returns [dsi_clcr::R](dsi_clcr::R) reader structure
impl crate::Readable for DSI_CLCR {}
///`write(|w| ..)` method takes [dsi_clcr::W](dsi_clcr::W) writer structure
impl crate::Writable for DSI_CLCR {}
///DSI Host Clock Lane Configuration Register
pub mod dsi_clcr;
///DSI Host Clock Lane Timer Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_cltcr](dsi_cltcr) module
pub type DSI_CLTCR = crate::Reg<u32, _DSI_CLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_CLTCR;
///`read()` method returns [dsi_cltcr::R](dsi_cltcr::R) reader structure
impl crate::Readable for DSI_CLTCR {}
///`write(|w| ..)` method takes [dsi_cltcr::W](dsi_cltcr::W) writer structure
impl crate::Writable for DSI_CLTCR {}
///DSI Host Clock Lane Timer Configuration Register
pub mod dsi_cltcr;
///DSI Host Data Lane Timer Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_dltrc](dsi_dltrc) module
pub type DSI_DLTRC = crate::Reg<u32, _DSI_DLTRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_DLTRC;
///`read()` method returns [dsi_dltrc::R](dsi_dltrc::R) reader structure
impl crate::Readable for DSI_DLTRC {}
///`write(|w| ..)` method takes [dsi_dltrc::W](dsi_dltrc::W) writer structure
impl crate::Writable for DSI_DLTRC {}
///DSI Host Data Lane Timer Configuration Register
pub mod dsi_dltrc;
///DSI Host PHY Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_pctlr](dsi_pctlr) module
pub type DSI_PCTLR = crate::Reg<u32, _DSI_PCTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PCTLR;
///`read()` method returns [dsi_pctlr::R](dsi_pctlr::R) reader structure
impl crate::Readable for DSI_PCTLR {}
///`write(|w| ..)` method takes [dsi_pctlr::W](dsi_pctlr::W) writer structure
impl crate::Writable for DSI_PCTLR {}
///DSI Host PHY Control Register
pub mod dsi_pctlr;
///DSI Host PHY Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_pconfr](dsi_pconfr) module
pub type DSI_PCONFR = crate::Reg<u32, _DSI_PCONFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PCONFR;
///`read()` method returns [dsi_pconfr::R](dsi_pconfr::R) reader structure
impl crate::Readable for DSI_PCONFR {}
///`write(|w| ..)` method takes [dsi_pconfr::W](dsi_pconfr::W) writer structure
impl crate::Writable for DSI_PCONFR {}
///DSI Host PHY Configuration Register
pub mod dsi_pconfr;
///DSI Host PHY ULPS Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_pucr](dsi_pucr) module
pub type DSI_PUCR = crate::Reg<u32, _DSI_PUCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PUCR;
///`read()` method returns [dsi_pucr::R](dsi_pucr::R) reader structure
impl crate::Readable for DSI_PUCR {}
///`write(|w| ..)` method takes [dsi_pucr::W](dsi_pucr::W) writer structure
impl crate::Writable for DSI_PUCR {}
///DSI Host PHY ULPS Control Register
pub mod dsi_pucr;
///DSI Host PHY TX Triggers Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_pttcr](dsi_pttcr) module
pub type DSI_PTTCR = crate::Reg<u32, _DSI_PTTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PTTCR;
///`read()` method returns [dsi_pttcr::R](dsi_pttcr::R) reader structure
impl crate::Readable for DSI_PTTCR {}
///`write(|w| ..)` method takes [dsi_pttcr::W](dsi_pttcr::W) writer structure
impl crate::Writable for DSI_PTTCR {}
///DSI Host PHY TX Triggers Configuration Register
pub mod dsi_pttcr;
///DSI Host PHY Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_psr](dsi_psr) module
pub type DSI_PSR = crate::Reg<u32, _DSI_PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_PSR;
///`read()` method returns [dsi_psr::R](dsi_psr::R) reader structure
impl crate::Readable for DSI_PSR {}
///DSI Host PHY Status Register
pub mod dsi_psr;
///DSI Host Interrupt & Status Register 0
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_isr0](dsi_isr0) module
pub type DSI_ISR0 = crate::Reg<u32, _DSI_ISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_ISR0;
///`read()` method returns [dsi_isr0::R](dsi_isr0::R) reader structure
impl crate::Readable for DSI_ISR0 {}
///DSI Host Interrupt & Status Register 0
pub mod dsi_isr0;
///DSI Host Interrupt & Status Register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_isr1](dsi_isr1) module
pub type DSI_ISR1 = crate::Reg<u32, _DSI_ISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_ISR1;
///`read()` method returns [dsi_isr1::R](dsi_isr1::R) reader structure
impl crate::Readable for DSI_ISR1 {}
///DSI Host Interrupt & Status Register 1
pub mod dsi_isr1;
///DSI Host Interrupt Enable Register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_ier0](dsi_ier0) module
pub type DSI_IER0 = crate::Reg<u32, _DSI_IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_IER0;
///`read()` method returns [dsi_ier0::R](dsi_ier0::R) reader structure
impl crate::Readable for DSI_IER0 {}
///`write(|w| ..)` method takes [dsi_ier0::W](dsi_ier0::W) writer structure
impl crate::Writable for DSI_IER0 {}
///DSI Host Interrupt Enable Register 0
pub mod dsi_ier0;
///DSI Host Interrupt Enable Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_ier1](dsi_ier1) module
pub type DSI_IER1 = crate::Reg<u32, _DSI_IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_IER1;
///`read()` method returns [dsi_ier1::R](dsi_ier1::R) reader structure
impl crate::Readable for DSI_IER1 {}
///`write(|w| ..)` method takes [dsi_ier1::W](dsi_ier1::W) writer structure
impl crate::Writable for DSI_IER1 {}
///DSI Host Interrupt Enable Register 1
pub mod dsi_ier1;
///DSI Host Force Interrupt Register 0
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_fir0](dsi_fir0) module
pub type DSI_FIR0 = crate::Reg<u32, _DSI_FIR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_FIR0;
///`write(|w| ..)` method takes [dsi_fir0::W](dsi_fir0::W) writer structure
impl crate::Writable for DSI_FIR0 {}
///DSI Host Force Interrupt Register 0
pub mod dsi_fir0;
///DSI Host Force Interrupt Register 1
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_fir1](dsi_fir1) module
pub type DSI_FIR1 = crate::Reg<u32, _DSI_FIR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_FIR1;
///`write(|w| ..)` method takes [dsi_fir1::W](dsi_fir1::W) writer structure
impl crate::Writable for DSI_FIR1 {}
///DSI Host Force Interrupt Register 1
pub mod dsi_fir1;
///DSI Host Video Shadow Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vscr](dsi_vscr) module
pub type DSI_VSCR = crate::Reg<u32, _DSI_VSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VSCR;
///`read()` method returns [dsi_vscr::R](dsi_vscr::R) reader structure
impl crate::Readable for DSI_VSCR {}
///`write(|w| ..)` method takes [dsi_vscr::W](dsi_vscr::W) writer structure
impl crate::Writable for DSI_VSCR {}
///DSI Host Video Shadow Control Register
pub mod dsi_vscr;
///DSI Host LTDC Current VCID Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lcvcidr](dsi_lcvcidr) module
pub type DSI_LCVCIDR = crate::Reg<u32, _DSI_LCVCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCVCIDR;
///`read()` method returns [dsi_lcvcidr::R](dsi_lcvcidr::R) reader structure
impl crate::Readable for DSI_LCVCIDR {}
///DSI Host LTDC Current VCID Register
pub mod dsi_lcvcidr;
///DSI Host LTDC Current Color Coding Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lcccr](dsi_lcccr) module
pub type DSI_LCCCR = crate::Reg<u32, _DSI_LCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LCCCR;
///`read()` method returns [dsi_lcccr::R](dsi_lcccr::R) reader structure
impl crate::Readable for DSI_LCCCR {}
///DSI Host LTDC Current Color Coding Register
pub mod dsi_lcccr;
///DSI Host Low-Power mode Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_lpmccr](dsi_lpmccr) module
pub type DSI_LPMCCR = crate::Reg<u32, _DSI_LPMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_LPMCCR;
///`read()` method returns [dsi_lpmccr::R](dsi_lpmccr::R) reader structure
impl crate::Readable for DSI_LPMCCR {}
///DSI Host Low-Power mode Current Configuration Register
pub mod dsi_lpmccr;
///DSI Host Video mode Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vmccr](dsi_vmccr) module
pub type DSI_VMCCR = crate::Reg<u32, _DSI_VMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VMCCR;
///`read()` method returns [dsi_vmccr::R](dsi_vmccr::R) reader structure
impl crate::Readable for DSI_VMCCR {}
///DSI Host Video mode Current Configuration Register
pub mod dsi_vmccr;
///DSI Host Video Packet Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vpccr](dsi_vpccr) module
pub type DSI_VPCCR = crate::Reg<u32, _DSI_VPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VPCCR;
///`read()` method returns [dsi_vpccr::R](dsi_vpccr::R) reader structure
impl crate::Readable for DSI_VPCCR {}
///DSI Host Video Packet Current Configuration Register
pub mod dsi_vpccr;
///DSI Host Video Chunks Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vcccr](dsi_vcccr) module
pub type DSI_VCCCR = crate::Reg<u32, _DSI_VCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VCCCR;
///`read()` method returns [dsi_vcccr::R](dsi_vcccr::R) reader structure
impl crate::Readable for DSI_VCCCR {}
///DSI Host Video Chunks Current Configuration Register
pub mod dsi_vcccr;
///DSI Host Video Null Packet Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vnpccr](dsi_vnpccr) module
pub type DSI_VNPCCR = crate::Reg<u32, _DSI_VNPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VNPCCR;
///`read()` method returns [dsi_vnpccr::R](dsi_vnpccr::R) reader structure
impl crate::Readable for DSI_VNPCCR {}
///DSI Host Video Null Packet Current Configuration Register
pub mod dsi_vnpccr;
///DSI Host Video HSA Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vhsaccr](dsi_vhsaccr) module
pub type DSI_VHSACCR = crate::Reg<u32, _DSI_VHSACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHSACCR;
///`read()` method returns [dsi_vhsaccr::R](dsi_vhsaccr::R) reader structure
impl crate::Readable for DSI_VHSACCR {}
///DSI Host Video HSA Current Configuration Register
pub mod dsi_vhsaccr;
///DSI Host Video HBP Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vhbpccr](dsi_vhbpccr) module
pub type DSI_VHBPCCR = crate::Reg<u32, _DSI_VHBPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VHBPCCR;
///`read()` method returns [dsi_vhbpccr::R](dsi_vhbpccr::R) reader structure
impl crate::Readable for DSI_VHBPCCR {}
///DSI Host Video HBP Current Configuration Register
pub mod dsi_vhbpccr;
///DSI Host Video Line Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vlccr](dsi_vlccr) module
pub type DSI_VLCCR = crate::Reg<u32, _DSI_VLCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VLCCR;
///`read()` method returns [dsi_vlccr::R](dsi_vlccr::R) reader structure
impl crate::Readable for DSI_VLCCR {}
///DSI Host Video Line Current Configuration Register
pub mod dsi_vlccr;
///DSI Host Video VSA Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvsaccr](dsi_vvsaccr) module
pub type DSI_VVSACCR = crate::Reg<u32, _DSI_VVSACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVSACCR;
///`read()` method returns [dsi_vvsaccr::R](dsi_vvsaccr::R) reader structure
impl crate::Readable for DSI_VVSACCR {}
///DSI Host Video VSA Current Configuration Register
pub mod dsi_vvsaccr;
///DSI Host Video VBP Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvbpccr](dsi_vvbpccr) module
pub type DSI_VVBPCCR = crate::Reg<u32, _DSI_VVBPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVBPCCR;
///`read()` method returns [dsi_vvbpccr::R](dsi_vvbpccr::R) reader structure
impl crate::Readable for DSI_VVBPCCR {}
///DSI Host Video VBP Current Configuration Register
pub mod dsi_vvbpccr;
///DSI Host Video VFP Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvfpccr](dsi_vvfpccr) module
pub type DSI_VVFPCCR = crate::Reg<u32, _DSI_VVFPCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVFPCCR;
///`read()` method returns [dsi_vvfpccr::R](dsi_vvfpccr::R) reader structure
impl crate::Readable for DSI_VVFPCCR {}
///DSI Host Video VFP Current Configuration Register
pub mod dsi_vvfpccr;
///DSI Host Video VA Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_vvaccr](dsi_vvaccr) module
pub type DSI_VVACCR = crate::Reg<u32, _DSI_VVACCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_VVACCR;
///`read()` method returns [dsi_vvaccr::R](dsi_vvaccr::R) reader structure
impl crate::Readable for DSI_VVACCR {}
///DSI Host Video VA Current Configuration Register
pub mod dsi_vvaccr;
///DSI Wrapper Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wcfgr](dsi_wcfgr) module
pub type DSI_WCFGR = crate::Reg<u32, _DSI_WCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WCFGR;
///`read()` method returns [dsi_wcfgr::R](dsi_wcfgr::R) reader structure
impl crate::Readable for DSI_WCFGR {}
///`write(|w| ..)` method takes [dsi_wcfgr::W](dsi_wcfgr::W) writer structure
impl crate::Writable for DSI_WCFGR {}
///DSI Wrapper Configuration Register
pub mod dsi_wcfgr;
///DSI Wrapper Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wcr](dsi_wcr) module
pub type DSI_WCR = crate::Reg<u32, _DSI_WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WCR;
///`read()` method returns [dsi_wcr::R](dsi_wcr::R) reader structure
impl crate::Readable for DSI_WCR {}
///`write(|w| ..)` method takes [dsi_wcr::W](dsi_wcr::W) writer structure
impl crate::Writable for DSI_WCR {}
///DSI Wrapper Control Register
pub mod dsi_wcr;
///DSI Wrapper Interrupt Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wier](dsi_wier) module
pub type DSI_WIER = crate::Reg<u32, _DSI_WIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WIER;
///`read()` method returns [dsi_wier::R](dsi_wier::R) reader structure
impl crate::Readable for DSI_WIER {}
///`write(|w| ..)` method takes [dsi_wier::W](dsi_wier::W) writer structure
impl crate::Writable for DSI_WIER {}
///DSI Wrapper Interrupt Enable Register
pub mod dsi_wier;
///DSI Wrapper Interrupt & Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wisr](dsi_wisr) module
pub type DSI_WISR = crate::Reg<u32, _DSI_WISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WISR;
///`read()` method returns [dsi_wisr::R](dsi_wisr::R) reader structure
impl crate::Readable for DSI_WISR {}
///DSI Wrapper Interrupt & Status Register
pub mod dsi_wisr;
///DSI Wrapper Interrupt Flag Clear Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wifcr](dsi_wifcr) module
pub type DSI_WIFCR = crate::Reg<u32, _DSI_WIFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WIFCR;
///`read()` method returns [dsi_wifcr::R](dsi_wifcr::R) reader structure
impl crate::Readable for DSI_WIFCR {}
///`write(|w| ..)` method takes [dsi_wifcr::W](dsi_wifcr::W) writer structure
impl crate::Writable for DSI_WIFCR {}
///DSI Wrapper Interrupt Flag Clear Register
pub mod dsi_wifcr;
///DSI Wrapper PHY Configuration Register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wpcr0](dsi_wpcr0) module
pub type DSI_WPCR0 = crate::Reg<u32, _DSI_WPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WPCR0;
///`read()` method returns [dsi_wpcr0::R](dsi_wpcr0::R) reader structure
impl crate::Readable for DSI_WPCR0 {}
///`write(|w| ..)` method takes [dsi_wpcr0::W](dsi_wpcr0::W) writer structure
impl crate::Writable for DSI_WPCR0 {}
///DSI Wrapper PHY Configuration Register 0
pub mod dsi_wpcr0;
///DSI Wrapper PHY Configuration Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wpcr1](dsi_wpcr1) module
pub type DSI_WPCR1 = crate::Reg<u32, _DSI_WPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WPCR1;
///`read()` method returns [dsi_wpcr1::R](dsi_wpcr1::R) reader structure
impl crate::Readable for DSI_WPCR1 {}
///`write(|w| ..)` method takes [dsi_wpcr1::W](dsi_wpcr1::W) writer structure
impl crate::Writable for DSI_WPCR1 {}
///DSI Wrapper PHY Configuration Register 1
pub mod dsi_wpcr1;
///DSI Wrapper PHY Configuration Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wpcr2](dsi_wpcr2) module
pub type DSI_WPCR2 = crate::Reg<u32, _DSI_WPCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WPCR2;
///`read()` method returns [dsi_wpcr2::R](dsi_wpcr2::R) reader structure
impl crate::Readable for DSI_WPCR2 {}
///`write(|w| ..)` method takes [dsi_wpcr2::W](dsi_wpcr2::W) writer structure
impl crate::Writable for DSI_WPCR2 {}
///DSI Wrapper PHY Configuration Register 2
pub mod dsi_wpcr2;
///DSI_WPCR3
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wpcr3](dsi_wpcr3) module
pub type DSI_WPCR3 = crate::Reg<u32, _DSI_WPCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WPCR3;
///`read()` method returns [dsi_wpcr3::R](dsi_wpcr3::R) reader structure
impl crate::Readable for DSI_WPCR3 {}
///`write(|w| ..)` method takes [dsi_wpcr3::W](dsi_wpcr3::W) writer structure
impl crate::Writable for DSI_WPCR3 {}
///DSI_WPCR3
pub mod dsi_wpcr3;
///DSI Wrapper PHY Configuration Register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wpcr4](dsi_wpcr4) module
pub type DSI_WPCR4 = crate::Reg<u32, _DSI_WPCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WPCR4;
///`read()` method returns [dsi_wpcr4::R](dsi_wpcr4::R) reader structure
impl crate::Readable for DSI_WPCR4 {}
///`write(|w| ..)` method takes [dsi_wpcr4::W](dsi_wpcr4::W) writer structure
impl crate::Writable for DSI_WPCR4 {}
///DSI Wrapper PHY Configuration Register 4
pub mod dsi_wpcr4;
///DSI Wrapper Regulator and PLL Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dsi_wrpcr](dsi_wrpcr) module
pub type DSI_WRPCR = crate::Reg<u32, _DSI_WRPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSI_WRPCR;
///`read()` method returns [dsi_wrpcr::R](dsi_wrpcr::R) reader structure
impl crate::Readable for DSI_WRPCR {}
///`write(|w| ..)` method takes [dsi_wrpcr::W](dsi_wrpcr::W) writer structure
impl crate::Writable for DSI_WRPCR {}
///DSI Wrapper Regulator and PLL Control Register
pub mod dsi_wrpcr;
