///Reader of register DADDR
pub type R = crate::R<u32, super::DADDR>;
///Writer for register DADDR
pub type W = crate::W<u32, super::DADDR>;
///Register DADDR `reset()`'s with value 0
impl crate::ResetValue for super::DADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ADD`
pub type ADD_R = crate::R<u8, u8>;
///Write proxy for field `ADD`
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
///Reader of field `EF`
pub type EF_R = crate::R<bool, bool>;
///Write proxy for field `EF`
pub struct EF_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    ///Bits 0:6 - Device address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:6 - Device address
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W {
        EF_W { w: self }
    }
}
