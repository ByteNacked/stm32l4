///Reader of register EXTICR4
pub type R = crate::R<u32, super::EXTICR4>;
///Writer for register EXTICR4
pub type W = crate::W<u32, super::EXTICR4>;
///Register EXTICR4 `reset()`'s with value 0
impl crate::ResetValue for super::EXTICR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EXTI15`
pub type EXTI15_R = crate::R<u8, u8>;
///Write proxy for field `EXTI15`
pub struct EXTI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
///Reader of field `EXTI14`
pub type EXTI14_R = crate::R<u8, u8>;
///Write proxy for field `EXTI14`
pub struct EXTI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
///Reader of field `EXTI13`
pub type EXTI13_R = crate::R<u8, u8>;
///Write proxy for field `EXTI13`
pub struct EXTI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///Reader of field `EXTI12`
pub type EXTI12_R = crate::R<u8, u8>;
///Write proxy for field `EXTI12`
pub struct EXTI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 12:14 - EXTI15 configuration bits
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bits 8:10 - EXTI14 configuration bits
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 4:6 - EXTI13 configuration bits
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bits 0:2 - EXTI12 configuration bits
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 12:14 - EXTI15 configuration bits
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W {
        EXTI15_W { w: self }
    }
    ///Bits 8:10 - EXTI14 configuration bits
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W {
        EXTI14_W { w: self }
    }
    ///Bits 4:6 - EXTI13 configuration bits
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W {
        EXTI13_W { w: self }
    }
    ///Bits 0:2 - EXTI12 configuration bits
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W {
        EXTI12_W { w: self }
    }
}
