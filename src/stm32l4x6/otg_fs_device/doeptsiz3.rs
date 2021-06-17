///Reader of register DOEPTSIZ3
pub type R = crate::R<u32, super::DOEPTSIZ3>;
///Writer for register DOEPTSIZ3
pub type W = crate::W<u32, super::DOEPTSIZ3>;
///Register DOEPTSIZ3 `reset()`'s with value 0
impl crate::ResetValue for super::DOEPTSIZ3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RXDPID_STUPCNT`
pub type RXDPID_STUPCNT_R = crate::R<u8, u8>;
///Write proxy for field `RXDPID_STUPCNT`
pub struct RXDPID_STUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDPID_STUPCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
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
impl R {
    ///Bits 29:30 - Received data PID/SETUP packet count
    #[inline(always)]
    pub fn rxdpid_stupcnt(&self) -> RXDPID_STUPCNT_R {
        RXDPID_STUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    ///Bits 29:30 - Received data PID/SETUP packet count
    #[inline(always)]
    pub fn rxdpid_stupcnt(&mut self) -> RXDPID_STUPCNT_W {
        RXDPID_STUPCNT_W { w: self }
    }
    ///Bits 19:28 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    ///Bits 0:18 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W {
        XFRSIZ_W { w: self }
    }
}
