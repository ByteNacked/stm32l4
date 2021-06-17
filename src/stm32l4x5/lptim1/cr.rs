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
///Reader of field `CNTSTRT`
pub type CNTSTRT_R = crate::R<bool, bool>;
///Write proxy for field `CNTSTRT`
pub struct CNTSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSTRT_W<'a> {
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
///Reader of field `SNGSTRT`
pub type SNGSTRT_R = crate::R<bool, bool>;
///Write proxy for field `SNGSTRT`
pub struct SNGSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SNGSTRT_W<'a> {
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
///Reader of field `ENABLE`
pub type ENABLE_R = crate::R<bool, bool>;
///Write proxy for field `ENABLE`
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    ///Bit 2 - Timer start in continuous mode
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - LPTIM start in single mode
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - LPTIM Enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Timer start in continuous mode
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W {
        CNTSTRT_W { w: self }
    }
    ///Bit 1 - LPTIM start in single mode
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SNGSTRT_W {
        SNGSTRT_W { w: self }
    }
    ///Bit 0 - LPTIM Enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
