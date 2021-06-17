///Reader of register P1CR
pub type R = crate::R<u32, super::P1CR>;
///Writer for register P1CR
pub type W = crate::W<u32, super::P1CR>;
///Register P1CR `reset()`'s with value 0x0301_0111
impl crate::ResetValue for super::P1CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0301_0111
    }
}
///Reader of field `CLKEN`
pub type CLKEN_R = crate::R<bool, bool>;
///Write proxy for field `CLKEN`
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
///Reader of field `CLKSRC`
pub type CLKSRC_R = crate::R<bool, bool>;
///Write proxy for field `CLKSRC`
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
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
///Reader of field `DQSEN`
pub type DQSEN_R = crate::R<bool, bool>;
///Write proxy for field `DQSEN`
pub struct DQSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSEN_W<'a> {
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
///Reader of field `DQSSRC`
pub type DQSSRC_R = crate::R<bool, bool>;
///Write proxy for field `DQSSRC`
pub struct DQSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSSRC_W<'a> {
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
///Reader of field `NCSEN`
pub type NCSEN_R = crate::R<bool, bool>;
///Write proxy for field `NCSEN`
pub struct NCSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NCSEN_W<'a> {
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
///Reader of field `NCSSRC`
pub type NCSSRC_R = crate::R<bool, bool>;
///Write proxy for field `NCSSRC`
pub struct NCSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> NCSSRC_W<'a> {
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
///Reader of field `IOLEN`
pub type IOLEN_R = crate::R<bool, bool>;
///Write proxy for field `IOLEN`
pub struct IOLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `IOLSRC`
pub type IOLSRC_R = crate::R<u8, u8>;
///Write proxy for field `IOLSRC`
pub struct IOLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLSRC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
///Reader of field `IOHEN`
pub type IOHEN_R = crate::R<bool, bool>;
///Write proxy for field `IOHEN`
pub struct IOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `IOHSRC`
pub type IOHSRC_R = crate::R<u8, u8>;
///Write proxy for field `IOHSRC`
pub struct IOHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHSRC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    ///Bit 0 - CLK/CLK Enable for Port
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - CLK/CLK Source for Port
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - DQS Enable for Port
    #[inline(always)]
    pub fn dqsen(&self) -> DQSEN_R {
        DQSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - DQS Source for Port
    #[inline(always)]
    pub fn dqssrc(&self) -> DQSSRC_R {
        DQSSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 8 - CS Enable for Port
    #[inline(always)]
    pub fn ncsen(&self) -> NCSEN_R {
        NCSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - CS Source for Port
    #[inline(always)]
    pub fn ncssrc(&self) -> NCSSRC_R {
        NCSSRC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 16 - Enable for Port
    #[inline(always)]
    pub fn iolen(&self) -> IOLEN_R {
        IOLEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 17:18 - Source for Port
    #[inline(always)]
    pub fn iolsrc(&self) -> IOLSRC_R {
        IOLSRC_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    ///Bit 24 - Enable for Port n
    #[inline(always)]
    pub fn iohen(&self) -> IOHEN_R {
        IOHEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 25:26 - Source for Port
    #[inline(always)]
    pub fn iohsrc(&self) -> IOHSRC_R {
        IOHSRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    ///Bit 0 - CLK/CLK Enable for Port
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    ///Bit 1 - CLK/CLK Source for Port
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    ///Bit 4 - DQS Enable for Port
    #[inline(always)]
    pub fn dqsen(&mut self) -> DQSEN_W {
        DQSEN_W { w: self }
    }
    ///Bit 5 - DQS Source for Port
    #[inline(always)]
    pub fn dqssrc(&mut self) -> DQSSRC_W {
        DQSSRC_W { w: self }
    }
    ///Bit 8 - CS Enable for Port
    #[inline(always)]
    pub fn ncsen(&mut self) -> NCSEN_W {
        NCSEN_W { w: self }
    }
    ///Bit 9 - CS Source for Port
    #[inline(always)]
    pub fn ncssrc(&mut self) -> NCSSRC_W {
        NCSSRC_W { w: self }
    }
    ///Bit 16 - Enable for Port
    #[inline(always)]
    pub fn iolen(&mut self) -> IOLEN_W {
        IOLEN_W { w: self }
    }
    ///Bits 17:18 - Source for Port
    #[inline(always)]
    pub fn iolsrc(&mut self) -> IOLSRC_W {
        IOLSRC_W { w: self }
    }
    ///Bit 24 - Enable for Port n
    #[inline(always)]
    pub fn iohen(&mut self) -> IOHEN_W {
        IOHEN_W { w: self }
    }
    ///Bits 25:26 - Source for Port
    #[inline(always)]
    pub fn iohsrc(&mut self) -> IOHSRC_W {
        IOHSRC_W { w: self }
    }
}
