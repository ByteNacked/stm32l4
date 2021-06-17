///Reader of register WINR
pub type R = crate::R<u32, super::WINR>;
///Writer for register WINR
pub type W = crate::W<u32, super::WINR>;
///Register WINR `reset()`'s with value 0x0fff
impl crate::ResetValue for super::WINR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
///Reader of field `WIN`
pub type WIN_R = crate::R<u16, u16>;
///Write proxy for field `WIN`
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - Watchdog counter window value
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Watchdog counter window value
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
}
