///Reader of register CSR20
pub type R = crate::R<u32, super::CSR20>;
///Writer for register CSR20
pub type W = crate::W<u32, super::CSR20>;
///Register CSR20 `reset()`'s with value 0
impl crate::ResetValue for super::CSR20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CSR20`
pub type CSR20_R = crate::R<u32, u32>;
///Write proxy for field `CSR20`
pub struct CSR20_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR20_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - CSR20
    #[inline(always)]
    pub fn csr20(&self) -> CSR20_R {
        CSR20_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - CSR20
    #[inline(always)]
    pub fn csr20(&mut self) -> CSR20_W {
        CSR20_W { w: self }
    }
}
