///Reader of register ESCR
pub type R = crate::R<u32, super::ESCR>;
///Writer for register ESCR
pub type W = crate::W<u32, super::ESCR>;
///Register ESCR `reset()`'s with value 0
impl crate::ResetValue for super::ESCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FEC`
pub type FEC_R = crate::R<u8, u8>;
///Write proxy for field `FEC`
pub struct FEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
///Reader of field `LEC`
pub type LEC_R = crate::R<u8, u8>;
///Write proxy for field `LEC`
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `LSC`
pub type LSC_R = crate::R<u8, u8>;
///Write proxy for field `LSC`
pub struct LSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `FSC`
pub type FSC_R = crate::R<u8, u8>;
///Write proxy for field `FSC`
pub struct FSC_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 24:31 - Frame end delimiter code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:23 - Line end delimiter code
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 8:15 - Line start delimiter code
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - Frame start delimiter code
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 24:31 - Frame end delimiter code
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W {
        FEC_W { w: self }
    }
    ///Bits 16:23 - Line end delimiter code
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    ///Bits 8:15 - Line start delimiter code
    #[inline(always)]
    pub fn lsc(&mut self) -> LSC_W {
        LSC_W { w: self }
    }
    ///Bits 0:7 - Frame start delimiter code
    #[inline(always)]
    pub fn fsc(&mut self) -> FSC_W {
        FSC_W { w: self }
    }
}
