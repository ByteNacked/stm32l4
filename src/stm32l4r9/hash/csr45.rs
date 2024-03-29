///Reader of register CSR45
pub type R = crate::R<u32, super::CSR45>;
///Writer for register CSR45
pub type W = crate::W<u32, super::CSR45>;
///Register CSR45 `reset()`'s with value 0
impl crate::ResetValue for super::CSR45 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR45`
pub type CSR45_R = crate::R<u32, u32>;
///Write proxy for field `CSR45`
pub struct CSR45_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR45_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR45
    #[inline(always)]
    pub fn csr45(&self) -> CSR45_R {
        CSR45_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR45
    #[inline(always)]
    pub fn csr45(&mut self) -> CSR45_W {
        CSR45_W { w: self }
    }
}
