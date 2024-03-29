///Reader of register DSI_DLTRC
pub type R = crate::R<u32, super::DSI_DLTRC>;
///Writer for register DSI_DLTRC
pub type W = crate::W<u32, super::DSI_DLTRC>;
///Register DSI_DLTRC `reset()`'s with value 0
impl crate::ResetValue for super::DSI_DLTRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MRD_TIME`
pub type MRD_TIME_R = crate::R<u16, u16>;
///Write proxy for field `MRD_TIME`
pub struct MRD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_TIME_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
///Reader of field `LP2HS_TIME`
pub type LP2HS_TIME_R = crate::R<u8, u8>;
///Write proxy for field `LP2HS_TIME`
pub struct LP2HS_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> LP2HS_TIME_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `HS2LP_TIME`
pub type HS2LP_TIME_R = crate::R<u8, u8>;
///Write proxy for field `HS2LP_TIME`
pub struct HS2LP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> HS2LP_TIME_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    ///Bits 0:14 - Maximum Read Time
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 16:23 - Low-Power To High-Speed Time
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - High-Speed To Low-Power Time
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:14 - Maximum Read Time
    #[inline(always)]
    pub fn mrd_time(&mut self) -> MRD_TIME_W {
        MRD_TIME_W { w: self }
    }
    ///Bits 16:23 - Low-Power To High-Speed Time
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W {
        LP2HS_TIME_W { w: self }
    }
    ///Bits 24:31 - High-Speed To Low-Power Time
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W {
        HS2LP_TIME_W { w: self }
    }
}
