///Reader of register ABR
pub type R = crate::R<u32, super::ABR>;
///Writer for register ABR
pub type W = crate::W<u32, super::ABR>;
///Register ABR `reset()`'s with value 0
impl crate::ResetValue for super::ABR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ALTERNATE`
pub type ALTERNATE_R = crate::R<u32, u32>;
///Write proxy for field `ALTERNATE`
pub struct ALTERNATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTERNATE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    pub fn alternate(&mut self) -> ALTERNATE_W {
        ALTERNATE_W { w: self }
    }
}
