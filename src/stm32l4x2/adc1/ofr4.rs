///Reader of register OFR4
pub type R = crate::R<u32, super::OFR4>;
///Writer for register OFR4
pub type W = crate::W<u32, super::OFR4>;
///Register OFR4 `reset()`'s with value 0
impl crate::ResetValue for super::OFR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OFFSET4_EN`
pub type OFFSET4_EN_R = crate::R<bool, bool>;
///Write proxy for field `OFFSET4_EN`
pub struct OFFSET4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_EN_W<'a> {
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
///Reader of field `OFFSET4_CH`
pub type OFFSET4_CH_R = crate::R<u8, u8>;
///Write proxy for field `OFFSET4_CH`
pub struct OFFSET4_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_CH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
///Reader of field `OFFSET4`
pub type OFFSET4_R = crate::R<u16, u16>;
///Write proxy for field `OFFSET4`
pub struct OFFSET4_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET4_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bit 31 - OFFSET4_EN
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 26:30 - OFFSET4_CH
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bits 0:11 - OFFSET4
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bit 31 - OFFSET4_EN
    #[inline(always)]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W {
        OFFSET4_EN_W { w: self }
    }
    ///Bits 26:30 - OFFSET4_CH
    #[inline(always)]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W {
        OFFSET4_CH_W { w: self }
    }
    ///Bits 0:11 - OFFSET4
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W {
        OFFSET4_W { w: self }
    }
}
