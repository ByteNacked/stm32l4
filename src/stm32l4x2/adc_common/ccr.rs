///Reader of register CCR
pub type R = crate::R<u32, super::CCR>;
///Writer for register CCR
pub type W = crate::W<u32, super::CCR>;
///Register CCR `reset()`'s with value 0
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CH18SEL`
pub type CH18SEL_R = crate::R<bool, bool>;
///Write proxy for field `CH18SEL`
pub struct CH18SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18SEL_W<'a> {
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
///Reader of field `CH17SEL`
pub type CH17SEL_R = crate::R<bool, bool>;
///Write proxy for field `CH17SEL`
pub struct CH17SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH17SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///Reader of field `VREFEN`
pub type VREFEN_R = crate::R<bool, bool>;
///Write proxy for field `VREFEN`
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
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
///Reader of field `PRESC`
pub type PRESC_R = crate::R<u8, u8>;
///Write proxy for field `PRESC`
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
///Reader of field `CKMODE`
pub type CKMODE_R = crate::R<u8, u8>;
///Write proxy for field `CKMODE`
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Reader of field `MDMA`
pub type MDMA_R = crate::R<u8, u8>;
///Write proxy for field `MDMA`
pub struct MDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Reader of field `DMACFG`
pub type DMACFG_R = crate::R<bool, bool>;
///Write proxy for field `DMACFG`
pub struct DMACFG_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACFG_W<'a> {
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
///Reader of field `DELAY`
pub type DELAY_R = crate::R<u8, u8>;
///Write proxy for field `DELAY`
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `DUAL`
pub type DUAL_R = crate::R<u8, u8>;
///Write proxy for field `DUAL`
pub struct DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    ///Bit 24 - CH18 selection (Vbat)
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 23 - CH17 selection (temperature)
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - Vrefint enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 14:15 - Direct memory access mode for dual ADC mode
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bit 13 - DMA configuration (for dual ADC mode)
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bit 24 - CH18 selection (Vbat)
    #[inline(always)]
    pub fn ch18sel(&mut self) -> CH18SEL_W {
        CH18SEL_W { w: self }
    }
    ///Bit 23 - CH17 selection (temperature)
    #[inline(always)]
    pub fn ch17sel(&mut self) -> CH17SEL_W {
        CH17SEL_W { w: self }
    }
    ///Bit 22 - Vrefint enable
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    ///Bits 14:15 - Direct memory access mode for dual ADC mode
    #[inline(always)]
    pub fn mdma(&mut self) -> MDMA_W {
        MDMA_W { w: self }
    }
    ///Bit 13 - DMA configuration (for dual ADC mode)
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W {
        DMACFG_W { w: self }
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W {
        DUAL_W { w: self }
    }
}
