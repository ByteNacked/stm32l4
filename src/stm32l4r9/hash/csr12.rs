///Reader of register CSR12
pub type R = crate::R<u32, super::CSR12>;
///Writer for register CSR12
pub type W = crate::W<u32, super::CSR12>;
///Register CSR12 `reset()`'s with value 0
impl crate::ResetValue for super::CSR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR12`
pub type CSR12_R = crate::R<u32, u32>;
///Write proxy for field `CSR12`
pub struct CSR12_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR12_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR12
    #[inline(always)]
    pub fn csr12(&self) -> CSR12_R {
        CSR12_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR12
    #[inline(always)]
    pub fn csr12(&mut self) -> CSR12_W {
        CSR12_W { w: self }
    }
}
