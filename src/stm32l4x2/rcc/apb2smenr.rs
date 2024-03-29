///Reader of register APB2SMENR
pub type R = crate::R<u32, super::APB2SMENR>;
///Writer for register APB2SMENR
pub type W = crate::W<u32, super::APB2SMENR>;
///Register APB2SMENR `reset()`'s with value 0x0167_7c01
impl crate::ResetValue for super::APB2SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0167_7c01
    }
}
///Reader of field `SAI1SMEN`
pub type SAI1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `SAI1SMEN`
pub struct SAI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///Reader of field `TIM16SMEN`
pub type TIM16SMEN_R = crate::R<bool, bool>;
///Write proxy for field `TIM16SMEN`
pub struct TIM16SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16SMEN_W<'a> {
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
///Reader of field `TIM15SMEN`
pub type TIM15SMEN_R = crate::R<bool, bool>;
///Write proxy for field `TIM15SMEN`
pub struct TIM15SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15SMEN_W<'a> {
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
///Reader of field `USART1SMEN`
pub type USART1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `USART1SMEN`
pub struct USART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SMEN_W<'a> {
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
///Reader of field `SPI1SMEN`
pub type SPI1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `SPI1SMEN`
pub struct SPI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1SMEN_W<'a> {
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
///Reader of field `TIM1SMEN`
pub type TIM1SMEN_R = crate::R<bool, bool>;
///Write proxy for field `TIM1SMEN`
pub struct TIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `SDMMCSMEN`
pub type SDMMCSMEN_R = crate::R<bool, bool>;
///Write proxy for field `SDMMCSMEN`
pub struct SDMMCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMCSMEN_W<'a> {
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
///Reader of field `SYSCFGSMEN`
pub type SYSCFGSMEN_R = crate::R<bool, bool>;
///Write proxy for field `SYSCFGSMEN`
pub struct SYSCFGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGSMEN_W<'a> {
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
    ///Bit 21 - SAI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sai1smen(&self) -> SAI1SMEN_R {
        SAI1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim15smen(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 14 - USART1clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart1smen(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 12 - SPI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi1smen(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim1smen(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - SDMMC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmcsmen(&self) -> SDMMCSMEN_R {
        SDMMCSMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 0 - SYSCFG clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 21 - SAI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sai1smen(&mut self) -> SAI1SMEN_W {
        SAI1SMEN_W { w: self }
    }
    ///Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W {
        TIM16SMEN_W { w: self }
    }
    ///Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim15smen(&mut self) -> TIM15SMEN_W {
        TIM15SMEN_W { w: self }
    }
    ///Bit 14 - USART1clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn usart1smen(&mut self) -> USART1SMEN_W {
        USART1SMEN_W { w: self }
    }
    ///Bit 12 - SPI1 clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn spi1smen(&mut self) -> SPI1SMEN_W {
        SPI1SMEN_W { w: self }
    }
    ///Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn tim1smen(&mut self) -> TIM1SMEN_W {
        TIM1SMEN_W { w: self }
    }
    ///Bit 10 - SDMMC clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn sdmmcsmen(&mut self) -> SDMMCSMEN_W {
        SDMMCSMEN_W { w: self }
    }
    ///Bit 0 - SYSCFG clocks enable during Sleep and Stop modes
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W {
        SYSCFGSMEN_W { w: self }
    }
}
