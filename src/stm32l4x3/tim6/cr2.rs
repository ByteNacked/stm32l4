///Reader of register CR2
pub type R = crate::R<u32, super::CR2>;
///Writer for register CR2
pub type W = crate::W<u32, super::CR2>;
///Register CR2 `reset()`'s with value 0
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Master mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MMS_A {
    ///0: Use UG bit from TIMx_EGR register
    RESET = 0,
    ///1: Use CNT bit from TIMx_CEN register
    ENABLE = 1,
    ///2: Use the update event
    UPDATE = 2,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
///Reader of field `MMS`
pub type MMS_R = crate::R<u8, MMS_A>;
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MMS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MMS_A::RESET),
            1 => Val(MMS_A::ENABLE),
            2 => Val(MMS_A::UPDATE),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RESET`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::RESET
    }
    ///Checks if the value of the field is `ENABLE`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::ENABLE
    }
    ///Checks if the value of the field is `UPDATE`
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::UPDATE
    }
}
///Write proxy for field `MMS`
pub struct MMS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MMS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Use UG bit from TIMx_EGR register
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::RESET)
    }
    ///Use CNT bit from TIMx_CEN register
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::ENABLE)
    }
    ///Use the update event
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::UPDATE)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W {
        MMS_W { w: self }
    }
}
