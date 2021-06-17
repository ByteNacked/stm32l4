///Reader of register CSR34
pub type R = crate::R<u32, super::CSR34>;
///Writer for register CSR34
pub type W = crate::W<u32, super::CSR34>;
///Register CSR34 `reset()`'s with value 0
impl crate::ResetValue for super::CSR34 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR34`
pub type CSR34_R = crate::R<u32, u32>;
///Write proxy for field `CSR34`
pub struct CSR34_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR34_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR34
    #[inline(always)]
    pub fn csr34(&self) -> CSR34_R {
        CSR34_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR34
    #[inline(always)]
    pub fn csr34(&mut self) -> CSR34_W {
        CSR34_W { w: self }
    }
}
