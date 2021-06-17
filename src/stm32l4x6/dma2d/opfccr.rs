///Reader of register OPFCCR
pub type R = crate::R<u32, super::OPFCCR>;
///Writer for register OPFCCR
pub type W = crate::W<u32, super::OPFCCR>;
///Register OPFCCR `reset()`'s with value 0
impl crate::ResetValue for super::OPFCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Color mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    ///0: ARGB8888
    ARGB8888 = 0,
    ///1: RGB888
    RGB888 = 1,
    ///2: RGB565
    RGB565 = 2,
    ///3: ARGB1555
    ARGB1555 = 3,
    ///4: ARGB4444
    ARGB4444 = 4,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
///Reader of field `CM`
pub type CM_R = crate::R<u8, CM_A>;
impl CM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CM_A::ARGB8888),
            1 => Val(CM_A::RGB888),
            2 => Val(CM_A::RGB565),
            3 => Val(CM_A::ARGB1555),
            4 => Val(CM_A::ARGB4444),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `ARGB8888`
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM_A::ARGB8888
    }
    ///Checks if the value of the field is `RGB888`
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM_A::RGB888
    }
    ///Checks if the value of the field is `RGB565`
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM_A::RGB565
    }
    ///Checks if the value of the field is `ARGB1555`
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM_A::ARGB1555
    }
    ///Checks if the value of the field is `ARGB4444`
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM_A::ARGB4444
    }
}
///Write proxy for field `CM`
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CM_A::ARGB8888)
    }
    ///RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CM_A::RGB888)
    }
    ///RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(CM_A::RGB565)
    }
    ///ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(CM_A::ARGB1555)
    }
    ///ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(CM_A::ARGB4444)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Reader of field `RBS`
pub type RBS_R = crate::R<bool, bool>;
///Write proxy for field `RBS`
pub struct RBS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBS_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
///Reader of field `AI`
pub type AI_R = crate::R<bool, bool>;
///Write proxy for field `AI`
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 21 - Red Blue Swap
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - Alpha Inverted
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - Color mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    ///Bit 21 - Red Blue Swap
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W {
        RBS_W { w: self }
    }
    ///Bit 20 - Alpha Inverted
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W { w: self }
    }
}
