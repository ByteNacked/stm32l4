///Reader of register CSR0
pub type R = crate::R<u32, super::CSR0>;
///Writer for register CSR0
pub type W = crate::W<u32, super::CSR0>;
///Register CSR0 `reset()`'s with value 0
impl crate::ResetValue for super::CSR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR0`
pub type CSR0_R = crate::R<u32, u32>;
///Write proxy for field `CSR0`
pub struct CSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr0(&self) -> CSR0_R {
        CSR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR0
    #[inline(always)]
    pub fn csr0(&mut self) -> CSR0_W {
        CSR0_W { w: self }
    }
}
