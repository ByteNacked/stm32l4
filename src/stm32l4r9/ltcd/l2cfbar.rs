///Reader of register L2CFBAR
pub type R = crate::R<u32, super::L2CFBAR>;
///Writer for register L2CFBAR
pub type W = crate::W<u32, super::L2CFBAR>;
///Register L2CFBAR `reset()`'s with value 0
impl crate::ResetValue for super::L2CFBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CFBADD`
pub type CFBADD_R = crate::R<u32, u32>;
///Write proxy for field `CFBADD`
pub struct CFBADD_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Color Frame Buffer Start Address
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Color Frame Buffer Start Address
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CFBADD_W {
        CFBADD_W { w: self }
    }
}
