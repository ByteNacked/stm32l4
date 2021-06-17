///Reader of register PCROP1ER
pub type R = crate::R<u32, super::PCROP1ER>;
///Writer for register PCROP1ER
pub type W = crate::W<u32, super::PCROP1ER>;
///Register PCROP1ER `reset()`'s with value 0x0fff_0000
impl crate::ResetValue for super::PCROP1ER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
///Reader of field `PCROP1_END`
pub type PCROP1_END_R = crate::R<u16, u16>;
///Write proxy for field `PCROP1_END`
pub struct PCROP1_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1_END_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `PCROP_RDP`
pub type PCROP_RDP_R = crate::R<bool, bool>;
///Write proxy for field `PCROP_RDP`
pub struct PCROP_RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP_RDP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Bank 1 PCROP area end offset
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - Bank 1 PCROP area end offset
    #[inline(always)]
    pub fn pcrop1_end(&mut self) -> PCROP1_END_W {
        PCROP1_END_W { w: self }
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W {
        PCROP_RDP_W { w: self }
    }
}
