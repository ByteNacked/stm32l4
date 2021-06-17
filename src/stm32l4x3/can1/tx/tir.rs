///Reader of register TIR
pub type R = crate::R<u32, super::TIR>;
///Writer for register TIR
pub type W = crate::W<u32, super::TIR>;
///Register TIR `reset()`'s with value 0
impl crate::ResetValue for super::TIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `STID`
pub type STID_R = crate::R<u16, u16>;
///Write proxy for field `STID`
pub struct STID_W<'a> {
    w: &'a mut W,
}
impl<'a> STID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | (((value as u32) & 0x07ff) << 21);
        self.w
    }
}
///Reader of field `EXID`
pub type EXID_R = crate::R<u32, u32>;
///Write proxy for field `EXID`
pub struct EXID_W<'a> {
    w: &'a mut W,
}
impl<'a> EXID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 3)) | (((value as u32) & 0x0003_ffff) << 3);
        self.w
    }
}
///IDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    ///0: Standard identifier
    STANDARD = 0,
    ///1: Extended identifier
    EXTENDED = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IDE`
pub type IDE_R = crate::R<bool, IDE_A>;
impl IDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::STANDARD,
            true => IDE_A::EXTENDED,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE_A::STANDARD
    }
    ///Checks if the value of the field is `EXTENDED`
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE_A::EXTENDED
    }
}
///Write proxy for field `IDE`
pub struct IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Standard identifier
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(IDE_A::STANDARD)
    }
    ///Extended identifier
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(IDE_A::EXTENDED)
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
///RTR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    ///0: Data frame
    DATA = 0,
    ///1: Remote frame
    REMOTE = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RTR`
pub type RTR_R = crate::R<bool, RTR_A>;
impl RTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::DATA,
            true => RTR_A::REMOTE,
        }
    }
    ///Checks if the value of the field is `DATA`
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR_A::DATA
    }
    ///Checks if the value of the field is `REMOTE`
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR_A::REMOTE
    }
}
///Write proxy for field `RTR`
pub struct RTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Data frame
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(RTR_A::DATA)
    }
    ///Remote frame
    #[inline(always)]
    pub fn remote(self) -> &'a mut W {
        self.variant(RTR_A::REMOTE)
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
///Reader of field `TXRQ`
pub type TXRQ_R = crate::R<bool, bool>;
///Write proxy for field `TXRQ`
pub struct TXRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRQ_W<'a> {
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
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - TXRQ
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&mut self) -> STID_W {
        STID_W { w: self }
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&mut self) -> EXID_W {
        EXID_W { w: self }
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W { w: self }
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W {
        RTR_W { w: self }
    }
    ///Bit 0 - TXRQ
    #[inline(always)]
    pub fn txrq(&mut self) -> TXRQ_W {
        TXRQ_W { w: self }
    }
}
