///Reader of register CSR46
pub type R = crate::R<u32, super::CSR46>;
///Writer for register CSR46
pub type W = crate::W<u32, super::CSR46>;
///Register CSR46 `reset()`'s with value 0
impl crate::ResetValue for super::CSR46 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR46`
pub type CSR46_R = crate::R<u32, u32>;
///Write proxy for field `CSR46`
pub struct CSR46_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR46_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR46
    #[inline(always)]
    pub fn csr46(&self) -> CSR46_R {
        CSR46_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR46
    #[inline(always)]
    pub fn csr46(&mut self) -> CSR46_W {
        CSR46_W { w: self }
    }
}
