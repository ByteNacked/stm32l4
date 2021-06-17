///Reader of register DOEPTSIZ0
pub type R = crate::R<u32, super::DOEPTSIZ0>;
///Writer for register DOEPTSIZ0
pub type W = crate::W<u32, super::DOEPTSIZ0>;
///Register DOEPTSIZ0 `reset()`'s with value 0
impl crate::ResetValue for super::DOEPTSIZ0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `STUPCNT`
pub type STUPCNT_R = crate::R<u8, u8>;
///Write proxy for field `STUPCNT`
pub struct STUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> STUPCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
///Reader of field `PKTCNT`
pub type PKTCNT_R = crate::R<bool, bool>;
///Write proxy for field `PKTCNT`
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Reader of field `XFRSIZ`
pub type XFRSIZ_R = crate::R<u8, u8>;
///Write proxy for field `XFRSIZ`
pub struct XFRSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRSIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    ///Bits 29:30 - SETUP packet count
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    ///Bit 19 - Packet count
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bits 0:6 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    ///Bits 29:30 - SETUP packet count
    #[inline(always)]
    pub fn stupcnt(&mut self) -> STUPCNT_W {
        STUPCNT_W { w: self }
    }
    ///Bit 19 - Packet count
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    ///Bits 0:6 - Transfer size
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W {
        XFRSIZ_W { w: self }
    }
}
