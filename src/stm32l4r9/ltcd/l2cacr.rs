///Reader of register L2CACR
pub type R = crate::R<u32, super::L2CACR>;
///Writer for register L2CACR
pub type W = crate::W<u32, super::L2CACR>;
///Register L2CACR `reset()`'s with value 0
impl crate::ResetValue for super::L2CACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CONSTA`
pub type CONSTA_R = crate::R<u8, u8>;
///Write proxy for field `CONSTA`
pub struct CONSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Constant Alpha
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Constant Alpha
    #[inline(always)]
    pub fn consta(&mut self) -> CONSTA_W {
        CONSTA_W { w: self }
    }
}
