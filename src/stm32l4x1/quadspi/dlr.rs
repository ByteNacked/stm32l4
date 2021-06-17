///Reader of register DLR
pub type R = crate::R<u32, super::DLR>;
///Writer for register DLR
pub type W = crate::W<u32, super::DLR>;
///Register DLR `reset()`'s with value 0
impl crate::ResetValue for super::DLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DL`
pub type DL_R = crate::R<u32, u32>;
///Write proxy for field `DL`
pub struct DL_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Data length
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Data length
    #[inline(always)]
    pub fn dl(&mut self) -> DL_W {
        DL_W { w: self }
    }
}
