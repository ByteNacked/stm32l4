///Reader of register CH5DLYR
pub type R = crate::R<u32, super::CH5DLYR>;
///Writer for register CH5DLYR
pub type W = crate::W<u32, super::CH5DLYR>;
///Register CH5DLYR `reset()`'s with value 0
impl crate::ResetValue for super::CH5DLYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PLSSKP`
pub type PLSSKP_R = crate::R<u8, u8>;
///Write proxy for field `PLSSKP`
pub struct PLSSKP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSSKP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W {
        PLSSKP_W { w: self }
    }
}
