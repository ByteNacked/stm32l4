///Reader of register OR
pub type R = crate::R<u32, super::OR>;
///Writer for register OR
pub type W = crate::W<u32, super::OR>;
///Register OR `reset()`'s with value 0
impl crate::ResetValue for super::OR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ETR_RMP`
pub type ETR_RMP_R = crate::R<u8, u8>;
///Write proxy for field `ETR_RMP`
pub struct ETR_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Reader of field `TI4_RMP`
pub type TI4_RMP_R = crate::R<u8, u8>;
///Write proxy for field `TI4_RMP`
pub struct TI4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4_RMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W {
        ETR_RMP_W { w: self }
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W {
        TI4_RMP_W { w: self }
    }
}
