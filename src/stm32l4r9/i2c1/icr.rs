///Writer for register ICR
pub type W = crate::W<u32, super::ICR>;
///Register ICR `reset()`'s with value 0
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Alert flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTCF_AW {
    ///1: Clears the ALERT flag in ISR register
    CLEAR = 1,
}
impl From<ALERTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ALERTCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `ALERTCF`
pub struct ALERTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERTCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALERTCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the ALERT flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALERTCF_AW::CLEAR)
    }
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
///Timeout detection flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUTCF_AW {
    ///1: Clears the TIMOUT flag in ISR register
    CLEAR = 1,
}
impl From<TIMOUTCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `TIMOUTCF`
pub struct TIMOUTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUTCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIMOUTCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the TIMOUT flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMOUTCF_AW::CLEAR)
    }
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
///PEC Error flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECCF_AW {
    ///1: Clears the PEC flag in ISR register
    CLEAR = 1,
}
impl From<PECCF_AW> for bool {
    #[inline(always)]
    fn from(variant: PECCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `PECCF`
pub struct PECCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PECCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PECCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the PEC flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECCF_AW::CLEAR)
    }
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
///Overrun/Underrun flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRCF_AW {
    ///1: Clears the OVR flag in ISR register
    CLEAR = 1,
}
impl From<OVRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `OVRCF`
pub struct OVRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the OVR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRCF_AW::CLEAR)
    }
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
///Arbitration lost flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLOCF_AW {
    ///1: Clears the ARLO flag in ISR register
    CLEAR = 1,
}
impl From<ARLOCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ARLOCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `ARLOCF`
pub struct ARLOCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARLOCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARLOCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the ARLO flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARLOCF_AW::CLEAR)
    }
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
///Bus error flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERRCF_AW {
    ///1: Clears the BERR flag in ISR register
    CLEAR = 1,
}
impl From<BERRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: BERRCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `BERRCF`
pub struct BERRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> BERRCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BERRCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the BERR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BERRCF_AW::CLEAR)
    }
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
///Stop detection flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPCF_AW {
    ///1: Clears the STOP flag in ISR register
    CLEAR = 1,
}
impl From<STOPCF_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `STOPCF`
pub struct STOPCF_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STOPCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the STOP flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPCF_AW::CLEAR)
    }
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
///Not Acknowledge flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKCF_AW {
    ///1: Clears the NACK flag in ISR register
    CLEAR = 1,
}
impl From<NACKCF_AW> for bool {
    #[inline(always)]
    fn from(variant: NACKCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `NACKCF`
pub struct NACKCF_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NACKCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the NACK flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NACKCF_AW::CLEAR)
    }
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
///Address Matched flag clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRCF_AW {
    ///1: Clears the ADDR flag in ISR register
    CLEAR = 1,
}
impl From<ADDRCF_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDRCF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `ADDRCF`
pub struct ADDRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADDRCF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clears the ADDR flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRCF_AW::CLEAR)
    }
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
impl W {
    ///Bit 13 - Alert flag clear
    #[inline(always)]
    pub fn alertcf(&mut self) -> ALERTCF_W {
        ALERTCF_W { w: self }
    }
    ///Bit 12 - Timeout detection flag clear
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W {
        TIMOUTCF_W { w: self }
    }
    ///Bit 11 - PEC Error flag clear
    #[inline(always)]
    pub fn peccf(&mut self) -> PECCF_W {
        PECCF_W { w: self }
    }
    ///Bit 10 - Overrun/Underrun flag clear
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W {
        OVRCF_W { w: self }
    }
    ///Bit 9 - Arbitration lost flag clear
    #[inline(always)]
    pub fn arlocf(&mut self) -> ARLOCF_W {
        ARLOCF_W { w: self }
    }
    ///Bit 8 - Bus error flag clear
    #[inline(always)]
    pub fn berrcf(&mut self) -> BERRCF_W {
        BERRCF_W { w: self }
    }
    ///Bit 5 - Stop detection flag clear
    #[inline(always)]
    pub fn stopcf(&mut self) -> STOPCF_W {
        STOPCF_W { w: self }
    }
    ///Bit 4 - Not Acknowledge flag clear
    #[inline(always)]
    pub fn nackcf(&mut self) -> NACKCF_W {
        NACKCF_W { w: self }
    }
    ///Bit 3 - Address Matched flag clear
    #[inline(always)]
    pub fn addrcf(&mut self) -> ADDRCF_W {
        ADDRCF_W { w: self }
    }
}
