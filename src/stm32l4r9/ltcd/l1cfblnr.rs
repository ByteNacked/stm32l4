///Reader of register L1CFBLNR
pub type R = crate::R<u32, super::L1CFBLNR>;
///Writer for register L1CFBLNR
pub type W = crate::W<u32, super::L1CFBLNR>;
///Register L1CFBLNR `reset()`'s with value 0
impl crate::ResetValue for super::L1CFBLNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CFBLNBR`
pub type CFBLNBR_R = crate::R<u16, u16>;
///Write proxy for field `CFBLNBR`
pub struct CFBLNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBLNBR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    ///Bits 0:10 - Frame Buffer Line Number
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Frame Buffer Line Number
    #[inline(always)]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W {
        CFBLNBR_W { w: self }
    }
}
