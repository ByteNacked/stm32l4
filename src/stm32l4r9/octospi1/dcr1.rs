///Reader of register DCR1
pub type R = crate::R<u32, super::DCR1>;
///Writer for register DCR1
pub type W = crate::W<u32, super::DCR1>;
///Register DCR1 `reset()`'s with value 0
impl crate::ResetValue for super::DCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
///Reader of field `FRCK`
pub type FRCK_R = crate::R<bool, bool>;
///Write proxy for field `FRCK`
pub struct FRCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCK_W<'a> {
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
///Reader of field `DEVSIZE`
pub type DEVSIZE_R = crate::R<u8, u8>;
///Write proxy for field `DEVSIZE`
pub struct DEVSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
///Reader of field `MTYP`
pub type MTYP_R = crate::R<u8, u8>;
///Write proxy for field `MTYP`
pub struct MTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> MTYP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    ///Bit 0 - Mode 0 / mode 3
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Free running clock
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 8:10 - Chip-select high time
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 16:20 - Device size
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:25 - Memory type
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    ///Bit 0 - Mode 0 / mode 3
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    ///Bit 1 - Free running clock
    #[inline(always)]
    pub fn frck(&mut self) -> FRCK_W {
        FRCK_W { w: self }
    }
    ///Bits 8:10 - Chip-select high time
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W {
        CSHT_W { w: self }
    }
    ///Bits 16:20 - Device size
    #[inline(always)]
    pub fn devsize(&mut self) -> DEVSIZE_W {
        DEVSIZE_W { w: self }
    }
    ///Bits 24:25 - Memory type
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W {
        MTYP_W { w: self }
    }
}
