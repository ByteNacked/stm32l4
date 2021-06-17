///Reader of register CSR15
pub type R = crate::R<u32, super::CSR15>;
///Writer for register CSR15
pub type W = crate::W<u32, super::CSR15>;
///Register CSR15 `reset()`'s with value 0
impl crate::ResetValue for super::CSR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR15`
pub type CSR15_R = crate::R<u32, u32>;
///Write proxy for field `CSR15`
pub struct CSR15_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR15_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR15
    #[inline(always)]
    pub fn csr15(&self) -> CSR15_R {
        CSR15_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR15
    #[inline(always)]
    pub fn csr15(&mut self) -> CSR15_W {
        CSR15_W { w: self }
    }
}
