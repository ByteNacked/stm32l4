///Reader of register WUTR
pub type R = crate::R<u32, super::WUTR>;
///Writer for register WUTR
pub type W = crate::W<u32, super::WUTR>;
///Register WUTR `reset()`'s with value 0xffff
impl crate::ResetValue for super::WUTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
///Reader of field `WUT`
pub type WUT_R = crate::R<u16, u16>;
///Write proxy for field `WUT`
pub struct WUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Wakeup auto-reload value bits
    #[inline(always)]
    pub fn wut(&self) -> WUT_R {
        WUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Wakeup auto-reload value bits
    #[inline(always)]
    pub fn wut(&mut self) -> WUT_W {
        WUT_W { w: self }
    }
}
