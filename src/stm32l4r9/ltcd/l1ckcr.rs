///Reader of register L1CKCR
pub type R = crate::R<u32, super::L1CKCR>;
///Writer for register L1CKCR
pub type W = crate::W<u32, super::L1CKCR>;
///Register L1CKCR `reset()`'s with value 0
impl crate::ResetValue for super::L1CKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CKBLUE`
pub type CKBLUE_R = crate::R<u8, u8>;
///Write proxy for field `CKBLUE`
pub struct CKBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKBLUE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `CKGREEN`
pub type CKGREEN_R = crate::R<u8, u8>;
///Write proxy for field `CKGREEN`
pub struct CKGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKGREEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `CKRED`
pub type CKRED_R = crate::R<u8, u8>;
///Write proxy for field `CKRED`
pub struct CKRED_W<'a> {
    w: &'a mut W,
}
impl<'a> CKRED_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Color Key Green value
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Color Key Red value
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Color Key Blue value
    #[inline(always)]
    pub fn ckblue(&mut self) -> CKBLUE_W {
        CKBLUE_W { w: self }
    }
    ///Bits 8:15 - Color Key Green value
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CKGREEN_W {
        CKGREEN_W { w: self }
    }
    ///Bits 16:23 - Color Key Red value
    #[inline(always)]
    pub fn ckred(&mut self) -> CKRED_W {
        CKRED_W { w: self }
    }
}
