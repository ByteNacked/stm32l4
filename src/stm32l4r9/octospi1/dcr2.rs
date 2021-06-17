///Reader of register DCR2
pub type R = crate::R<u32, super::DCR2>;
///Writer for register DCR2
pub type W = crate::W<u32, super::DCR2>;
///Register DCR2 `reset()`'s with value 0
impl crate::ResetValue for super::DCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PRESCALER`
pub type PRESCALER_R = crate::R<u8, u8>;
///Write proxy for field `PRESCALER`
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `WRAPSIZE`
pub type WRAPSIZE_R = crate::R<u8, u8>;
///Write proxy for field `WRAPSIZE`
pub struct WRAPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAPSIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    pub fn wrapsize(&self) -> WRAPSIZE_R {
        WRAPSIZE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:7 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    ///Bits 16:18 - Wrap size
    #[inline(always)]
    pub fn wrapsize(&mut self) -> WRAPSIZE_W {
        WRAPSIZE_W { w: self }
    }
}
