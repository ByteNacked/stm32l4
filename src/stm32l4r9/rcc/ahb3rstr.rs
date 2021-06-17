///Reader of register AHB3RSTR
pub type R = crate::R<u32, super::AHB3RSTR>;
///Writer for register AHB3RSTR
pub type W = crate::W<u32, super::AHB3RSTR>;
///Register AHB3RSTR `reset()`'s with value 0
impl crate::ResetValue for super::AHB3RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FMCRST`
pub type FMCRST_R = crate::R<bool, bool>;
///Write proxy for field `FMCRST`
pub struct FMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCRST_W<'a> {
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
///Reader of field `OSPI2RST`
pub type OSPI2RST_R = crate::R<bool, bool>;
///Write proxy for field `OSPI2RST`
pub struct OSPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPI2RST_W<'a> {
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
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 9 - OctOSPI2 memory interface reset
    #[inline(always)]
    pub fn ospi2rst(&self) -> OSPI2RST_R {
        OSPI2RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - OctOSPI1 memory interface reset
    #[inline(always)]
    pub fn ospi1en(&self) -> OSPI1EN_R {
        OSPI1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
    ///Bit 9 - OctOSPI2 memory interface reset
    #[inline(always)]
    pub fn ospi2rst(&mut self) -> OSPI2RST_W {
        OSPI2RST_W { w: self }
    }
    ///Bit 8 - OctOSPI1 memory interface reset
    #[inline(always)]
    pub fn ospi1en(&mut self) -> OSPI1EN_W {
        OSPI1EN_W { w: self }
    }
}
