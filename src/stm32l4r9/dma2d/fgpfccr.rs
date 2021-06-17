///Reader of register FGPFCCR
pub type R = crate::R<u32, super::FGPFCCR>;
///Writer for register FGPFCCR
pub type W = crate::W<u32, super::FGPFCCR>;
///Register FGPFCCR `reset()`'s with value 0
impl crate::ResetValue for super::FGPFCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ALPHA`
pub type ALPHA_R = crate::R<u8, u8>;
///Write proxy for field `ALPHA`
pub struct ALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> ALPHA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
///Alpha mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AM_A {
    ///0: No modification of alpha channel
    NOMODIFY = 0,
    ///1: Replace with value in ALPHA\[7:0\]
    REPLACE = 1,
    ///2: Multiply with value in ALPHA\[7:0\]
    MULTIPLY = 2,
}
impl From<AM_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_A) -> Self {
        variant as _
    }
}
///Reader of field `AM`
pub type AM_R = crate::R<u8, AM_A>;
impl AM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AM_A::NOMODIFY),
            1 => Val(AM_A::REPLACE),
            2 => Val(AM_A::MULTIPLY),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `NOMODIFY`
    #[inline(always)]
    pub fn is_no_modify(&self) -> bool {
        *self == AM_A::NOMODIFY
    }
    ///Checks if the value of the field is `REPLACE`
    #[inline(always)]
    pub fn is_replace(&self) -> bool {
        *self == AM_A::REPLACE
    }
    ///Checks if the value of the field is `MULTIPLY`
    #[inline(always)]
    pub fn is_multiply(&self) -> bool {
        *self == AM_A::MULTIPLY
    }
}
///Write proxy for field `AM`
pub struct AM_W<'a> {
    w: &'a mut W,
}
impl<'a> AM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No modification of alpha channel
    #[inline(always)]
    pub fn no_modify(self) -> &'a mut W {
        self.variant(AM_A::NOMODIFY)
    }
    ///Replace with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn replace(self) -> &'a mut W {
        self.variant(AM_A::REPLACE)
    }
    ///Multiply with value in ALPHA\[7:0\]
    #[inline(always)]
    pub fn multiply(self) -> &'a mut W {
        self.variant(AM_A::MULTIPLY)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Reader of field `CS`
pub type CS_R = crate::R<u8, u8>;
///Write proxy for field `CS`
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    ///1: Start the automatic loading of the CLUT
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `START`
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, START_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(START_A::START),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `START`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
///Write proxy for field `START`
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Start the automatic loading of the CLUT
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///CLUT color mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCM_A {
    ///0: CLUT color format ARGB8888
    ARGB8888 = 0,
    ///1: CLUT color format RGB888
    RGB888 = 1,
}
impl From<CCM_A> for bool {
    #[inline(always)]
    fn from(variant: CCM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CCM`
pub type CCM_R = crate::R<bool, CCM_A>;
impl CCM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCM_A {
        match self.bits {
            false => CCM_A::ARGB8888,
            true => CCM_A::RGB888,
        }
    }
    ///Checks if the value of the field is `ARGB8888`
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CCM_A::ARGB8888
    }
    ///Checks if the value of the field is `RGB888`
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CCM_A::RGB888
    }
}
///Write proxy for field `CCM`
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CLUT color format ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CCM_A::ARGB8888)
    }
    ///CLUT color format RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CCM_A::RGB888)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Color mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CM_A {
    ///0: Color mode ARGB8888
    ARGB8888 = 0,
    ///1: Color mode RGB888
    RGB888 = 1,
    ///2: Color mode RGB565
    RGB565 = 2,
    ///3: Color mode ARGB1555
    ARGB1555 = 3,
    ///4: Color mode ARGB4444
    ARGB4444 = 4,
    ///5: Color mode L8
    L8 = 5,
    ///6: Color mode AL44
    AL44 = 6,
    ///7: Color mode AL88
    AL88 = 7,
    ///8: Color mode L4
    L4 = 8,
    ///9: Color mode A8
    A8 = 9,
    ///10: Color mode A4
    A4 = 10,
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
            5 => Val(CM_A::L8),
            6 => Val(CM_A::AL44),
            7 => Val(CM_A::AL88),
            8 => Val(CM_A::L4),
            9 => Val(CM_A::A8),
            10 => Val(CM_A::A4),
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
    ///Checks if the value of the field is `L8`
    #[inline(always)]
    pub fn is_l8(&self) -> bool {
        *self == CM_A::L8
    }
    ///Checks if the value of the field is `AL44`
    #[inline(always)]
    pub fn is_al44(&self) -> bool {
        *self == CM_A::AL44
    }
    ///Checks if the value of the field is `AL88`
    #[inline(always)]
    pub fn is_al88(&self) -> bool {
        *self == CM_A::AL88
    }
    ///Checks if the value of the field is `L4`
    #[inline(always)]
    pub fn is_l4(&self) -> bool {
        *self == CM_A::L4
    }
    ///Checks if the value of the field is `A8`
    #[inline(always)]
    pub fn is_a8(&self) -> bool {
        *self == CM_A::A8
    }
    ///Checks if the value of the field is `A4`
    #[inline(always)]
    pub fn is_a4(&self) -> bool {
        *self == CM_A::A4
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
    ///Color mode ARGB8888
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CM_A::ARGB8888)
    }
    ///Color mode RGB888
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CM_A::RGB888)
    }
    ///Color mode RGB565
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(CM_A::RGB565)
    }
    ///Color mode ARGB1555
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(CM_A::ARGB1555)
    }
    ///Color mode ARGB4444
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(CM_A::ARGB4444)
    }
    ///Color mode L8
    #[inline(always)]
    pub fn l8(self) -> &'a mut W {
        self.variant(CM_A::L8)
    }
    ///Color mode AL44
    #[inline(always)]
    pub fn al44(self) -> &'a mut W {
        self.variant(CM_A::AL44)
    }
    ///Color mode AL88
    #[inline(always)]
    pub fn al88(self) -> &'a mut W {
        self.variant(CM_A::AL88)
    }
    ///Color mode L4
    #[inline(always)]
    pub fn l4(self) -> &'a mut W {
        self.variant(CM_A::L4)
    }
    ///Color mode A8
    #[inline(always)]
    pub fn a8(self) -> &'a mut W {
        self.variant(CM_A::A8)
    }
    ///Color mode A4
    #[inline(always)]
    pub fn a4(self) -> &'a mut W {
        self.variant(CM_A::A4)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - CLUT color mode
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 0:3 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
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
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&mut self) -> ALPHA_W {
        ALPHA_W { w: self }
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&mut self) -> AM_W {
        AM_W { w: self }
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    ///Bit 4 - CLUT color mode
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    ///Bits 0:3 - Color mode
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
