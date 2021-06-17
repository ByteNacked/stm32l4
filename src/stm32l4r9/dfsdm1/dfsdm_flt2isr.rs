///Reader of register DFSDM_FLT2ISR
pub type R = crate::R<u32, super::DFSDM_FLT2ISR>;
///Reader of field `SCDF`
pub type SCDF_R = crate::R<u8, u8>;
///Reader of field `CKABF`
pub type CKABF_R = crate::R<u8, u8>;
///Reader of field `RCIP`
pub type RCIP_R = crate::R<bool, bool>;
///Reader of field `JCIP`
pub type JCIP_R = crate::R<bool, bool>;
///Reader of field `AWDF`
pub type AWDF_R = crate::R<bool, bool>;
///Reader of field `ROVRF`
pub type ROVRF_R = crate::R<bool, bool>;
///Reader of field `JOVRF`
pub type JOVRF_R = crate::R<bool, bool>;
///Reader of field `REOCF`
pub type REOCF_R = crate::R<bool, bool>;
///Reader of field `JEOCF`
pub type JEOCF_R = crate::R<bool, bool>;
impl R {
    ///Bits 24:31 - short-circuit detector flag
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:23 - Clock absence flag
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 14 - Regular conversion in progress status
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Injected conversion in progress status
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 4 - Analog watchdog
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Regular conversion overrun flag
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Injected conversion overrun flag
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - End of regular conversion flag
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - End of injected conversion flag
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 0x01) != 0)
    }
}
