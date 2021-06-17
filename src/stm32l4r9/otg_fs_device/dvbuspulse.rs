///Reader of register DVBUSPULSE
pub type R = crate::R<u32, super::DVBUSPULSE>;
///Writer for register DVBUSPULSE
pub type W = crate::W<u32, super::DVBUSPULSE>;
///Register DVBUSPULSE `reset()`'s with value 0x05b8
impl crate::ResetValue for super::DVBUSPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05b8
    }
}
///Reader of field `DVBUSP`
pub type DVBUSP_R = crate::R<u16, u16>;
///Write proxy for field `DVBUSP`
pub struct DVBUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DVBUSP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - Device VBUS pulsing time
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Device VBUS pulsing time
    #[inline(always)]
    pub fn dvbusp(&mut self) -> DVBUSP_W {
        DVBUSP_W { w: self }
    }
}
