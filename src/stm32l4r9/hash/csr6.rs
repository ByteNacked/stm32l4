///Reader of register CSR6
pub type R = crate::R<u32, super::CSR6>;
///Writer for register CSR6
pub type W = crate::W<u32, super::CSR6>;
///Register CSR6 `reset()`'s with value 0
impl crate::ResetValue for super::CSR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR6`
pub type CSR6_R = crate::R<u32, u32>;
///Write proxy for field `CSR6`
pub struct CSR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR6_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR6
    #[inline(always)]
    pub fn csr6(&self) -> CSR6_R {
        CSR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR6
    #[inline(always)]
    pub fn csr6(&mut self) -> CSR6_W {
        CSR6_W { w: self }
    }
}
