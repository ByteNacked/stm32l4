///Reader of register OR1
pub type R = crate::R<u32, super::OR1>;
///Writer for register OR1
pub type W = crate::W<u32, super::OR1>;
///Register OR1 `reset()`'s with value 0
impl crate::ResetValue for super::OR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TI1_RMP`
pub type TI1_RMP_R = crate::R<u8, u8>;
///Write proxy for field `TI1_RMP`
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Input capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - Input capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
}
