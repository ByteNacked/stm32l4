///Reader of register CH7DATINR
pub type R = crate::R<u32, super::CH7DATINR>;
///Writer for register CH7DATINR
pub type W = crate::W<u32, super::CH7DATINR>;
///Register CH7DATINR `reset()`'s with value 0
impl crate::ResetValue for super::CH7DATINR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `INDAT1`
pub type INDAT1_R = crate::R<u16, u16>;
///Write proxy for field `INDAT1`
pub struct INDAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> INDAT1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
///Reader of field `INDAT0`
pub type INDAT0_R = crate::R<u16, u16>;
///Write proxy for field `INDAT0`
pub struct INDAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> INDAT0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 16:31 - INDAT1
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    ///Bits 0:15 - INDAT0
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - INDAT1
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W {
        INDAT1_W { w: self }
    }
    ///Bits 0:15 - INDAT0
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W {
        INDAT0_W { w: self }
    }
}
