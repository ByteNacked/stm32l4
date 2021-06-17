///Reader of register BTR
pub type R = crate::R<u32, super::BTR>;
///Writer for register BTR
pub type W = crate::W<u32, super::BTR>;
///Register BTR `reset()`'s with value 0
impl crate::ResetValue for super::BTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///SILM
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SILM_A {
    ///0: Normal operation
    NORMAL = 0,
    ///1: Silent Mode
    SILENT = 1,
}
impl From<SILM_A> for bool {
    #[inline(always)]
    fn from(variant: SILM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SILM`
pub type SILM_R = crate::R<bool, SILM_A>;
impl SILM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SILM_A {
        match self.bits {
            false => SILM_A::NORMAL,
            true => SILM_A::SILENT,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SILM_A::NORMAL
    }
    ///Checks if the value of the field is `SILENT`
    #[inline(always)]
    pub fn is_silent(&self) -> bool {
        *self == SILM_A::SILENT
    }
}
///Write proxy for field `SILM`
pub struct SILM_W<'a> {
    w: &'a mut W,
}
impl<'a> SILM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SILM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Normal operation
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SILM_A::NORMAL)
    }
    ///Silent Mode
    #[inline(always)]
    pub fn silent(self) -> &'a mut W {
        self.variant(SILM_A::SILENT)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
///LBKM
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKM_A {
    ///0: Loop Back Mode disabled
    DISABLED = 0,
    ///1: Loop Back Mode enabled
    ENABLED = 1,
}
impl From<LBKM_A> for bool {
    #[inline(always)]
    fn from(variant: LBKM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LBKM`
pub type LBKM_R = crate::R<bool, LBKM_A>;
impl LBKM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBKM_A {
        match self.bits {
            false => LBKM_A::DISABLED,
            true => LBKM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBKM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBKM_A::ENABLED
    }
}
///Write proxy for field `LBKM`
pub struct LBKM_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LBKM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Loop Back Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBKM_A::DISABLED)
    }
    ///Loop Back Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBKM_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///Reader of field `SJW`
pub type SJW_R = crate::R<u8, u8>;
///Write proxy for field `SJW`
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
///Reader of field `TS2`
pub type TS2_R = crate::R<u8, u8>;
///Write proxy for field `TS2`
pub struct TS2_W<'a> {
    w: &'a mut W,
}
impl<'a> TS2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
///Reader of field `TS1`
pub type TS1_R = crate::R<u8, u8>;
///Write proxy for field `TS1`
pub struct TS1_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
///Reader of field `BRP`
pub type BRP_R = crate::R<u16, u16>;
///Write proxy for field `BRP`
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    ///Bit 31 - SILM
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - LBKM
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bits 24:25 - SJW
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    ///Bits 20:22 - TS2
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    ///Bits 16:19 - TS1
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 0:9 - BRP
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bit 31 - SILM
    #[inline(always)]
    pub fn silm(&mut self) -> SILM_W {
        SILM_W { w: self }
    }
    ///Bit 30 - LBKM
    #[inline(always)]
    pub fn lbkm(&mut self) -> LBKM_W {
        LBKM_W { w: self }
    }
    ///Bits 24:25 - SJW
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    ///Bits 20:22 - TS2
    #[inline(always)]
    pub fn ts2(&mut self) -> TS2_W {
        TS2_W { w: self }
    }
    ///Bits 16:19 - TS1
    #[inline(always)]
    pub fn ts1(&mut self) -> TS1_W {
        TS1_W { w: self }
    }
    ///Bits 0:9 - BRP
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
}
