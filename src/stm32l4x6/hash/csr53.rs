///Reader of register CSR53
pub type R = crate::R<u32, super::CSR53>;
///Writer for register CSR53
pub type W = crate::W<u32, super::CSR53>;
///Register CSR53 `reset()`'s with value 0
impl crate::ResetValue for super::CSR53 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR53`
pub type CSR53_R = crate::R<u32, u32>;
///Write proxy for field `CSR53`
pub struct CSR53_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR53_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR53
    #[inline(always)]
    pub fn csr53(&self) -> CSR53_R {
        CSR53_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR53
    #[inline(always)]
    pub fn csr53(&mut self) -> CSR53_W {
        CSR53_W { w: self }
    }
}
