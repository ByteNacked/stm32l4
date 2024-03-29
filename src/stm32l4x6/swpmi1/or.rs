///Reader of register OR
pub type R = crate::R<u32, super::OR>;
///Writer for register OR
pub type W = crate::W<u32, super::OR>;
///Register OR `reset()`'s with value 0
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SWP_TBYP`
pub type SWP_TBYP_R = crate::R<bool, bool>;
///Write proxy for field `SWP_TBYP`
pub struct SWP_TBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_TBYP_W<'a> {
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
///Reader of field `SWP_CLASS`
pub type SWP_CLASS_R = crate::R<bool, bool>;
///Write proxy for field `SWP_CLASS`
pub struct SWP_CLASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_CLASS_W<'a> {
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
impl R {
    ///Bit 0 - SWP transceiver bypass
    #[inline(always)]
    pub fn swp_tbyp(&self) -> SWP_TBYP_R {
        SWP_TBYP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - SWP class selection
    #[inline(always)]
    pub fn swp_class(&self) -> SWP_CLASS_R {
        SWP_CLASS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - SWP transceiver bypass
    #[inline(always)]
    pub fn swp_tbyp(&mut self) -> SWP_TBYP_W {
        SWP_TBYP_W { w: self }
    }
    ///Bit 1 - SWP class selection
    #[inline(always)]
    pub fn swp_class(&mut self) -> SWP_CLASS_W {
        SWP_CLASS_W { w: self }
    }
}
