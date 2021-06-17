///Reader of register CSR21
pub type R = crate::R<u32, super::CSR21>;
///Writer for register CSR21
pub type W = crate::W<u32, super::CSR21>;
///Register CSR21 `reset()`'s with value 0
impl crate::ResetValue for super::CSR21 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR21`
pub type CSR21_R = crate::R<u32, u32>;
///Write proxy for field `CSR21`
pub struct CSR21_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR21_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR21
    #[inline(always)]
    pub fn csr21(&self) -> CSR21_R {
        CSR21_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR21
    #[inline(always)]
    pub fn csr21(&mut self) -> CSR21_W {
        CSR21_W { w: self }
    }
}
