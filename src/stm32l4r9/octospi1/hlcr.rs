///Reader of register HLCR
pub type R = crate::R<u32, super::HLCR>;
///Writer for register HLCR
pub type W = crate::W<u32, super::HLCR>;
///Register HLCR `reset()`'s with value 0
impl crate::ResetValue for super::HLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LM`
pub type LM_R = crate::R<bool, bool>;
///Write proxy for field `LM`
pub struct LM_W<'a> {
    w: &'a mut W,
}
impl<'a> LM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `WZL`
pub type WZL_R = crate::R<bool, bool>;
///Write proxy for field `WZL`
pub struct WZL_W<'a> {
    w: &'a mut W,
}
impl<'a> WZL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `TACC`
pub type TACC_R = crate::R<u8, u8>;
///Write proxy for field `TACC`
pub struct TACC_W<'a> {
    w: &'a mut W,
}
impl<'a> TACC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `TRWR`
pub type TRWR_R = crate::R<u8, u8>;
///Write proxy for field `TRWR`
pub struct TRWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRWR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Latency mode
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Latency mode
    #[inline(always)]
    pub fn lm(&mut self) -> LM_W {
        LM_W { w: self }
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    pub fn wzl(&mut self) -> WZL_W {
        WZL_W { w: self }
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    pub fn tacc(&mut self) -> TACC_W {
        TACC_W { w: self }
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    pub fn trwr(&mut self) -> TRWR_W {
        TRWR_W { w: self }
    }
}
