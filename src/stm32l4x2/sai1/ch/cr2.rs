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
///Companding mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_A {
    ///0: No companding algorithm
    NOCOMPANDING = 0,
    ///2: μ-Law algorithm
    MULAW = 2,
    ///3: A-Law algorithm
    ALAW = 3,
}
impl From<COMP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as _
    }
}
///Reader of field `COMP`
pub type COMP_R = crate::R<u8, COMP_A>;
impl COMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP_A::NOCOMPANDING),
            2 => Val(COMP_A::MULAW),
            3 => Val(COMP_A::ALAW),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `NOCOMPANDING`
    #[inline(always)]
    pub fn is_no_companding(&self) -> bool {
        *self == COMP_A::NOCOMPANDING
    }
    ///Checks if the value of the field is `MULAW`
    #[inline(always)]
    pub fn is_mu_law(&self) -> bool {
        *self == COMP_A::MULAW
    }
    ///Checks if the value of the field is `ALAW`
    #[inline(always)]
    pub fn is_alaw(&self) -> bool {
        *self == COMP_A::ALAW
    }
}
///Write proxy for field `COMP`
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No companding algorithm
    #[inline(always)]
    pub fn no_companding(self) -> &'a mut W {
        self.variant(COMP_A::NOCOMPANDING)
    }
    ///μ-Law algorithm
    #[inline(always)]
    pub fn mu_law(self) -> &'a mut W {
        self.variant(COMP_A::MULAW)
    }
    ///A-Law algorithm
    #[inline(always)]
    pub fn alaw(self) -> &'a mut W {
        self.variant(COMP_A::ALAW)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Complement bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPL_A {
    ///0: 1’s complement representation
    ONESCOMPLEMENT = 0,
    ///1: 2’s complement representation
    TWOSCOMPLEMENT = 1,
}
impl From<CPL_A> for bool {
    #[inline(always)]
    fn from(variant: CPL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CPL`
pub type CPL_R = crate::R<bool, CPL_A>;
impl CPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPL_A {
        match self.bits {
            false => CPL_A::ONESCOMPLEMENT,
            true => CPL_A::TWOSCOMPLEMENT,
        }
    }
    ///Checks if the value of the field is `ONESCOMPLEMENT`
    #[inline(always)]
    pub fn is_ones_complement(&self) -> bool {
        *self == CPL_A::ONESCOMPLEMENT
    }
    ///Checks if the value of the field is `TWOSCOMPLEMENT`
    #[inline(always)]
    pub fn is_twos_complement(&self) -> bool {
        *self == CPL_A::TWOSCOMPLEMENT
    }
}
///Write proxy for field `CPL`
pub struct CPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CPL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///1’s complement representation
    #[inline(always)]
    pub fn ones_complement(self) -> &'a mut W {
        self.variant(CPL_A::ONESCOMPLEMENT)
    }
    ///2’s complement representation
    #[inline(always)]
    pub fn twos_complement(self) -> &'a mut W {
        self.variant(CPL_A::TWOSCOMPLEMENT)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `MUTECN`
pub type MUTECN_R = crate::R<u8, u8>;
///Write proxy for field `MUTECN`
pub struct MUTECN_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTECN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
///Mute value
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEVAL_A {
    ///0: Bit value 0 is sent during the mute mode
    SENDZERO = 0,
    ///1: Last values are sent during the mute mode
    SENDLAST = 1,
}
impl From<MUTEVAL_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEVAL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MUTEVAL`
pub type MUTEVAL_R = crate::R<bool, MUTEVAL_A>;
impl MUTEVAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUTEVAL_A {
        match self.bits {
            false => MUTEVAL_A::SENDZERO,
            true => MUTEVAL_A::SENDLAST,
        }
    }
    ///Checks if the value of the field is `SENDZERO`
    #[inline(always)]
    pub fn is_send_zero(&self) -> bool {
        *self == MUTEVAL_A::SENDZERO
    }
    ///Checks if the value of the field is `SENDLAST`
    #[inline(always)]
    pub fn is_send_last(&self) -> bool {
        *self == MUTEVAL_A::SENDLAST
    }
}
///Write proxy for field `MUTEVAL`
pub struct MUTEVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEVAL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MUTEVAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Bit value 0 is sent during the mute mode
    #[inline(always)]
    pub fn send_zero(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SENDZERO)
    }
    ///Last values are sent during the mute mode
    #[inline(always)]
    pub fn send_last(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SENDLAST)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Mute
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTE_A {
    ///0: No mute mode
    DISABLED = 0,
    ///1: Mute mode enabled
    ENABLED = 1,
}
impl From<MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MUTE`
pub type MUTE_R = crate::R<bool, MUTE_A>;
impl MUTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUTE_A {
        match self.bits {
            false => MUTE_A::DISABLED,
            true => MUTE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTE_A::ENABLED
    }
}
///Write proxy for field `MUTE`
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MUTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No mute mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTE_A::DISABLED)
    }
    ///Mute mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTE_A::ENABLED)
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
///Reader of field `TRIS`
pub type TRIS_R = crate::R<bool, bool>;
///Write proxy for field `TRIS`
pub struct TRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIS_W<'a> {
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
///FIFO flush
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLUSH_A {
    ///0: No FIFO flush
    NOFLUSH = 0,
    ///1: FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared
    FLUSH = 1,
}
impl From<FFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: FFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FFLUSH`
pub type FFLUSH_R = crate::R<bool, FFLUSH_A>;
impl FFLUSH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FFLUSH_A {
        match self.bits {
            false => FFLUSH_A::NOFLUSH,
            true => FFLUSH_A::FLUSH,
        }
    }
    ///Checks if the value of the field is `NOFLUSH`
    #[inline(always)]
    pub fn is_no_flush(&self) -> bool {
        *self == FFLUSH_A::NOFLUSH
    }
    ///Checks if the value of the field is `FLUSH`
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FFLUSH_A::FLUSH
    }
}
///Write proxy for field `FFLUSH`
pub struct FFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLUSH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FFLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No FIFO flush
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::NOFLUSH)
    }
    ///FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::FLUSH)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///FIFO threshold
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTH_A {
    ///0: FIFO empty
    EMPTY = 0,
    ///1: 1⁄4 FIFO
    QUARTER1 = 1,
    ///2: 1⁄2 FIFO
    QUARTER2 = 2,
    ///3: 3⁄4 FIFO
    QUARTER3 = 3,
    ///4: FIFO full
    FULL = 4,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        variant as _
    }
}
///Reader of field `FTH`
pub type FTH_R = crate::R<u8, FTH_A>;
impl FTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTH_A::EMPTY),
            1 => Val(FTH_A::QUARTER1),
            2 => Val(FTH_A::QUARTER2),
            3 => Val(FTH_A::QUARTER3),
            4 => Val(FTH_A::FULL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FTH_A::EMPTY
    }
    ///Checks if the value of the field is `QUARTER1`
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FTH_A::QUARTER1
    }
    ///Checks if the value of the field is `QUARTER2`
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FTH_A::QUARTER2
    }
    ///Checks if the value of the field is `QUARTER3`
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FTH_A::QUARTER3
    }
    ///Checks if the value of the field is `FULL`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH_A::FULL
    }
}
///Write proxy for field `FTH`
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///FIFO empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(FTH_A::EMPTY)
    }
    ///1⁄4 FIFO
    #[inline(always)]
    pub fn quarter1(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER1)
    }
    ///1⁄2 FIFO
    #[inline(always)]
    pub fn quarter2(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER2)
    }
    ///3⁄4 FIFO
    #[inline(always)]
    pub fn quarter3(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER3)
    }
    ///FIFO full
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::FULL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    pub fn mutecn(&self) -> MUTECN_R {
        MUTECN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Mute
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    pub fn fflush(&self) -> FFLUSH_R {
        FFLUSH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W { w: self }
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    pub fn mutecn(&mut self) -> MUTECN_W {
        MUTECN_W { w: self }
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W {
        MUTEVAL_W { w: self }
    }
    ///Bit 5 - Mute
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W {
        TRIS_W { w: self }
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W {
        FFLUSH_W { w: self }
    }
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
}
