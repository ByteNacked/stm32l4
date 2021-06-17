///Reader of register FGCLUT
pub type R = crate::R<u32, super::FGCLUT>;
///Writer for register FGCLUT
pub type W = crate::W<u32, super::FGCLUT>;
///Register FGCLUT `reset()`'s with value 0
impl crate::ResetValue for super::FGCLUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `APLHA`
pub type APLHA_R = crate::R<u8, u8>;
///Write proxy for field `APLHA`
pub struct APLHA_W<'a> {
    w: &'a mut W,
}
impl<'a> APLHA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
///Reader of field `RED`
pub type RED_R = crate::R<u8, u8>;
///Write proxy for field `RED`
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `GREEN`
pub type GREEN_R = crate::R<u8, u8>;
///Write proxy for field `GREEN`
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `BLUE`
pub type BLUE_R = crate::R<u8, u8>;
///Write proxy for field `BLUE`
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 24:31 - APLHA
    #[inline(always)]
    pub fn aplha(&self) -> APLHA_R {
        APLHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:23 - RED
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 8:15 - GREEN
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - BLUE
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 24:31 - APLHA
    #[inline(always)]
    pub fn aplha(&mut self) -> APLHA_W {
        APLHA_W { w: self }
    }
    ///Bits 16:23 - RED
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    ///Bits 8:15 - GREEN
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    ///Bits 0:7 - BLUE
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
}
