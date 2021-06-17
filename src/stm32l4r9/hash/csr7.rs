///Reader of register CSR7
pub type R = crate::R<u32, super::CSR7>;
///Writer for register CSR7
pub type W = crate::W<u32, super::CSR7>;
///Register CSR7 `reset()`'s with value 0
impl crate::ResetValue for super::CSR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR7`
pub type CSR7_R = crate::R<u32, u32>;
///Write proxy for field `CSR7`
pub struct CSR7_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR7_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR7
    #[inline(always)]
    pub fn csr7(&self) -> CSR7_R {
        CSR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR7
    #[inline(always)]
    pub fn csr7(&mut self) -> CSR7_W {
        CSR7_W { w: self }
    }
}
