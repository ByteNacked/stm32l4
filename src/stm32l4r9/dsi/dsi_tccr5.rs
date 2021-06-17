///Reader of register DSI_TCCR5
pub type R = crate::R<u32, super::DSI_TCCR5>;
///Writer for register DSI_TCCR5
pub type W = crate::W<u32, super::DSI_TCCR5>;
///Register DSI_TCCR5 `reset()`'s with value 0
impl crate::ResetValue for super::DSI_TCCR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `BTA_TOCNT`
pub type BTA_TOCNT_R = crate::R<u16, u16>;
///Write proxy for field `BTA_TOCNT`
pub struct BTA_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BTA_TOCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Bus-Turn-Around Timeout Counter
    #[inline(always)]
    pub fn bta_tocnt(&self) -> BTA_TOCNT_R {
        BTA_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Bus-Turn-Around Timeout Counter
    #[inline(always)]
    pub fn bta_tocnt(&mut self) -> BTA_TOCNT_W {
        BTA_TOCNT_W { w: self }
    }
}
