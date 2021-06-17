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
///Output Idle state 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1N_A {
    ///0: OC1N=0 after a dead-time when MOE=0
    LOW = 0,
    ///1: OC1N=1 after a dead-time when MOE=0
    HIGH = 1,
}
impl From<OIS1N_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1N_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OIS1N`
pub type OIS1N_R = crate::R<bool, OIS1N_A>;
impl OIS1N_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OIS1N_A {
        match self.bits {
            false => OIS1N_A::LOW,
            true => OIS1N_A::HIGH,
        }
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OIS1N_A::LOW
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OIS1N_A::HIGH
    }
}
///Write proxy for field `OIS1N`
pub struct OIS1N_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1N_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OIS1N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///OC1N=0 after a dead-time when MOE=0
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OIS1N_A::LOW)
    }
    ///OC1N=1 after a dead-time when MOE=0
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OIS1N_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Output Idle state 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1_A {
    ///0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0
    LOW = 0,
    ///1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0
    HIGH = 1,
}
impl From<OIS1_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OIS1`
pub type OIS1_R = crate::R<bool, OIS1_A>;
impl OIS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OIS1_A {
        match self.bits {
            false => OIS1_A::LOW,
            true => OIS1_A::HIGH,
        }
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OIS1_A::LOW
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OIS1_A::HIGH
    }
}
///Write proxy for field `OIS1`
pub struct OIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OIS1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OIS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///OC1=0 (after a dead-time if OC1N is implemented) when MOE=0
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OIS1_A::LOW)
    }
    ///OC1=1 (after a dead-time if OC1N is implemented) when MOE=0
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OIS1_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Capture/compare DMA selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDS_A {
    ///0: CCx DMA request sent when CCx event occurs
    ONCOMPARE = 0,
    ///1: CCx DMA request sent when update event occurs
    ONUPDATE = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CCDS`
pub type CCDS_R = crate::R<bool, CCDS_A>;
impl CCDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::ONCOMPARE,
            true => CCDS_A::ONUPDATE,
        }
    }
    ///Checks if the value of the field is `ONCOMPARE`
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS_A::ONCOMPARE
    }
    ///Checks if the value of the field is `ONUPDATE`
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS_A::ONUPDATE
    }
}
///Write proxy for field `CCDS`
pub struct CCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCDS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCDS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCx DMA request sent when CCx event occurs
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDS_A::ONCOMPARE)
    }
    ///CCx DMA request sent when update event occurs
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDS_A::ONUPDATE)
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
///Capture/compare control update selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUS_A {
    ///0: Capture/compare are updated only by setting the COMG bit
    DEFAULT = 0,
    ///1: Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI
    WITHRISINGEDGE = 1,
}
impl From<CCUS_A> for bool {
    #[inline(always)]
    fn from(variant: CCUS_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CCUS`
pub type CCUS_R = crate::R<bool, CCUS_A>;
impl CCUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCUS_A {
        match self.bits {
            false => CCUS_A::DEFAULT,
            true => CCUS_A::WITHRISINGEDGE,
        }
    }
    ///Checks if the value of the field is `DEFAULT`
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CCUS_A::DEFAULT
    }
    ///Checks if the value of the field is `WITHRISINGEDGE`
    #[inline(always)]
    pub fn is_with_rising_edge(&self) -> bool {
        *self == CCUS_A::WITHRISINGEDGE
    }
}
///Write proxy for field `CCUS`
pub struct CCUS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Capture/compare are updated only by setting the COMG bit
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CCUS_A::DEFAULT)
    }
    ///Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI
    #[inline(always)]
    pub fn with_rising_edge(self) -> &'a mut W {
        self.variant(CCUS_A::WITHRISINGEDGE)
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
///Capture/compare preloaded control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPC_A {
    ///0: CCxE, CCxNE and OCxM bits are not preloaded
    NOTPRELOADED = 0,
    ///1: CCxE, CCxNE and OCxM bits are preloaded
    PRELOADED = 1,
}
impl From<CCPC_A> for bool {
    #[inline(always)]
    fn from(variant: CCPC_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CCPC`
pub type CCPC_R = crate::R<bool, CCPC_A>;
impl CCPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCPC_A {
        match self.bits {
            false => CCPC_A::NOTPRELOADED,
            true => CCPC_A::PRELOADED,
        }
    }
    ///Checks if the value of the field is `NOTPRELOADED`
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCPC_A::NOTPRELOADED
    }
    ///Checks if the value of the field is `PRELOADED`
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == CCPC_A::PRELOADED
    }
}
///Write proxy for field `CCPC`
pub struct CCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCPC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///CCxE, CCxNE and OCxM bits are not preloaded
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut W {
        self.variant(CCPC_A::NOTPRELOADED)
    }
    ///CCxE, CCxNE and OCxM bits are preloaded
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut W {
        self.variant(CCPC_A::PRELOADED)
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
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W {
        OIS1N_W { w: self }
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W {
        OIS1_W { w: self }
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W {
        CCDS_W { w: self }
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W {
        CCUS_W { w: self }
    }
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W {
        CCPC_W { w: self }
    }
}
