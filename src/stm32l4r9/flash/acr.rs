///Reader of register ACR
pub type R = crate::R<u32, super::ACR>;
///Writer for register ACR
pub type W = crate::W<u32, super::ACR>;
///Register ACR `reset()`'s with value 0x0600
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600
    }
}
///Reader of field `LATENCY`
pub type LATENCY_R = crate::R<u8, u8>;
///Write proxy for field `LATENCY`
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Reader of field `PRFTEN`
pub type PRFTEN_R = crate::R<bool, bool>;
///Write proxy for field `PRFTEN`
pub struct PRFTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRFTEN_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Reader of field `ICEN`
pub type ICEN_R = crate::R<bool, bool>;
///Write proxy for field `ICEN`
pub struct ICEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEN_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `DCEN`
pub type DCEN_R = crate::R<bool, bool>;
///Write proxy for field `DCEN`
pub struct DCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCEN_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Reader of field `ICRST`
pub type ICRST_R = crate::R<bool, bool>;
///Write proxy for field `ICRST`
pub struct ICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICRST_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `DCRST`
pub type DCRST_R = crate::R<bool, bool>;
///Write proxy for field `DCRST`
pub struct DCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRST_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `RUN_PD`
pub type RUN_PD_R = crate::R<bool, bool>;
///Write proxy for field `RUN_PD`
pub struct RUN_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_PD_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `SLEEP_PD`
pub type SLEEP_PD_R = crate::R<bool, bool>;
///Write proxy for field `SLEEP_PD`
pub struct SLEEP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_PD_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W {
        PRFTEN_W { w: self }
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W {
        ICEN_W { w: self }
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W {
        DCEN_W { w: self }
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W {
        ICRST_W { w: self }
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&mut self) -> DCRST_W {
        DCRST_W { w: self }
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W {
        RUN_PD_W { w: self }
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W {
        SLEEP_PD_W { w: self }
    }
}
