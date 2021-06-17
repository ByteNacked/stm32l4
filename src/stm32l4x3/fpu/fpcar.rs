///Reader of register FPCAR
pub type R = crate::R<u32, super::FPCAR>;
///Writer for register FPCAR
pub type W = crate::W<u32, super::FPCAR>;
///Register FPCAR `reset()`'s with value 0
impl crate::ResetValue for super::FPCAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ADDRESS`
pub type ADDRESS_R = crate::R<u32, u32>;
///Write proxy for field `ADDRESS`
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    ///Bits 3:31 - Location of unpopulated floating-point
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    ///Bits 3:31 - Location of unpopulated floating-point
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
}
