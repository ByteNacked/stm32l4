///Reader of register SHRR
pub type R = crate::R<u32, super::SHRR>;
///Writer for register SHRR
pub type W = crate::W<u32, super::SHRR>;
///Register SHRR `reset()`'s with value 0x01
impl crate::ResetValue for super::SHRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
///Reader of field `TREFRESH1`
pub type TREFRESH1_R = crate::R<u8, u8>;
///Write proxy for field `TREFRESH1`
pub struct TREFRESH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TREFRESH1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `TREFRESH2`
pub type TREFRESH2_R = crate::R<u8, u8>;
///Write proxy for field `TREFRESH2`
pub struct TREFRESH2_W<'a> {
    w: &'a mut W,
}
impl<'a> TREFRESH2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:7 - DAC Channel 1 refresh Time
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - DAC Channel 2 refresh Time
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH2_R {
        TREFRESH2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - DAC Channel 1 refresh Time
    #[inline(always)]
    pub fn trefresh1(&mut self) -> TREFRESH1_W {
        TREFRESH1_W { w: self }
    }
    ///Bits 16:23 - DAC Channel 2 refresh Time
    #[inline(always)]
    pub fn trefresh2(&mut self) -> TREFRESH2_W {
        TREFRESH2_W { w: self }
    }
}
