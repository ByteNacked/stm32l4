///Reader of register L1CR
pub type R = crate::R<u32, super::L1CR>;
///Writer for register L1CR
pub type W = crate::W<u32, super::L1CR>;
///Register L1CR `reset()`'s with value 0
impl crate::ResetValue for super::L1CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LEN`
pub type LEN_R = crate::R<bool, bool>;
///Write proxy for field `LEN`
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
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
///Reader of field `COLKEN`
pub type COLKEN_R = crate::R<bool, bool>;
///Write proxy for field `COLKEN`
pub struct COLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COLKEN_W<'a> {
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
///Reader of field `CLUTEN`
pub type CLUTEN_R = crate::R<bool, bool>;
///Write proxy for field `CLUTEN`
pub struct CLUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLUTEN_W<'a> {
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
impl R {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W {
        COLKEN_W { w: self }
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W {
        CLUTEN_W { w: self }
    }
}
