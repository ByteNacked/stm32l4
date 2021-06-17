///Reader of register APB1RSTR2
pub type R = crate::R<u32, super::APB1RSTR2>;
///Writer for register APB1RSTR2
pub type W = crate::W<u32, super::APB1RSTR2>;
///Register APB1RSTR2 `reset()`'s with value 0
impl crate::ResetValue for super::APB1RSTR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LPTIM2RST`
pub type LPTIM2RST_R = crate::R<bool, bool>;
///Write proxy for field `LPTIM2RST`
pub struct LPTIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2RST_W<'a> {
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
///Reader of field `SWPMI1RST`
pub type SWPMI1RST_R = crate::R<bool, bool>;
///Write proxy for field `SWPMI1RST`
pub struct SWPMI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPMI1RST_W<'a> {
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
///Reader of field `LPUART1RST`
pub type LPUART1RST_R = crate::R<bool, bool>;
///Write proxy for field `LPUART1RST`
pub struct LPUART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1RST_W<'a> {
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
    ///Bit 5 - Low-power timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 2 - Single wire protocol reset
    #[inline(always)]
    pub fn swpmi1rst(&self) -> SWPMI1RST_R {
        SWPMI1RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 0 - Low-power UART 1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 5 - Low-power timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W {
        LPTIM2RST_W { w: self }
    }
    ///Bit 2 - Single wire protocol reset
    #[inline(always)]
    pub fn swpmi1rst(&mut self) -> SWPMI1RST_W {
        SWPMI1RST_W { w: self }
    }
    ///Bit 0 - Low-power UART 1 reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W {
        LPUART1RST_W { w: self }
    }
}
