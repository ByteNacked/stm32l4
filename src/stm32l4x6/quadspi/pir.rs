///Reader of register PIR
pub type R = crate::R<u32, super::PIR>;
///Writer for register PIR
pub type W = crate::W<u32, super::PIR>;
///Register PIR `reset()`'s with value 0
impl crate::ResetValue for super::PIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `INTERVAL`
pub type INTERVAL_R = crate::R<u16, u16>;
///Write proxy for field `INTERVAL`
pub struct INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERVAL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Polling interval
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Polling interval
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W {
        INTERVAL_W { w: self }
    }
}
