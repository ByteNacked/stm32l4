///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - power control register
    pub power: POWER,
    ///0x04 - SDI clock control register
    pub clkcr: CLKCR,
    ///0x08 - argument register
    pub arg: ARG,
    ///0x0c - command register
    pub cmd: CMD,
    ///0x10 - command response register
    pub respcmd: RESPCMD,
    ///0x14 - response 1..4 register
    pub resp1: RESP1,
    ///0x18 - response 1..4 register
    pub resp2: RESP2,
    ///0x1c - response 1..4 register
    pub resp3: RESP3,
    ///0x20 - response 1..4 register
    pub resp4: RESP4,
    ///0x24 - data timer register
    pub dtimer: DTIMER,
    ///0x28 - data length register
    pub dlen: DLEN,
    ///0x2c - data control register
    pub dctrl: DCTRL,
    ///0x30 - data counter register
    pub dcount: DCOUNT,
    ///0x34 - status register
    pub sta: STA,
    ///0x38 - interrupt clear register
    pub icr: ICR,
    ///0x3c - mask register
    pub mask: MASK,
    _reserved16: [u8; 8usize],
    ///0x48 - FIFO counter register
    pub fifocnt: FIFOCNT,
    _reserved17: [u8; 52usize],
    ///0x80 - data FIFO register
    pub fifo: FIFO,
}
///power control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [power](power) module
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
///`read()` method returns [power::R](power::R) reader structure
impl crate::Readable for POWER {}
///`write(|w| ..)` method takes [power::W](power::W) writer structure
impl crate::Writable for POWER {}
///power control register
pub mod power;
///SDI clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clkcr](clkcr) module
pub type CLKCR = crate::Reg<u32, _CLKCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCR;
///`read()` method returns [clkcr::R](clkcr::R) reader structure
impl crate::Readable for CLKCR {}
///`write(|w| ..)` method takes [clkcr::W](clkcr::W) writer structure
impl crate::Writable for CLKCR {}
///SDI clock control register
pub mod clkcr;
///argument register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [arg](arg) module
pub type ARG = crate::Reg<u32, _ARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARG;
///`read()` method returns [arg::R](arg::R) reader structure
impl crate::Readable for ARG {}
///`write(|w| ..)` method takes [arg::W](arg::W) writer structure
impl crate::Writable for ARG {}
///argument register
pub mod arg;
///command register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmd](cmd) module
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
///`read()` method returns [cmd::R](cmd::R) reader structure
impl crate::Readable for CMD {}
///`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure
impl crate::Writable for CMD {}
///command register
pub mod cmd;
///command response register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [respcmd](respcmd) module
pub type RESPCMD = crate::Reg<u32, _RESPCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESPCMD;
///`read()` method returns [respcmd::R](respcmd::R) reader structure
impl crate::Readable for RESPCMD {}
///command response register
pub mod respcmd;
///response 1..4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp1](resp1) module
pub type RESP1 = crate::Reg<u32, _RESP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP1;
///`read()` method returns [resp1::R](resp1::R) reader structure
impl crate::Readable for RESP1 {}
///response 1..4 register
pub mod resp1;
///response 1..4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp2](resp2) module
pub type RESP2 = crate::Reg<u32, _RESP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP2;
///`read()` method returns [resp2::R](resp2::R) reader structure
impl crate::Readable for RESP2 {}
///response 1..4 register
pub mod resp2;
///response 1..4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp3](resp3) module
pub type RESP3 = crate::Reg<u32, _RESP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP3;
///`read()` method returns [resp3::R](resp3::R) reader structure
impl crate::Readable for RESP3 {}
///response 1..4 register
pub mod resp3;
///response 1..4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp4](resp4) module
pub type RESP4 = crate::Reg<u32, _RESP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP4;
///`read()` method returns [resp4::R](resp4::R) reader structure
impl crate::Readable for RESP4 {}
///response 1..4 register
pub mod resp4;
///data timer register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtimer](dtimer) module
pub type DTIMER = crate::Reg<u32, _DTIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTIMER;
///`read()` method returns [dtimer::R](dtimer::R) reader structure
impl crate::Readable for DTIMER {}
///`write(|w| ..)` method takes [dtimer::W](dtimer::W) writer structure
impl crate::Writable for DTIMER {}
///data timer register
pub mod dtimer;
///data length register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dlen](dlen) module
pub type DLEN = crate::Reg<u32, _DLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLEN;
///`read()` method returns [dlen::R](dlen::R) reader structure
impl crate::Readable for DLEN {}
///`write(|w| ..)` method takes [dlen::W](dlen::W) writer structure
impl crate::Writable for DLEN {}
///data length register
pub mod dlen;
///data control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dctrl](dctrl) module
pub type DCTRL = crate::Reg<u32, _DCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTRL;
///`read()` method returns [dctrl::R](dctrl::R) reader structure
impl crate::Readable for DCTRL {}
///`write(|w| ..)` method takes [dctrl::W](dctrl::W) writer structure
impl crate::Writable for DCTRL {}
///data control register
pub mod dctrl;
///data counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcount](dcount) module
pub type DCOUNT = crate::Reg<u32, _DCOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCOUNT;
///`read()` method returns [dcount::R](dcount::R) reader structure
impl crate::Readable for DCOUNT {}
///data counter register
pub mod dcount;
///status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sta](sta) module
pub type STA = crate::Reg<u32, _STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STA;
///`read()` method returns [sta::R](sta::R) reader structure
impl crate::Readable for STA {}
///status register
pub mod sta;
///interrupt clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](icr) module
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
///`read()` method returns [icr::R](icr::R) reader structure
impl crate::Readable for ICR {}
///`write(|w| ..)` method takes [icr::W](icr::W) writer structure
impl crate::Writable for ICR {}
///interrupt clear register
pub mod icr;
///mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mask](mask) module
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
///`read()` method returns [mask::R](mask::R) reader structure
impl crate::Readable for MASK {}
///`write(|w| ..)` method takes [mask::W](mask::W) writer structure
impl crate::Writable for MASK {}
///mask register
pub mod mask;
///FIFO counter register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifocnt](fifocnt) module
pub type FIFOCNT = crate::Reg<u32, _FIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOCNT;
///`read()` method returns [fifocnt::R](fifocnt::R) reader structure
impl crate::Readable for FIFOCNT {}
///FIFO counter register
pub mod fifocnt;
///data FIFO register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fifo](fifo) module
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
///`read()` method returns [fifo::R](fifo::R) reader structure
impl crate::Readable for FIFO {}
///`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure
impl crate::Writable for FIFO {}
///data FIFO register
pub mod fifo;
