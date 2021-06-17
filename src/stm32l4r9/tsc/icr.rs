///Reader of register ICR
pub type R = crate::R<u32, super::ICR>;
///Writer for register ICR
pub type W = crate::W<u32, super::ICR>;
///Register ICR `reset()`'s with value 0
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MCEIC`
pub type MCEIC_R = crate::R<bool, bool>;
///Write proxy for field `MCEIC`
pub struct MCEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCEIC_W<'a> {
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
///Reader of field `EOAIC`
pub type EOAIC_R = crate::R<bool, bool>;
///Write proxy for field `EOAIC`
pub struct EOAIC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOAIC_W<'a> {
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
    ///Bit 1 - Max count error interrupt clear
    #[inline(always)]
    pub fn mceic(&self) -> MCEIC_R {
        MCEIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - End of acquisition interrupt clear
    #[inline(always)]
    pub fn eoaic(&self) -> EOAIC_R {
        EOAIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Max count error interrupt clear
    #[inline(always)]
    pub fn mceic(&mut self) -> MCEIC_W {
        MCEIC_W { w: self }
    }
    ///Bit 0 - End of acquisition interrupt clear
    #[inline(always)]
    pub fn eoaic(&mut self) -> EOAIC_W {
        EOAIC_W { w: self }
    }
}
