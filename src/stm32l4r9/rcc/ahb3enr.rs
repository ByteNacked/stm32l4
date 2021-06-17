///Reader of register AHB3ENR
pub type R = crate::R<u32, super::AHB3ENR>;
///Writer for register AHB3ENR
pub type W = crate::W<u32, super::AHB3ENR>;
///Register AHB3ENR `reset()`'s with value 0
impl crate::ResetValue for super::AHB3ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FMCEN`
pub type FMCEN_R = crate::R<bool, bool>;
///Write proxy for field `FMCEN`
pub struct FMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCEN_W<'a> {
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
///Reader of field `OSPI2EN`
pub type OSPI2EN_R = crate::R<bool, bool>;
///Write proxy for field `OSPI2EN`
pub struct OSPI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPI2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `OSPI1EN`
pub type OSPI1EN_R = crate::R<bool, bool>;
///Write proxy for field `OSPI1EN`
pub struct OSPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPI1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    ///Bit 0 - Flexible memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 9 - OSPI2EN memory interface clock enable
    #[inline(always)]
    pub fn ospi2en(&self) -> OSPI2EN_R {
        OSPI2EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - OSPI1EN memory interface clock enable
    #[inline(always)]
    pub fn ospi1en(&self) -> OSPI1EN_R {
        OSPI1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller clock enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W { w: self }
    }
    ///Bit 9 - OSPI2EN memory interface clock enable
    #[inline(always)]
    pub fn ospi2en(&mut self) -> OSPI2EN_W {
        OSPI2EN_W { w: self }
    }
    ///Bit 8 - OSPI1EN memory interface clock enable
    #[inline(always)]
    pub fn ospi1en(&mut self) -> OSPI1EN_W {
        OSPI1EN_W { w: self }
    }
}
