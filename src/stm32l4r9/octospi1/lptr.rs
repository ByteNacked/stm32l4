///Reader of register LPTR
pub type R = crate::R<u32, super::LPTR>;
///Writer for register LPTR
pub type W = crate::W<u32, super::LPTR>;
///Register LPTR `reset()`'s with value 0
impl crate::ResetValue for super::LPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TIMEOUT`
pub type TIMEOUT_R = crate::R<u16, u16>;
///Write proxy for field `TIMEOUT`
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Timeout period
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timeout period
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
}
