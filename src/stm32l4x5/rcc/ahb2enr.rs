///Reader of register AHB2ENR
pub type R = crate::R<u32, super::AHB2ENR>;
///Writer for register AHB2ENR
pub type W = crate::W<u32, super::AHB2ENR>;
///Register AHB2ENR `reset()`'s with value 0
impl crate::ResetValue for super::AHB2ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RNGEN`
pub type RNGEN_R = crate::R<bool, bool>;
///Write proxy for field `RNGEN`
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
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
///Reader of field `AESEN`
pub type AESEN_R = crate::R<bool, bool>;
///Write proxy for field `AESEN`
pub struct AESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESEN_W<'a> {
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
///Reader of field `ADCEN`
pub type ADCEN_R = crate::R<bool, bool>;
///Write proxy for field `ADCEN`
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
///Reader of field `OTGFSEN`
pub type OTGFSEN_R = crate::R<bool, bool>;
///Write proxy for field `OTGFSEN`
pub struct OTGFSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSEN_W<'a> {
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
///Reader of field `GPIOHEN`
pub type GPIOHEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOHEN`
pub struct GPIOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHEN_W<'a> {
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
///Reader of field `GPIOGEN`
pub type GPIOGEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOGEN`
pub struct GPIOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGEN_W<'a> {
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
///Reader of field `GPIOFEN`
pub type GPIOFEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOFEN`
pub struct GPIOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFEN_W<'a> {
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
///Reader of field `GPIOEEN`
pub type GPIOEEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOEEN`
pub struct GPIOEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOEEN_W<'a> {
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
///Reader of field `GPIODEN`
pub type GPIODEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIODEN`
pub struct GPIODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODEN_W<'a> {
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
///Reader of field `GPIOCEN`
pub type GPIOCEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOCEN`
pub struct GPIOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCEN_W<'a> {
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
///Reader of field `GPIOBEN`
pub type GPIOBEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOBEN`
pub struct GPIOBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBEN_W<'a> {
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
///Reader of field `GPIOAEN`
pub type GPIOAEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOAEN`
pub struct GPIOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOAEN_W<'a> {
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
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W {
        AESEN_W { w: self }
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W {
        OTGFSEN_W { w: self }
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W {
        GPIOHEN_W { w: self }
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W {
        GPIOGEN_W { w: self }
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W {
        GPIOFEN_W { w: self }
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W {
        GPIOEEN_W { w: self }
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W {
        GPIODEN_W { w: self }
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W {
        GPIOCEN_W { w: self }
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W {
        GPIOBEN_W { w: self }
    }
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W {
        GPIOAEN_W { w: self }
    }
}
