///Reader of register CSR22
pub type R = crate::R<u32, super::CSR22>;
///Writer for register CSR22
pub type W = crate::W<u32, super::CSR22>;
///Register CSR22 `reset()`'s with value 0
impl crate::ResetValue for super::CSR22 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR22`
pub type CSR22_R = crate::R<u32, u32>;
///Write proxy for field `CSR22`
pub struct CSR22_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR22_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR22
    #[inline(always)]
    pub fn csr22(&self) -> CSR22_R {
        CSR22_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR22
    #[inline(always)]
    pub fn csr22(&mut self) -> CSR22_W {
        CSR22_W { w: self }
    }
}
