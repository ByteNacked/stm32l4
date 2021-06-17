///Reader of register LOAD
pub type R = crate::R<u32, super::LOAD>;
///Writer for register LOAD
pub type W = crate::W<u32, super::LOAD>;
///Register LOAD `reset()`'s with value 0
impl crate::ResetValue for super::LOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `RELOAD`
pub type RELOAD_R = crate::R<u32, u32>;
///Write proxy for field `RELOAD`
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
