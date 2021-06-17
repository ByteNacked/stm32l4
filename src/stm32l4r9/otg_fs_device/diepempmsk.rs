///Reader of register DIEPEMPMSK
pub type R = crate::R<u32, super::DIEPEMPMSK>;
///Writer for register DIEPEMPMSK
pub type W = crate::W<u32, super::DIEPEMPMSK>;
///Register DIEPEMPMSK `reset()`'s with value 0
impl crate::ResetValue for super::DIEPEMPMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `INEPTXFEM`
pub type INEPTXFEM_R = crate::R<u16, u16>;
///Write proxy for field `INEPTXFEM`
pub struct INEPTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXFEM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits
    #[inline(always)]
    pub fn ineptxfem(&self) -> INEPTXFEM_R {
        INEPTXFEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits
    #[inline(always)]
    pub fn ineptxfem(&mut self) -> INEPTXFEM_W {
        INEPTXFEM_W { w: self }
    }
}
