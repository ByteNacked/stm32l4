///Reader of register CSR38
pub type R = crate::R<u32, super::CSR38>;
///Writer for register CSR38
pub type W = crate::W<u32, super::CSR38>;
///Register CSR38 `reset()`'s with value 0
impl crate::ResetValue for super::CSR38 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR38`
pub type CSR38_R = crate::R<u32, u32>;
///Write proxy for field `CSR38`
pub struct CSR38_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR38_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR38
    #[inline(always)]
    pub fn csr38(&self) -> CSR38_R {
        CSR38_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR38
    #[inline(always)]
    pub fn csr38(&mut self) -> CSR38_W {
        CSR38_W { w: self }
    }
}
