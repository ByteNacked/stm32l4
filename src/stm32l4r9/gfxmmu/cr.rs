///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `B0OIE`
pub type B0OIE_R = crate::R<bool, bool>;
///Write proxy for field `B0OIE`
pub struct B0OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0OIE_W<'a> {
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
///Reader of field `B1OIE`
pub type B1OIE_R = crate::R<bool, bool>;
///Write proxy for field `B1OIE`
pub struct B1OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1OIE_W<'a> {
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
///Reader of field `B2OIE`
pub type B2OIE_R = crate::R<bool, bool>;
///Write proxy for field `B2OIE`
pub struct B2OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B2OIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `B3OIE`
pub type B3OIE_R = crate::R<bool, bool>;
///Write proxy for field `B3OIE`
pub struct B3OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B3OIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `AMEIE`
pub type AMEIE_R = crate::R<bool, bool>;
///Write proxy for field `AMEIE`
pub struct AMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Reader of field `BM192`
pub type BM192_R = crate::R<bool, bool>;
///Write proxy for field `BM192`
pub struct BM192_W<'a> {
    w: &'a mut W,
}
impl<'a> BM192_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    ///Bit 0 - Buffer 0 overflow interrupt enable
    #[inline(always)]
    pub fn b0oie(&self) -> B0OIE_R {
        B0OIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable
    #[inline(always)]
    pub fn b1oie(&self) -> B1OIE_R {
        B1OIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable
    #[inline(always)]
    pub fn b2oie(&self) -> B2OIE_R {
        B2OIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable
    #[inline(always)]
    pub fn b3oie(&self) -> B3OIE_R {
        B3OIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - AHB master error interrupt enable
    #[inline(always)]
    pub fn ameie(&self) -> AMEIE_R {
        AMEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 6 - 192 Block mode
    #[inline(always)]
    pub fn bm192(&self) -> BM192_R {
        BM192_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Buffer 0 overflow interrupt enable
    #[inline(always)]
    pub fn b0oie(&mut self) -> B0OIE_W {
        B0OIE_W { w: self }
    }
    ///Bit 1 - Buffer 1 overflow interrupt enable
    #[inline(always)]
    pub fn b1oie(&mut self) -> B1OIE_W {
        B1OIE_W { w: self }
    }
    ///Bit 2 - Buffer 2 overflow interrupt enable
    #[inline(always)]
    pub fn b2oie(&mut self) -> B2OIE_W {
        B2OIE_W { w: self }
    }
    ///Bit 3 - Buffer 3 overflow interrupt enable
    #[inline(always)]
    pub fn b3oie(&mut self) -> B3OIE_W {
        B3OIE_W { w: self }
    }
    ///Bit 4 - AHB master error interrupt enable
    #[inline(always)]
    pub fn ameie(&mut self) -> AMEIE_W {
        AMEIE_W { w: self }
    }
    ///Bit 6 - 192 Block mode
    #[inline(always)]
    pub fn bm192(&mut self) -> BM192_W {
        BM192_W { w: self }
    }
}
