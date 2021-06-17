///Reader of register HFIR
pub type R = crate::R<u32, super::HFIR>;
///Writer for register HFIR
pub type W = crate::W<u32, super::HFIR>;
///Register HFIR `reset()`'s with value 0xea60
impl crate::ResetValue for super::HFIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xea60
    }
}
///Reader of field `FRIVL`
pub type FRIVL_R = crate::R<u16, u16>;
///Write proxy for field `FRIVL`
pub struct FRIVL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRIVL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Frame interval
    #[inline(always)]
    pub fn frivl(&self) -> FRIVL_R {
        FRIVL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Frame interval
    #[inline(always)]
    pub fn frivl(&mut self) -> FRIVL_W {
        FRIVL_W { w: self }
    }
}
