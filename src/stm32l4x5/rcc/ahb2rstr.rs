///Reader of register AHB2RSTR
pub type R = crate::R<u32, super::AHB2RSTR>;
///Writer for register AHB2RSTR
pub type W = crate::W<u32, super::AHB2RSTR>;
///Register AHB2RSTR `reset()`'s with value 0
impl crate::ResetValue for super::AHB2RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RNGRST`
pub type RNGRST_R = crate::R<bool, bool>;
///Write proxy for field `RNGRST`
pub struct RNGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Reader of field `AESRST`
pub type AESRST_R = crate::R<bool, bool>;
///Write proxy for field `AESRST`
pub struct AESRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AESRST_W<'a> {
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
///Reader of field `ADCRST`
pub type ADCRST_R = crate::R<bool, bool>;
///Write proxy for field `ADCRST`
pub struct ADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `OTGFSRST`
pub type OTGFSRST_R = crate::R<bool, bool>;
///Write proxy for field `OTGFSRST`
pub struct OTGFSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `GPIOHRST`
pub type GPIOHRST_R = crate::R<bool, bool>;
///Write proxy for field `GPIOHRST`
pub struct GPIOHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHRST_W<'a> {
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
///Reader of field `GPIOGRST`
pub type GPIOGRST_R = crate::R<bool, bool>;
///Write proxy for field `GPIOGRST`
pub struct GPIOGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGRST_W<'a> {
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
///Reader of field `GPIOFRST`
pub type GPIOFRST_R = crate::R<bool, bool>;
///Write proxy for field `GPIOFRST`
pub struct GPIOFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFRST_W<'a> {
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
///Reader of field `GPIOERST`
pub type GPIOERST_R = crate::R<bool, bool>;
///Write proxy for field `GPIOERST`
pub struct GPIOERST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOERST_W<'a> {
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
///Reader of field `GPIODRST`
pub type GPIODRST_R = crate::R<bool, bool>;
///Write proxy for field `GPIODRST`
pub struct GPIODRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `GPIOCRST`
pub type GPIOCRST_R = crate::R<bool, bool>;
///Write proxy for field `GPIOCRST`
pub struct GPIOCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCRST_W<'a> {
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
///Reader of field `GPIOBRST`
pub type GPIOBRST_R = crate::R<bool, bool>;
///Write proxy for field `GPIOBRST`
pub struct GPIOBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBRST_W<'a> {
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
///Reader of field `GPIOARST`
pub type GPIOARST_R = crate::R<bool, bool>;
///Write proxy for field `GPIOARST`
pub struct GPIOARST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOARST_W<'a> {
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
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&mut self) -> RNGRST_W {
        RNGRST_W { w: self }
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&mut self) -> AESRST_W {
        AESRST_W { w: self }
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W { w: self }
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W {
        OTGFSRST_W { w: self }
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W {
        GPIOHRST_W { w: self }
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W {
        GPIOGRST_W { w: self }
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W {
        GPIOFRST_W { w: self }
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W {
        GPIOERST_W { w: self }
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W {
        GPIODRST_W { w: self }
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W {
        GPIOCRST_W { w: self }
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W {
        GPIOBRST_W { w: self }
    }
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W {
        GPIOARST_W { w: self }
    }
}
