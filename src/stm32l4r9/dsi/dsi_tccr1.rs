///Reader of register DSI_TCCR1
pub type R = crate::R<u32, super::DSI_TCCR1>;
///Writer for register DSI_TCCR1
pub type W = crate::W<u32, super::DSI_TCCR1>;
///Register DSI_TCCR1 `reset()`'s with value 0
impl crate::ResetValue for super::DSI_TCCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `HSRD_TOCNT`
pub type HSRD_TOCNT_R = crate::R<u16, u16>;
///Write proxy for field `HSRD_TOCNT`
pub struct HSRD_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRD_TOCNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - High-Speed Read Timeout Counter
    #[inline(always)]
    pub fn hsrd_tocnt(&self) -> HSRD_TOCNT_R {
        HSRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - High-Speed Read Timeout Counter
    #[inline(always)]
    pub fn hsrd_tocnt(&mut self) -> HSRD_TOCNT_W {
        HSRD_TOCNT_W { w: self }
    }
}
