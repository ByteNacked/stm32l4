///Writer for register PDKEYR
pub type W = crate::W<u32, super::PDKEYR>;
///Register PDKEYR `reset()`'s with value 0
impl crate::ResetValue for super::PDKEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `PDKEYR`
pub struct PDKEYR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDKEYR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    ///Bits 0:31 - RUN_PD in FLASH_ACR key
    #[inline(always)]
    pub fn pdkeyr(&mut self) -> PDKEYR_W {
        PDKEYR_W { w: self }
    }
}
