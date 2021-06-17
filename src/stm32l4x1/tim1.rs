///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - slave mode control register
    pub smcr: SMCR,
    ///0x0c - DMA/Interrupt enable register
    pub dier: DIER,
    ///0x10 - status register
    pub sr: SR,
    ///0x14 - event generation register
    pub egr: EGR,
    _reserved_6_ccmr1: [u8; 4usize],
    _reserved_7_ccmr2: [u8; 4usize],
    ///0x20 - capture/compare enable register
    pub ccer: CCER,
    ///0x24 - counter
    pub cnt: CNT,
    ///0x28 - prescaler
    pub psc: PSC,
    ///0x2c - auto-reload register
    pub arr: ARR,
    ///0x30 - repetition counter register
    pub rcr: RCR,
    ///0x34 - capture/compare register 1
    pub ccr1: CCR,
    ///0x38 - capture/compare register 1
    pub ccr2: CCR,
    ///0x3c - capture/compare register 1
    pub ccr3: CCR,
    ///0x40 - capture/compare register 1
    pub ccr4: CCR,
    ///0x44 - break and dead-time register
    pub bdtr: BDTR,
    ///0x48 - DMA control register
    pub dcr: DCR,
    ///0x4c - DMA address for full transfer
    pub dmar: DMAR,
    ///0x50 - DMA address for full transfer
    pub or1: OR1,
    ///0x54 - capture/compare mode register 2 (output mode)
    pub ccmr3_output: CCMR3_OUTPUT,
    ///0x58 - capture/compare register 4
    pub ccr5: CCR5,
    ///0x5c - capture/compare register 4
    pub ccr6: CCR6,
    ///0x60 - DMA address for full transfer
    pub or2: OR2,
    ///0x64 - DMA address for full transfer
    pub or3: OR3,
}
impl RegisterBlock {
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_INPUT) }
    }
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub fn ccmr1_input_mut(&self) -> &mut CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_INPUT) }
    }
    ///0x18 - capture/compare mode register 1 (output mode)
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_OUTPUT) }
    }
    ///0x18 - capture/compare mode register 1 (output mode)
    #[inline(always)]
    pub fn ccmr1_output_mut(&self) -> &mut CCMR1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CCMR1_OUTPUT) }
    }
    ///0x1c - capture/compare mode register 2 (input mode)
    #[inline(always)]
    pub fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_INPUT) }
    }
    ///0x1c - capture/compare mode register 2 (input mode)
    #[inline(always)]
    pub fn ccmr2_input_mut(&self) -> &mut CCMR2_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CCMR2_INPUT) }
    }
    ///0x1c - capture/compare mode register 2 (output mode)
    #[inline(always)]
    pub fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_OUTPUT) }
    }
    ///0x1c - capture/compare mode register 2 (output mode)
    #[inline(always)]
    pub fn ccmr2_output_mut(&self) -> &mut CCMR2_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CCMR2_OUTPUT) }
    }
}
///control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](cr1) module
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
///`read()` method returns [cr1::R](cr1::R) reader structure
impl crate::Readable for CR1 {}
///`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure
impl crate::Writable for CR1 {}
///control register 1
pub mod cr1;
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](cr2) module
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
///`read()` method returns [cr2::R](cr2::R) reader structure
impl crate::Readable for CR2 {}
///`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure
impl crate::Writable for CR2 {}
///control register 2
pub mod cr2;
///slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smcr](smcr) module
pub type SMCR = crate::Reg<u32, _SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCR;
///`read()` method returns [smcr::R](smcr::R) reader structure
impl crate::Readable for SMCR {}
///`write(|w| ..)` method takes [smcr::W](smcr::W) writer structure
impl crate::Writable for SMCR {}
///slave mode control register
pub mod smcr;
///DMA/Interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dier](dier) module
pub type DIER = crate::Reg<u32, _DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIER;
///`read()` method returns [dier::R](dier::R) reader structure
impl crate::Readable for DIER {}
///`write(|w| ..)` method takes [dier::W](dier::W) writer structure
impl crate::Writable for DIER {}
///DMA/Interrupt enable register
pub mod dier;
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///`write(|w| ..)` method takes [sr::W](sr::W) writer structure
impl crate::Writable for SR {}
///status register
pub mod sr;
///event generation register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [egr](egr) module
pub type EGR = crate::Reg<u32, _EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EGR;
///`write(|w| ..)` method takes [egr::W](egr::W) writer structure
impl crate::Writable for EGR {}
///event generation register
pub mod egr;
///capture/compare mode register 1 (output mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_output](ccmr1_output) module
pub type CCMR1_OUTPUT = crate::Reg<u32, _CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_OUTPUT;
///`read()` method returns [ccmr1_output::R](ccmr1_output::R) reader structure
impl crate::Readable for CCMR1_OUTPUT {}
///`write(|w| ..)` method takes [ccmr1_output::W](ccmr1_output::W) writer structure
impl crate::Writable for CCMR1_OUTPUT {}
///capture/compare mode register 1 (output mode)
pub mod ccmr1_output;
///capture/compare mode register 1 (input mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_input](ccmr1_input) module
pub type CCMR1_INPUT = crate::Reg<u32, _CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR1_INPUT;
///`read()` method returns [ccmr1_input::R](ccmr1_input::R) reader structure
impl crate::Readable for CCMR1_INPUT {}
///`write(|w| ..)` method takes [ccmr1_input::W](ccmr1_input::W) writer structure
impl crate::Writable for CCMR1_INPUT {}
///capture/compare mode register 1 (input mode)
pub mod ccmr1_input;
///capture/compare mode register 2 (output mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr2_output](ccmr2_output) module
pub type CCMR2_OUTPUT = crate::Reg<u32, _CCMR2_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR2_OUTPUT;
///`read()` method returns [ccmr2_output::R](ccmr2_output::R) reader structure
impl crate::Readable for CCMR2_OUTPUT {}
///`write(|w| ..)` method takes [ccmr2_output::W](ccmr2_output::W) writer structure
impl crate::Writable for CCMR2_OUTPUT {}
///capture/compare mode register 2 (output mode)
pub mod ccmr2_output;
///capture/compare mode register 2 (input mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr2_input](ccmr2_input) module
pub type CCMR2_INPUT = crate::Reg<u32, _CCMR2_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR2_INPUT;
///`read()` method returns [ccmr2_input::R](ccmr2_input::R) reader structure
impl crate::Readable for CCMR2_INPUT {}
///`write(|w| ..)` method takes [ccmr2_input::W](ccmr2_input::W) writer structure
impl crate::Writable for CCMR2_INPUT {}
///capture/compare mode register 2 (input mode)
pub mod ccmr2_input;
///capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccer](ccer) module
pub type CCER = crate::Reg<u32, _CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCER;
///`read()` method returns [ccer::R](ccer::R) reader structure
impl crate::Readable for CCER {}
///`write(|w| ..)` method takes [ccer::W](ccer::W) writer structure
impl crate::Writable for CCER {}
///capture/compare enable register
pub mod ccer;
///counter
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cnt](cnt) module
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
///`read()` method returns [cnt::R](cnt::R) reader structure
impl crate::Readable for CNT {}
///`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure
impl crate::Writable for CNT {}
///counter
pub mod cnt;
///prescaler
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psc](psc) module
pub type PSC = crate::Reg<u32, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
///`read()` method returns [psc::R](psc::R) reader structure
impl crate::Readable for PSC {}
///`write(|w| ..)` method takes [psc::W](psc::W) writer structure
impl crate::Writable for PSC {}
///prescaler
pub mod psc;
///auto-reload register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [arr](arr) module
pub type ARR = crate::Reg<u32, _ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARR;
///`read()` method returns [arr::R](arr::R) reader structure
impl crate::Readable for ARR {}
///`write(|w| ..)` method takes [arr::W](arr::W) writer structure
impl crate::Writable for ARR {}
///auto-reload register
pub mod arr;
///repetition counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcr](rcr) module
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
///`read()` method returns [rcr::R](rcr::R) reader structure
impl crate::Readable for RCR {}
///`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure
impl crate::Writable for RCR {}
///repetition counter register
pub mod rcr;
///capture/compare register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](ccr) module
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
///`read()` method returns [ccr::R](ccr::R) reader structure
impl crate::Readable for CCR {}
///`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure
impl crate::Writable for CCR {}
///capture/compare register 1
pub mod ccr;
///break and dead-time register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtr](bdtr) module
pub type BDTR = crate::Reg<u32, _BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTR;
///`read()` method returns [bdtr::R](bdtr::R) reader structure
impl crate::Readable for BDTR {}
///`write(|w| ..)` method takes [bdtr::W](bdtr::W) writer structure
impl crate::Writable for BDTR {}
///break and dead-time register
pub mod bdtr;
///DMA control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr](dcr) module
pub type DCR = crate::Reg<u32, _DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR;
///`read()` method returns [dcr::R](dcr::R) reader structure
impl crate::Readable for DCR {}
///`write(|w| ..)` method takes [dcr::W](dcr::W) writer structure
impl crate::Writable for DCR {}
///DMA control register
pub mod dcr;
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmar](dmar) module
pub type DMAR = crate::Reg<u32, _DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAR;
///`read()` method returns [dmar::R](dmar::R) reader structure
impl crate::Readable for DMAR {}
///`write(|w| ..)` method takes [dmar::W](dmar::W) writer structure
impl crate::Writable for DMAR {}
///DMA address for full transfer
pub mod dmar;
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or1](or1) module
pub type OR1 = crate::Reg<u32, _OR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OR1;
///`read()` method returns [or1::R](or1::R) reader structure
impl crate::Readable for OR1 {}
///`write(|w| ..)` method takes [or1::W](or1::W) writer structure
impl crate::Writable for OR1 {}
///DMA address for full transfer
pub mod or1;
///capture/compare mode register 2 (output mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr3_output](ccmr3_output) module
pub type CCMR3_OUTPUT = crate::Reg<u32, _CCMR3_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCMR3_OUTPUT;
///`read()` method returns [ccmr3_output::R](ccmr3_output::R) reader structure
impl crate::Readable for CCMR3_OUTPUT {}
///`write(|w| ..)` method takes [ccmr3_output::W](ccmr3_output::W) writer structure
impl crate::Writable for CCMR3_OUTPUT {}
///capture/compare mode register 2 (output mode)
pub mod ccmr3_output;
///capture/compare register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr5](ccr5) module
pub type CCR5 = crate::Reg<u32, _CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR5;
///`read()` method returns [ccr5::R](ccr5::R) reader structure
impl crate::Readable for CCR5 {}
///`write(|w| ..)` method takes [ccr5::W](ccr5::W) writer structure
impl crate::Writable for CCR5 {}
///capture/compare register 4
pub mod ccr5;
///capture/compare register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr6](ccr6) module
pub type CCR6 = crate::Reg<u32, _CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR6;
///`read()` method returns [ccr6::R](ccr6::R) reader structure
impl crate::Readable for CCR6 {}
///`write(|w| ..)` method takes [ccr6::W](ccr6::W) writer structure
impl crate::Writable for CCR6 {}
///capture/compare register 4
pub mod ccr6;
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or2](or2) module
pub type OR2 = crate::Reg<u32, _OR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OR2;
///`read()` method returns [or2::R](or2::R) reader structure
impl crate::Readable for OR2 {}
///`write(|w| ..)` method takes [or2::W](or2::W) writer structure
impl crate::Writable for OR2 {}
///DMA address for full transfer
pub mod or2;
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or3](or3) module
pub type OR3 = crate::Reg<u32, _OR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OR3;
///`read()` method returns [or3::R](or3::R) reader structure
impl crate::Readable for OR3 {}
///`write(|w| ..)` method takes [or3::W](or3::W) writer structure
impl crate::Writable for OR3 {}
///DMA address for full transfer
pub mod or3;
