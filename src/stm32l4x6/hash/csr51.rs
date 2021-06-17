///Reader of register CSR51
pub type R = crate::R<u32, super::CSR51>;
///Writer for register CSR51
pub type W = crate::W<u32, super::CSR51>;
///Register CSR51 `reset()`'s with value 0
impl crate::ResetValue for super::CSR51 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR51`
pub type CSR51_R = crate::R<u32, u32>;
///Write proxy for field `CSR51`
pub struct CSR51_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR51_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR51
    #[inline(always)]
    pub fn csr51(&self) -> CSR51_R {
        CSR51_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR51
    #[inline(always)]
    pub fn csr51(&mut self) -> CSR51_W {
        CSR51_W { w: self }
    }
}
