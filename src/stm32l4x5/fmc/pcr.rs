///Reader of register PCR
pub type R = crate::R<u32, super::PCR>;
///Writer for register PCR
pub type W = crate::W<u32, super::PCR>;
///Register PCR `reset()`'s with value 0x18
impl crate::ResetValue for super::PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x18
    }
}
///ECCPS
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECCPS_A {
    ///0: ECC page size 256 bytes
    BYTES256 = 0,
    ///1: ECC page size 512 bytes
    BYTES512 = 1,
    ///2: ECC page size 1024 bytes
    BYTES1024 = 2,
    ///3: ECC page size 2048 bytes
    BYTES2048 = 3,
    ///4: ECC page size 4096 bytes
    BYTES4096 = 4,
    ///5: ECC page size 8192 bytes
    BYTES8192 = 5,
}
impl From<ECCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCPS_A) -> Self {
        variant as _
    }
}
///Reader of field `ECCPS`
pub type ECCPS_R = crate::R<u8, ECCPS_A>;
impl ECCPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ECCPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ECCPS_A::BYTES256),
            1 => Val(ECCPS_A::BYTES512),
            2 => Val(ECCPS_A::BYTES1024),
            3 => Val(ECCPS_A::BYTES2048),
            4 => Val(ECCPS_A::BYTES4096),
            5 => Val(ECCPS_A::BYTES8192),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `BYTES256`
    #[inline(always)]
    pub fn is_bytes256(&self) -> bool {
        *self == ECCPS_A::BYTES256
    }
    ///Checks if the value of the field is `BYTES512`
    #[inline(always)]
    pub fn is_bytes512(&self) -> bool {
        *self == ECCPS_A::BYTES512
    }
    ///Checks if the value of the field is `BYTES1024`
    #[inline(always)]
    pub fn is_bytes1024(&self) -> bool {
        *self == ECCPS_A::BYTES1024
    }
    ///Checks if the value of the field is `BYTES2048`
    #[inline(always)]
    pub fn is_bytes2048(&self) -> bool {
        *self == ECCPS_A::BYTES2048
    }
    ///Checks if the value of the field is `BYTES4096`
    #[inline(always)]
    pub fn is_bytes4096(&self) -> bool {
        *self == ECCPS_A::BYTES4096
    }
    ///Checks if the value of the field is `BYTES8192`
    #[inline(always)]
    pub fn is_bytes8192(&self) -> bool {
        *self == ECCPS_A::BYTES8192
    }
}
///Write proxy for field `ECCPS`
pub struct ECCPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCPS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECCPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///ECC page size 256 bytes
    #[inline(always)]
    pub fn bytes256(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES256)
    }
    ///ECC page size 512 bytes
    #[inline(always)]
    pub fn bytes512(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES512)
    }
    ///ECC page size 1024 bytes
    #[inline(always)]
    pub fn bytes1024(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES1024)
    }
    ///ECC page size 2048 bytes
    #[inline(always)]
    pub fn bytes2048(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES2048)
    }
    ///ECC page size 4096 bytes
    #[inline(always)]
    pub fn bytes4096(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES4096)
    }
    ///ECC page size 8192 bytes
    #[inline(always)]
    pub fn bytes8192(self) -> &'a mut W {
        self.variant(ECCPS_A::BYTES8192)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
