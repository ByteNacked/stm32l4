///Reader of register BRR
pub type R = crate::R<u32, super::BRR>;
///Writer for register BRR
pub type W = crate::W<u32, super::BRR>;
///Register BRR `reset()`'s with value 0
impl crate::ResetValue for super::BRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `BRR`
pub type BRR_R = crate::R<u16, u16>;
///Write proxy for field `BRR`
pub struct BRR_W<'a> {
    w: &'a mut W,
}
impl<'a> BRR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - USARTDIV
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - USARTDIV
    #[inline(always)]
    pub fn brr(&mut self) -> BRR_W {
        BRR_W { w: self }
    }
}
