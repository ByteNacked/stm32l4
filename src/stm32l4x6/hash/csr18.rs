///Reader of register CSR18
pub type R = crate::R<u32, super::CSR18>;
///Writer for register CSR18
pub type W = crate::W<u32, super::CSR18>;
///Register CSR18 `reset()`'s with value 0
impl crate::ResetValue for super::CSR18 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR18`
pub type CSR18_R = crate::R<u32, u32>;
///Write proxy for field `CSR18`
pub struct CSR18_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR18_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR18
    #[inline(always)]
    pub fn csr18(&self) -> CSR18_R {
        CSR18_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR18
    #[inline(always)]
    pub fn csr18(&mut self) -> CSR18_W {
        CSR18_W { w: self }
    }
}
