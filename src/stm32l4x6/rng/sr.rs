///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SEIS`
pub type SEIS_R = crate::R<bool, bool>;
///Write proxy for field `SEIS`
pub struct SEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIS_W<'a> {
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
///Reader of field `CEIS`
pub type CEIS_R = crate::R<bool, bool>;
///Write proxy for field `CEIS`
pub struct CEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `SECS`
pub type SECS_R = crate::R<bool, bool>;
///Reader of field `CECS`
pub type CECS_R = crate::R<bool, bool>;
///Reader of field `DRDY`
pub type DRDY_R = crate::R<bool, bool>;
impl R {
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 2 - Seed error current status
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Clock error current status
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Data ready
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 6 - Seed error interrupt status
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W {
        SEIS_W { w: self }
    }
    ///Bit 5 - Clock error interrupt status
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W {
        CEIS_W { w: self }
    }
}
