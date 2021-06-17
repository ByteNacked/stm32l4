///Reader of register OFR2
pub type R = crate::R<u32, super::OFR2>;
///Writer for register OFR2
pub type W = crate::W<u32, super::OFR2>;
///Register OFR2 `reset()`'s with value 0
impl crate::ResetValue for super::OFR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OFFSET2_EN`
pub type OFFSET2_EN_R = crate::R<bool, bool>;
///Write proxy for field `OFFSET2_EN`
pub struct OFFSET2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_EN_W<'a> {
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
///Reader of field `OFFSET2_CH`
pub type OFFSET2_CH_R = crate::R<u8, u8>;
///Write proxy for field `OFFSET2_CH`
pub struct OFFSET2_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_CH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
///Reader of field `OFFSET2`
pub type OFFSET2_R = crate::R<u16, u16>;
///Write proxy for field `OFFSET2`
pub struct OFFSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bit 31 - OFFSET2_EN
    #[inline(always)]
    pub fn offset2_en(&self) -> OFFSET2_EN_R {
        OFFSET2_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 26:30 - OFFSET2_CH
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bits 0:11 - OFFSET2
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bit 31 - OFFSET2_EN
    #[inline(always)]
    pub fn offset2_en(&mut self) -> OFFSET2_EN_W {
        OFFSET2_EN_W { w: self }
    }
    ///Bits 26:30 - OFFSET2_CH
    #[inline(always)]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W {
        OFFSET2_CH_W { w: self }
    }
    ///Bits 0:11 - OFFSET2
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W {
        OFFSET2_W { w: self }
    }
}
