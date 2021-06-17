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
///Reader of field `QSPIRST`
pub type QSPIRST_R = crate::R<bool, bool>;
///Write proxy for field `QSPIRST`
pub struct QSPIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIRST_W<'a> {
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
impl R {
    ///Bit 8 - Quad SPI memory interface reset
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 8 - Quad SPI memory interface reset
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W {
        QSPIRST_W { w: self }
    }
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
}
