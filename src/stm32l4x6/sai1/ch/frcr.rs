///Reader of register FRCR
pub type R = crate::R<u32, super::FRCR>;
///Writer for register FRCR
pub type W = crate::W<u32, super::FRCR>;
///Register FRCR `reset()`'s with value 0x07
impl crate::ResetValue for super::FRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
///Frame synchronization offset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSOFF_A {
    ///0: FS is asserted on the first bit of the slot 0
    ONFIRST = 0,
    ///1: FS is asserted one bit before the first bit of the slot 0
    BEFOREFIRST = 1,
}
impl From<FSOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FSOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FSOFF`
pub type FSOFF_R = crate::R<bool, FSOFF_A>;
impl FSOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSOFF_A {
        match self.bits {
            false => FSOFF_A::ONFIRST,
            true => FSOFF_A::BEFOREFIRST,
        }
    }
    ///Checks if the value of the field is `ONFIRST`
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        *self == FSOFF_A::ONFIRST
    }
    ///Checks if the value of the field is `BEFOREFIRST`
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        *self == FSOFF_A::BEFOREFIRST
    }
}
///Write proxy for field `FSOFF`
pub struct FSOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSOFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FSOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///FS is asserted on the first bit of the slot 0
    #[inline(always)]
    pub fn on_first(self) -> &'a mut W {
        self.variant(FSOFF_A::ONFIRST)
    }
    ///FS is asserted one bit before the first bit of the slot 0
    #[inline(always)]
    pub fn before_first(self) -> &'a mut W {
        self.variant(FSOFF_A::BEFOREFIRST)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Frame synchronization polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSPOL_A {
    ///0: FS is active low (falling edge)
    FALLINGEDGE = 0,
    ///1: FS is active high (rising edge)
    RISINGEDGE = 1,
}
impl From<FSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: FSPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FSPOL`
pub type FSPOL_R = crate::R<bool, FSPOL_A>;
impl FSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSPOL_A {
        match self.bits {
            false => FSPOL_A::FALLINGEDGE,
            true => FSPOL_A::RISINGEDGE,
        }
    }
    ///Checks if the value of the field is `FALLINGEDGE`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == FSPOL_A::FALLINGEDGE
    }
    ///Checks if the value of the field is `RISINGEDGE`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == FSPOL_A::RISINGEDGE
    }
}
///Write proxy for field `FSPOL`
pub struct FSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FSPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///FS is active low (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::FALLINGEDGE)
    }
    ///FS is active high (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::RISINGEDGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Reader of field `FSDEF`
pub type FSDEF_R = crate::R<bool, bool>;
///Write proxy for field `FSDEF`
pub struct FSDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDEF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `FSALL`
pub type FSALL_R = crate::R<u8, u8>;
///Write proxy for field `FSALL`
pub struct FSALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSALL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
///Reader of field `FRL`
pub type FRL_R = crate::R<u8, u8>;
///Write proxy for field `FRL`
pub struct FRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bit 18 - Frame synchronization offset
    #[inline(always)]
    pub fn fsoff(&mut self) -> FSOFF_W {
        FSOFF_W { w: self }
    }
    ///Bit 17 - Frame synchronization polarity
    #[inline(always)]
    pub fn fspol(&mut self) -> FSPOL_W {
        FSPOL_W { w: self }
    }
    ///Bit 16 - Frame synchronization definition
    #[inline(always)]
    pub fn fsdef(&mut self) -> FSDEF_W {
        FSDEF_W { w: self }
    }
    ///Bits 8:14 - Frame synchronization active level length
    #[inline(always)]
    pub fn fsall(&mut self) -> FSALL_W {
        FSALL_W { w: self }
    }
    ///Bits 0:7 - Frame length
    #[inline(always)]
    pub fn frl(&mut self) -> FRL_W {
        FRL_W { w: self }
    }
}
