///Reader of register HCTSIZ7
pub type R = crate::R<u32, super::HCTSIZ7>;
///Writer for register HCTSIZ7
pub type W = crate::W<u32, super::HCTSIZ7>;
///Register HCTSIZ7 `reset()`'s with value 0
impl crate::ResetValue for super::HCTSIZ7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `XFRSIZ`
pub type XFRSIZ_R = crate::R<u32, u32>;
///Write proxy for field `XFRSIZ`
pub struct XFRSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRSIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
///Reader of field `PKTCNT`
pub type PKTCNT_R = crate::R<u16, u16>;
///Write proxy for field `PKTCNT`
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
///Reader of field `DPID`
pub type DPID_R = crate::R<u8, u8>;
///Write proxy for field `DPID`
pub struct DPID_W<'a> {
    w: &'a mut W,
}
impl<'a> DPID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x0007_ffff) as u32)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 29:30 - Data PID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W {
        XFRSIZ_W { w: self }
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    ///Bits 29:30 - Data PID
    #[inline(always)]
    pub fn dpid(&mut self) -> DPID_W {
        DPID_W { w: self }
    }
}
