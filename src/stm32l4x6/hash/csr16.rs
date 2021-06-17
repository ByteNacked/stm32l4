///Reader of register CSR16
pub type R = crate::R<u32, super::CSR16>;
///Writer for register CSR16
pub type W = crate::W<u32, super::CSR16>;
///Register CSR16 `reset()`'s with value 0
impl crate::ResetValue for super::CSR16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR16`
pub type CSR16_R = crate::R<u32, u32>;
///Write proxy for field `CSR16`
pub struct CSR16_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR16_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR16
    #[inline(always)]
    pub fn csr16(&self) -> CSR16_R {
        CSR16_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR16
    #[inline(always)]
    pub fn csr16(&mut self) -> CSR16_W {
        CSR16_W { w: self }
    }
}
