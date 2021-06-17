///Reader of register CR1
pub type R = crate::R<u32, super::CR1>;
///Writer for register CR1
pub type W = crate::W<u32, super::CR1>;
///Register CR1 `reset()`'s with value 0
impl crate::ResetValue for super::CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Clock division
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKD_A {
    ///0: t_DTS = t_CK_INT
    DIV1 = 0,
    ///1: t_DTS = 2 × t_CK_INT
    DIV2 = 1,
    ///2: t_DTS = 4 × t_CK_INT
    DIV4 = 2,
}
impl From<CKD_A> for u8 {
    #[inline(always)]
    fn from(variant: CKD_A) -> Self {
        variant as _
    }
}
///Reader of field `CKD`
pub type CKD_R = crate::R<u8, CKD_A>;
impl CKD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKD_A::DIV1),
            1 => Val(CKD_A::DIV2),
            2 => Val(CKD_A::DIV4),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `DIV1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKD_A::DIV1
    }
    ///Checks if the value of the field is `DIV2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKD_A::DIV2
    }
    ///Checks if the value of the field is `DIV4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKD_A::DIV4
    }
}
///Write proxy for field `CKD`
pub struct CKD_W<'a> {
    w: &'a mut W,
}
impl<'a> CKD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CKD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///t_DTS = t_CK_INT
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CKD_A::DIV1)
    }
    ///t_DTS = 2 × t_CK_INT
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CKD_A::DIV2)
    }
    ///t_DTS = 4 × t_CK_INT
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CKD_A::DIV4)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Auto-reload preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARPE_A {
    ///0: TIMx_APRR register is not buffered
    DISABLED = 0,
    ///1: TIMx_APRR register is buffered
    ENABLED = 1,
}
impl From<ARPE_A> for bool {
    #[inline(always)]
    fn from(variant: ARPE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ARPE`
pub type ARPE_R = crate::R<bool, ARPE_A>;
impl ARPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARPE_A {
        match self.bits {
            false => ARPE_A::DISABLED,
            true => ARPE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARPE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARPE_A::ENABLED
    }
}
///Write proxy for field `ARPE`
pub struct ARPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///TIMx_APRR register is not buffered
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARPE_A::DISABLED)
    }
    ///TIMx_APRR register is buffered
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARPE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Center-aligned mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMS_A {
    ///0: The counter counts up or down depending on the direction bit
    EDGEALIGNED = 0,
    ///1: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down.
    CENTERALIGNED1 = 1,
    ///2: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up.
    CENTERALIGNED2 = 2,
    ///3: The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down.
    CENTERALIGNED3 = 3,
}
impl From<CMS_A> for u8 {
    #[inline(always)]
    fn from(variant: CMS_A) -> Self {
        variant as _
    }
}
///Reader of field `CMS`
pub type CMS_R = crate::R<u8, CMS_A>;
impl CMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMS_A {
        match self.bits {
            0 => CMS_A::EDGEALIGNED,
            1 => CMS_A::CENTERALIGNED1,
            2 => CMS_A::CENTERALIGNED2,
            3 => CMS_A::CENTERALIGNED3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `EDGEALIGNED`
    #[inline(always)]
    pub fn is_edge_aligned(&self) -> bool {
        *self == CMS_A::EDGEALIGNED
    }
    ///Checks if the value of the field is `CENTERALIGNED1`
    #[inline(always)]
    pub fn is_center_aligned1(&self) -> bool {
        *self == CMS_A::CENTERALIGNED1
    }
    ///Checks if the value of the field is `CENTERALIGNED2`
    #[inline(always)]
    pub fn is_center_aligned2(&self) -> bool {
        *self == CMS_A::CENTERALIGNED2
    }
    ///Checks if the value of the field is `CENTERALIGNED3`
    #[inline(always)]
    pub fn is_center_aligned3(&self) -> bool {
        *self == CMS_A::CENTERALIGNED3
    }
}
///Write proxy for field `CMS`
pub struct CMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CMS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///The counter counts up or down depending on the direction bit
    #[inline(always)]
    pub fn edge_aligned(self) -> &'a mut W {
        self.variant(CMS_A::EDGEALIGNED)
    }
    ///The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down.
    #[inline(always)]
    pub fn center_aligned1(self) -> &'a mut W {
        self.variant(CMS_A::CENTERALIGNED1)
    }
    ///The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up.
    #[inline(always)]
    pub fn center_aligned2(self) -> &'a mut W {
        self.variant(CMS_A::CENTERALIGNED2)
    }
    ///The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down.
    #[inline(always)]
    pub fn center_aligned3(self) -> &'a mut W {
        self.variant(CMS_A::CENTERALIGNED3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
///Direction
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    ///0: Counter used as upcounter
    UP = 0,
    ///1: Counter used as downcounter
    DOWN = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `DIR`
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::UP,
            true => DIR_A::DOWN,
        }
    }
    ///Checks if the value of the field is `UP`
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == DIR_A::UP
    }
    ///Checks if the value of the field is `DOWN`
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == DIR_A::DOWN
    }
}
///Write proxy for field `DIR`
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Counter used as upcounter
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(DIR_A::UP)
    }
    ///Counter used as downcounter
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(DIR_A::DOWN)
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
///One-pulse mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPM_A {
    ///0: Counter is not stopped at update event
    DISABLED = 0,
    ///1: Counter stops counting at the next update event (clearing the CEN bit)
    ENABLED = 1,
}
impl From<OPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPM_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OPM`
pub type OPM_R = crate::R<bool, OPM_A>;
impl OPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPM_A {
        match self.bits {
            false => OPM_A::DISABLED,
            true => OPM_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPM_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPM_A::ENABLED
    }
}
///Write proxy for field `OPM`
pub struct OPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Counter is not stopped at update event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPM_A::DISABLED)
    }
    ///Counter stops counting at the next update event (clearing the CEN bit)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPM_A::ENABLED)
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
///Update request source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URS_A {
    ///0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
    ANYEVENT = 0,
    ///1: Only counter overflow/underflow generates an update interrupt or DMA request
    COUNTERONLY = 1,
}
impl From<URS_A> for bool {
    #[inline(always)]
    fn from(variant: URS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `URS`
pub type URS_R = crate::R<bool, URS_A>;
impl URS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> URS_A {
        match self.bits {
            false => URS_A::ANYEVENT,
            true => URS_A::COUNTERONLY,
        }
    }
    ///Checks if the value of the field is `ANYEVENT`
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == URS_A::ANYEVENT
    }
    ///Checks if the value of the field is `COUNTERONLY`
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == URS_A::COUNTERONLY
    }
}
///Write proxy for field `URS`
pub struct URS_W<'a> {
    w: &'a mut W,
}
impl<'a> URS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: URS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URS_A::ANYEVENT)
    }
    ///Only counter overflow/underflow generates an update interrupt or DMA request
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(URS_A::COUNTERONLY)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Update disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDIS_A {
    ///0: Update event enabled
    ENABLED = 0,
    ///1: Update event disabled
    DISABLED = 1,
}
impl From<UDIS_A> for bool {
    #[inline(always)]
    fn from(variant: UDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `UDIS`
pub type UDIS_R = crate::R<bool, UDIS_A>;
impl UDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDIS_A {
        match self.bits {
            false => UDIS_A::ENABLED,
            true => UDIS_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDIS_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDIS_A::DISABLED
    }
}
///Write proxy for field `UDIS`
pub struct UDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Update event enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDIS_A::ENABLED)
    }
    ///Update event disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDIS_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Counter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    ///0: Counter disabled
    DISABLED = 0,
    ///1: Counter enabled
    ENABLED = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CEN`
pub type CEN_R = crate::R<bool, CEN_A>;
impl CEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLED,
            true => CEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN_A::ENABLED
    }
}
///Write proxy for field `CEN`
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::DISABLED)
    }
    ///Counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    ///Bits 8:9 - Clock division
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 5:6 - Center-aligned mode selection
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bit 4 - Direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 8:9 - Clock division
    #[inline(always)]
    pub fn ckd(&mut self) -> CKD_W {
        CKD_W { w: self }
    }
    ///Bit 7 - Auto-reload preload enable
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W {
        ARPE_W { w: self }
    }
    ///Bits 5:6 - Center-aligned mode selection
    #[inline(always)]
    pub fn cms(&mut self) -> CMS_W {
        CMS_W { w: self }
    }
    ///Bit 4 - Direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    ///Bit 3 - One-pulse mode
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W {
        OPM_W { w: self }
    }
    ///Bit 2 - Update request source
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W {
        URS_W { w: self }
    }
    ///Bit 1 - Update disable
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W {
        UDIS_W { w: self }
    }
    ///Bit 0 - Counter enable
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
}
