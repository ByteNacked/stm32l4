///Reader of register GRXFSIZ
pub type R = crate::R<u32, super::GRXFSIZ>;
///Writer for register GRXFSIZ
pub type W = crate::W<u32, super::GRXFSIZ>;
///Register GRXFSIZ `reset()`'s with value 0x0200
impl crate::ResetValue for super::GRXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
///Reader of field `RXFD`
pub type RXFD_R = crate::R<u16, u16>;
///Write proxy for field `RXFD`
pub struct RXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - RxFIFO depth
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - RxFIFO depth
    #[inline(always)]
    pub fn rxfd(&mut self) -> RXFD_W {
        RXFD_W { w: self }
    }
}
