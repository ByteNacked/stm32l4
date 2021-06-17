///Reader of register WRP2BR
pub type R = crate::R<u32, super::WRP2BR>;
///Writer for register WRP2BR
pub type W = crate::W<u32, super::WRP2BR>;
///Register WRP2BR `reset()`'s with value 0xff00_ff00
impl crate::ResetValue for super::WRP2BR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff00_ff00
    }
}
///Reader of field `WRP2B_STRT`
pub type WRP2B_STRT_R = crate::R<u8, u8>;
///Write proxy for field `WRP2B_STRT`
pub struct WRP2B_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2B_STRT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `WRP2B_END`
pub type WRP2B_END_R = crate::R<u8, u8>;
///Write proxy for field `WRP2B_END`
pub struct WRP2B_END_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2B_END_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Bank 2 WRP second area B start offset
    #[inline(always)]
    pub fn wrp2b_strt(&self) -> WRP2B_STRT_R {
        WRP2B_STRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Bank 2 WRP second area B end offset
    #[inline(always)]
    pub fn wrp2b_end(&self) -> WRP2B_END_R {
        WRP2B_END_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Bank 2 WRP second area B start offset
    #[inline(always)]
    pub fn wrp2b_strt(&mut self) -> WRP2B_STRT_W {
        WRP2B_STRT_W { w: self }
    }
    ///Bits 16:23 - Bank 2 WRP second area B end offset
    #[inline(always)]
    pub fn wrp2b_end(&mut self) -> WRP2B_END_W {
        WRP2B_END_W { w: self }
    }
}
