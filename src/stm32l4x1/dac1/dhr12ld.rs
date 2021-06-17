///Reader of register DHR12LD
pub type R = crate::R<u32, super::DHR12LD>;
///Writer for register DHR12LD
pub type W = crate::W<u32, super::DHR12LD>;
///Register DHR12LD `reset()`'s with value 0
impl crate::ResetValue for super::DHR12LD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DACC1DHR`
pub type DACC1DHR_R = crate::R<u16, u16>;
///Write proxy for field `DACC1DHR`
pub struct DACC1DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
///Reader of field `DACC2DHR`
pub type DACC2DHR_R = crate::R<u16, u16>;
///Write proxy for field `DACC2DHR`
pub struct DACC2DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    ///Bits 20:31 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W { w: self }
    }
    ///Bits 20:31 - DAC channel2 12-bit left-aligned data
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W { w: self }
    }
}
