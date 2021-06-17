///Reader of register CCR%s
pub type R = crate::R<u32, super::CCR>;
///Writer for register CCR%s
pub type W = crate::W<u32, super::CCR>;
///Register CCR%s `reset()`'s with value 0
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CCR1_H`
pub type CCR1_H_R = crate::R<u16, u16>;
///Write proxy for field `CCR1_H`
pub struct CCR1_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_H_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
///Reader of field `CCR`
pub type CCR_R = crate::R<u16, u16>;
///Write proxy for field `CCR`
pub struct CCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 16:31 - High Capture/Compare 1 value (TIM2 only)
    #[inline(always)]
    pub fn ccr1_h(&self) -> CCR1_H_R {
        CCR1_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    ///Bits 0:15 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - High Capture/Compare 1 value (TIM2 only)
    #[inline(always)]
    pub fn ccr1_h(&mut self) -> CCR1_H_W {
        CCR1_H_W { w: self }
    }
    ///Bits 0:15 - Capture/Compare 1 value
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W { w: self }
    }
}
