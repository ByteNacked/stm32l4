///Reader of register DCR
pub type R = crate::R<u32, super::DCR>;
///Writer for register DCR
pub type W = crate::W<u32, super::DCR>;
///Register DCR `reset()`'s with value 0
impl crate::ResetValue for super::DCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FSIZE`
pub type FSIZE_R = crate::R<u8, u8>;
///Write proxy for field `FSIZE`
pub struct FSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
///Reader of field `CSHT`
pub type CSHT_R = crate::R<u8, u8>;
///Write proxy for field `CSHT`
pub struct CSHT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSHT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
///Reader of field `CKMODE`
pub type CKMODE_R = crate::R<bool, bool>;
///Write proxy for field `CKMODE`
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
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
impl R {
    ///Bits 16:20 - FLASH memory size
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 8:10 - Chip select high time
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 0 - Mode 0 / mode 3
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 16:20 - FLASH memory size
    #[inline(always)]
    pub fn fsize(&mut self) -> FSIZE_W {
        FSIZE_W { w: self }
    }
    ///Bits 8:10 - Chip select high time
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W {
        CSHT_W { w: self }
    }
    ///Bit 0 - Mode 0 / mode 3
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
}
