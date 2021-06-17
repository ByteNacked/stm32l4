///Reader of register AHB2SMENR
pub type R = crate::R<u32, super::AHB2SMENR>;
///Writer for register AHB2SMENR
pub type W = crate::W<u32, super::AHB2SMENR>;
///Register AHB2SMENR `reset()`'s with value 0x0005_32ff
impl crate::ResetValue for super::AHB2SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0005_32ff
    }
}
///Reader of field `GPIOASMEN`
pub type GPIOASMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOASMEN`
pub struct GPIOASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOASMEN_W<'a> {
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
///Reader of field `GPIOBSMEN`
pub type GPIOBSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOBSMEN`
pub struct GPIOBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBSMEN_W<'a> {
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
///Reader of field `GPIOCSMEN`
pub type GPIOCSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOCSMEN`
pub struct GPIOCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCSMEN_W<'a> {
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
///Reader of field `GPIODSMEN`
pub type GPIODSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIODSMEN`
pub struct GPIODSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODSMEN_W<'a> {
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
///Reader of field `GPIOESMEN`
pub type GPIOESMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOESMEN`
pub struct GPIOESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOESMEN_W<'a> {
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
///Reader of field `GPIOFSMEN`
pub type GPIOFSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOFSMEN`
pub struct GPIOFSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFSMEN_W<'a> {
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
///Reader of field `GPIOGSMEN`
pub type GPIOGSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOGSMEN`
pub struct GPIOGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGSMEN_W<'a> {
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
///Reader of field `GPIOHSMEN`
pub type GPIOHSMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOHSMEN`
pub struct GPIOHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHSMEN_W<'a> {
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
///Reader of field `GPIOISMEN`
pub type GPIOISMEN_R = crate::R<bool, bool>;
///Write proxy for field `GPIOISMEN`
pub struct GPIOISMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOISMEN_W<'a> {
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
///Reader of field `SRAM2SMEN`
pub type SRAM2SMEN_R = crate::R<bool, bool>;
///Write proxy for field `SRAM2SMEN`
pub struct SRAM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2SMEN_W<'a> {
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
///Reader of field `SRAM3SMEN`
pub type SRAM3SMEN_R = crate::R<bool, bool>;
///Write proxy for field `SRAM3SMEN`
pub struct SRAM3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Reader of field `OTGFSSMEN`
pub type OTGFSSMEN_R = crate::R<bool, bool>;
///Write proxy for field `OTGFSSMEN`
pub struct OTGFSSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSSMEN_W<'a> {
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
///Reader of field `ADCFSSMEN`
pub type ADCFSSMEN_R = crate::R<bool, bool>;
///Write proxy for field `ADCFSSMEN`
pub struct ADCFSSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCFSSMEN_W<'a> {
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
///Reader of field `DCMISMEN`
pub type DCMISMEN_R = crate::R<bool, bool>;
///Write proxy for field `DCMISMEN`
pub struct DCMISMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMISMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Reader of field `AESSMEN`
pub type AESSMEN_R = crate::R<bool, bool>;
///Write proxy for field `AESSMEN`
pub struct AESSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSMEN_W<'a> {
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
///Reader of field `HASHSMEN`
pub type HASHSMEN_R = crate::R<bool, bool>;
///Write proxy for field `HASHSMEN`
pub struct HASHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Reader of field `RNGSMEN`
pub type RNGSMEN_R = crate::R<bool, bool>;
///Write proxy for field `RNGSMEN`
pub struct RNGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSMEN_W<'a> {
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
///Reader of field `OSPIMSMEN`
pub type OSPIMSMEN_R = crate::R<bool, bool>;
///Write proxy for field `OSPIMSMEN`
pub struct OSPIMSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPIMSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///Reader of field `SDMMC1SMEN`
pub type SDMMC1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `SDMMC1SMEN`
pub struct SDMMC1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&self) -> GPIOGSMEN_R {
        GPIOGSMEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - IO port I clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioismen(&self) -> GPIOISMEN_R {
        GPIOISMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram3smen(&self) -> SRAM3SMEN_R {
        SRAM3SMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 12 - OTG full speed clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn otgfssmen(&self) -> OTGFSSMEN_R {
        OTGFSSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - DCMI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dcmismen(&self) -> DCMISMEN_R {
        DCMISMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn hashsmen(&self) -> HASHSMEN_R {
        HASHSMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ospimsmen(&self) -> OSPIMSMEN_R {
        OSPIMSMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmc1smen(&self) -> SDMMC1SMEN_R {
        SDMMC1SMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W {
        GPIOASMEN_W { w: self }
    }
    ///Bit 1 - IO port B clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W {
        GPIOBSMEN_W { w: self }
    }
    ///Bit 2 - IO port C clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W {
        GPIOCSMEN_W { w: self }
    }
    ///Bit 3 - IO port D clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W {
        GPIODSMEN_W { w: self }
    }
    ///Bit 4 - IO port E clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W {
        GPIOESMEN_W { w: self }
    }
    ///Bit 5 - IO port F clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W {
        GPIOFSMEN_W { w: self }
    }
    ///Bit 6 - IO port G clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiogsmen(&mut self) -> GPIOGSMEN_W {
        GPIOGSMEN_W { w: self }
    }
    ///Bit 7 - IO port H clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W {
        GPIOHSMEN_W { w: self }
    }
    ///Bit 8 - IO port I clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn gpioismen(&mut self) -> GPIOISMEN_W {
        GPIOISMEN_W { w: self }
    }
    ///Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W {
        SRAM2SMEN_W { w: self }
    }
    ///Bit 10 - SRAM2 interface clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sram3smen(&mut self) -> SRAM3SMEN_W {
        SRAM3SMEN_W { w: self }
    }
    ///Bit 12 - OTG full speed clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn otgfssmen(&mut self) -> OTGFSSMEN_W {
        OTGFSSMEN_W { w: self }
    }
    ///Bit 13 - ADC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W {
        ADCFSSMEN_W { w: self }
    }
    ///Bit 14 - DCMI clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn dcmismen(&mut self) -> DCMISMEN_W {
        DCMISMEN_W { w: self }
    }
    ///Bit 16 - AES accelerator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W {
        AESSMEN_W { w: self }
    }
    ///Bit 17 - HASH clock enable during Sleep and Stop modes
    #[inline(always)]
    pub fn hashsmen(&mut self) -> HASHSMEN_W {
        HASHSMEN_W { w: self }
    }
    ///Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W {
        RNGSMEN_W { w: self }
    }
    ///Bit 20 - OctoSPI IO manager clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn ospimsmen(&mut self) -> OSPIMSMEN_W {
        OSPIMSMEN_W { w: self }
    }
    ///Bit 22 - SDMMC1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmc1smen(&mut self) -> SDMMC1SMEN_W {
        SDMMC1SMEN_W { w: self }
    }
}
