///Reader of register CSR29
pub type R = crate::R<u32, super::CSR29>;
///Writer for register CSR29
pub type W = crate::W<u32, super::CSR29>;
///Register CSR29 `reset()`'s with value 0
impl crate::ResetValue for super::CSR29 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR29`
pub type CSR29_R = crate::R<u32, u32>;
///Write proxy for field `CSR29`
pub struct CSR29_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR29_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR29
    #[inline(always)]
    pub fn csr29(&self) -> CSR29_R {
        CSR29_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR29
    #[inline(always)]
    pub fn csr29(&mut self) -> CSR29_W {
        CSR29_W { w: self }
    }
}
