///Reader of register LUT239L
pub type R = crate::R<u32, super::LUT239L>;
///Writer for register LUT239L
pub type W = crate::W<u32, super::LUT239L>;
///Register LUT239L `reset()`'s with value 0
impl crate::ResetValue for super::LUT239L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EN`
pub type EN_R = crate::R<bool, bool>;
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
///Reader of field `FVB`
pub type FVB_R = crate::R<u8, u8>;
///Write proxy for field `FVB`
pub struct FVB_W<'a> {
    w: &'a mut W,
}
impl<'a> FVB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `LVB`
pub type LVB_R = crate::R<u8, u8>;
///Write proxy for field `LVB`
pub struct LVB_W<'a> {
    w: &'a mut W,
}
impl<'a> LVB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 8:15 - First Valid Block
    #[inline(always)]
    pub fn fvb(&self) -> FVB_R {
        FVB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Last Valid Block
    #[inline(always)]
    pub fn lvb(&self) -> LVB_R {
        LVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    ///Bits 8:15 - First Valid Block
    #[inline(always)]
    pub fn fvb(&mut self) -> FVB_W {
        FVB_W { w: self }
    }
    ///Bits 16:23 - Last Valid Block
    #[inline(always)]
    pub fn lvb(&mut self) -> LVB_W {
        LVB_W { w: self }
    }
}
