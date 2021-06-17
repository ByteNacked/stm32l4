///Reader of register OFR1
pub type R = crate::R<u32, super::OFR1>;
///Writer for register OFR1
pub type W = crate::W<u32, super::OFR1>;
///Register OFR1 `reset()`'s with value 0
impl crate::ResetValue for super::OFR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OFFSET1_EN`
pub type OFFSET1_EN_R = crate::R<bool, bool>;
///Write proxy for field `OFFSET1_EN`
pub struct OFFSET1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
///Reader of field `OFFSET1_CH`
pub type OFFSET1_CH_R = crate::R<u8, u8>;
///Write proxy for field `OFFSET1_CH`
pub struct OFFSET1_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET1_CH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
///Reader of field `OFFSET1`
pub type OFFSET1_R = crate::R<u16, u16>;
///Write proxy for field `OFFSET1`
pub struct OFFSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bit 31 - OFFSET1_EN
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bits 0:11 - OFFSET1
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bit 31 - OFFSET1_EN
    #[inline(always)]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W {
        OFFSET1_EN_W { w: self }
    }
    ///Bits 26:30 - OFFSET1_CH
    #[inline(always)]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W {
        OFFSET1_CH_W { w: self }
    }
    ///Bits 0:11 - OFFSET1
    #[inline(always)]
    pub fn offset1(&mut self) -> OFFSET1_W {
        OFFSET1_W { w: self }
    }
}
