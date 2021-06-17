///Reader of register CPACR
pub type R = crate::R<u32, super::CPACR>;
///Writer for register CPACR
pub type W = crate::W<u32, super::CPACR>;
///Register CPACR `reset()`'s with value 0
impl crate::ResetValue for super::CPACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CP`
pub type CP_R = crate::R<u8, u8>;
///Write proxy for field `CP`
pub struct CP_W<'a> {
    w: &'a mut W,
}
impl<'a> CP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    ///Bits 20:23 - CP
    #[inline(always)]
    pub fn cp(&self) -> CP_R {
        CP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 20:23 - CP
    #[inline(always)]
    pub fn cp(&mut self) -> CP_W {
        CP_W { w: self }
    }
}
