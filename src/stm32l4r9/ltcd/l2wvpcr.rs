///Reader of register L2WVPCR
pub type R = crate::R<u32, super::L2WVPCR>;
///Writer for register L2WVPCR
pub type W = crate::W<u32, super::L2WVPCR>;
///Register L2WVPCR `reset()`'s with value 0
impl crate::ResetValue for super::L2WVPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `WVSTPOS`
pub type WVSTPOS_R = crate::R<u16, u16>;
///Write proxy for field `WVSTPOS`
pub struct WVSTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSTPOS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
///Reader of field `WVSPPOS`
pub type WVSPPOS_R = crate::R<u16, u16>;
///Write proxy for field `WVSPPOS`
pub struct WVSPPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSPPOS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:10 - Window Vertical Start Position
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - Window Vertical Stop Position
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Window Vertical Start Position
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W {
        WVSTPOS_W { w: self }
    }
    ///Bits 16:26 - Window Vertical Stop Position
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W {
        WVSPPOS_W { w: self }
    }
}