///Reader of field `TAR`
pub type TAR_R = crate::R<u8, u8>;
///Write proxy for field `TAR`
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
///Reader of field `TCLR`
pub type TCLR_R = crate::R<u8, u8>;
///Write proxy for field `TCLR`
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
///ECCEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECCEN_A {
    ///0: ECC logic is disabled and reset
    DISABLED = 0,
    ///1: ECC logic is enabled
    ENABLED = 1,
}
impl From<ECCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ECCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ECCEN`
pub type ECCEN_R = crate::R<bool, ECCEN_A>;
impl ECCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECCEN_A {
        match self.bits {
            false => ECCEN_A::DISABLED,
            true => ECCEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECCEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECCEN_A::ENABLED
    }
}
///Write proxy for field `ECCEN`
pub struct ECCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ECCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///ECC logic is disabled and reset
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECCEN_A::DISABLED)
    }
    ///ECC logic is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECCEN_A::ENABLED)
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
///PWID
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWID_A {
    ///0: External memory device width 8 bits
    BITS8 = 0,
    ///1: External memory device width 16 bits
    BITS16 = 1,
}
impl From<PWID_A> for u8 {
    #[inline(always)]
    fn from(variant: PWID_A) -> Self {
        variant as _
    }
}
///Reader of field `PWID`
pub type PWID_R = crate::R<u8, PWID_A>;
impl PWID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWID_A::BITS8),
            1 => Val(PWID_A::BITS16),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `BITS8`
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PWID_A::BITS8
    }
    ///Checks if the value of the field is `BITS16`
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PWID_A::BITS16
    }
}
///Write proxy for field `PWID`
pub struct PWID_W<'a> {
    w: &'a mut W,
}
impl<'a> PWID_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PWID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///External memory device width 8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PWID_A::BITS8)
    }
    ///External memory device width 16 bits
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PWID_A::BITS16)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///PTYP
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYP_A {
    ///1: NAND Flash
    NANDFLASH = 1,
}
impl From<PTYP_A> for bool {
    #[inline(always)]
    fn from(variant: PTYP_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PTYP`
pub type PTYP_R = crate::R<bool, PTYP_A>;
impl PTYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PTYP_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PTYP_A::NANDFLASH),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `NANDFLASH`
    #[inline(always)]
    pub fn is_nandflash(&self) -> bool {
        *self == PTYP_A::NANDFLASH
    }
}
///Write proxy for field `PTYP`
pub struct PTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PTYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///NAND Flash
    #[inline(always)]
    pub fn nandflash(self) -> &'a mut W {
        self.variant(PTYP_A::NANDFLASH)
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
///PBKEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBKEN_A {
    ///0: Corresponding memory bank is disabled
    DISABLED = 0,
    ///1: Corresponding memory bank is enabled
    ENABLED = 1,
}
impl From<PBKEN_A> for bool {
    #[inline(always)]
    fn from(variant: PBKEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PBKEN`
pub type PBKEN_R = crate::R<bool, PBKEN_A>;
impl PBKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PBKEN_A {
        match self.bits {
            false => PBKEN_A::DISABLED,
            true => PBKEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PBKEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PBKEN_A::ENABLED
    }
}
///Write proxy for field `PBKEN`
pub struct PBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PBKEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PBKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Corresponding memory bank is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PBKEN_A::DISABLED)
    }
    ///Corresponding memory bank is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PBKEN_A::ENABLED)
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
///PWAITEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWAITEN_A {
    ///0: Wait feature disabled
    DISABLED = 0,
    ///1: Wait feature enabled
    ENABLED = 1,
}
impl From<PWAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWAITEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `PWAITEN`
pub type PWAITEN_R = crate::R<bool, PWAITEN_A>;
impl PWAITEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PWAITEN_A {
        match self.bits {
            false => PWAITEN_A::DISABLED,
            true => PWAITEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWAITEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWAITEN_A::ENABLED
    }
}
///Write proxy for field `PWAITEN`
pub struct PWAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWAITEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PWAITEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Wait feature disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWAITEN_A::DISABLED)
    }
    ///Wait feature enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWAITEN_A::ENABLED)
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
impl R {
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 3 - PTYP
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    pub fn eccps(&mut self) -> ECCPS_W {
        ECCPS_W { w: self }
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&mut self) -> ECCEN_W {
        ECCEN_W { w: self }
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&mut self) -> PWID_W {
        PWID_W { w: self }
    }
    ///Bit 3 - PTYP
    #[inline(always)]
    pub fn ptyp(&mut self) -> PTYP_W {
        PTYP_W { w: self }
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&mut self) -> PBKEN_W {
        PBKEN_W { w: self }
    }
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PWAITEN_W {
        PWAITEN_W { w: self }
    }
}
