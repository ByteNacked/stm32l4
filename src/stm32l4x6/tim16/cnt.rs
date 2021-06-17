///Reader of register CNT
pub type R = crate::R<u32, super::CNT>;
///Writer for register CNT
pub type W = crate::W<u32, super::CNT>;
///Register CNT `reset()`'s with value 0
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CNT`
pub type CNT_R = crate::R<u16, u16>;
///Write proxy for field `CNT`
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `UIFCPY`
pub type UIFCPY_R = crate::R<bool, bool>;
impl R {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 31 - UIF Copy
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
