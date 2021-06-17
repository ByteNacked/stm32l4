///Reader of register DSI_TCCR0
pub type R = crate::R<u32, super::DSI_TCCR0>;
///Writer for register DSI_TCCR0
pub type W = crate::W<u32, super::DSI_TCCR0>;
///Register DSI_TCCR0 `reset()`'s with value 0
impl crate::ResetValue for super::DSI_TCCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LPRX_TOCNT`
pub type LPRX_TOCNT_R = crate::R<u16, u16>;
///Write proxy for field `LPRX_TOCNT`
pub struct LPRX_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRX_TOCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `HSTX_TOCNT`
pub type HSTX_TOCNT_R = crate::R<u16, u16>;
///Write proxy for field `HSTX_TOCNT`
pub struct HSTX_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTX_TOCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Low-power Reception Timeout Counter
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High-Speed Transmission Timeout Counter
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Low-power Reception Timeout Counter
    #[inline(always)]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W {
        LPRX_TOCNT_W { w: self }
    }
    ///Bits 16:31 - High-Speed Transmission Timeout Counter
    #[inline(always)]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W {
        HSTX_TOCNT_W { w: self }
    }
}
