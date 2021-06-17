///Reader of register MCR
pub type R = crate::R<u32, super::MCR>;
///Writer for register MCR
pub type W = crate::W<u32, super::MCR>;
///Register MCR `reset()`'s with value 0
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `MODE1`
pub type MODE1_R = crate::R<u8, u8>;
///Write proxy for field `MODE1`
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Reader of field `MODE2`
pub type MODE2_R = crate::R<u8, u8>;
///Write proxy for field `MODE2`
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 16:18 - DAC Channel 2 mode
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - DAC Channel 1 mode
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    ///Bits 16:18 - DAC Channel 2 mode
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
}
