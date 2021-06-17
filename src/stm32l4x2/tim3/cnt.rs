///Reader of register CNT
pub type R = crate::R<u32, super::CNT>;
///Writer for register CNT
pub type W = crate::W<u32, super::CNT>;
///Register CNT `reset()`'s with value 0
impl crate::ResetValue for super::CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CNT_H`
pub type CNT_H_R = crate::R<u16, u16>;
///Write proxy for field `CNT_H`
pub struct CNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_H_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
///Reader of field `CNT`
pub type CNT_R = crate::R<u16, u16>;
///Write proxy for field `CNT`
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 16:31 - High counter value (TIM2 only)
    #[inline(always)]
    pub fn cnt_h(&self) -> CNT_H_R {
        CNT_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - High counter value (TIM2 only)
    #[inline(always)]
    pub fn cnt_h(&mut self) -> CNT_H_W {
        CNT_H_W { w: self }
    }
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
