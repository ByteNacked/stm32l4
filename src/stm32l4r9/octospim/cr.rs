///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MUXEN`
pub type MUXEN_R = crate::R<bool, bool>;
///Write proxy for field `MUXEN`
pub struct MUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `REQ2ACK_TIME`
pub type REQ2ACK_TIME_R = crate::R<u8, u8>;
///Write proxy for field `REQ2ACK_TIME`
pub struct REQ2ACK_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ2ACK_TIME_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    pub fn req2ack_time(&self) -> REQ2ACK_TIME_R {
        REQ2ACK_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Multiplexed mode enable
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W {
        MUXEN_W { w: self }
    }
    ///Bits 16:23 - REQ to ACK time
    #[inline(always)]
    pub fn req2ack_time(&mut self) -> REQ2ACK_TIME_W {
        REQ2ACK_TIME_W { w: self }
    }
}
