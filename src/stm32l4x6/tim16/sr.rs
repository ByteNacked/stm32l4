///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CC1OF`
pub type CC1OF_R = crate::R<bool, bool>;
///Write proxy for field `CC1OF`
pub struct CC1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1OF_W<'a> {
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
///Reader of field `BIF`
pub type BIF_R = crate::R<bool, bool>;
///Write proxy for field `BIF`
pub struct BIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BIF_W<'a> {
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
///Reader of field `TIF`
pub type TIF_R = crate::R<bool, bool>;
///Write proxy for field `TIF`
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
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
///Reader of field `COMIF`
pub type COMIF_R = crate::R<bool, bool>;
///Write proxy for field `COMIF`
pub struct COMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMIF_W<'a> {
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
///Reader of field `CC1IF`
pub type CC1IF_R = crate::R<bool, bool>;
///Write proxy for field `CC1IF`
pub struct CC1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IF_W<'a> {
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
///Update interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIF_A {
    ///0: No update occurred
    CLEAR = 0,
    ///1: Update interrupt pending.
    UPDATEPENDING = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `UIF`
pub type UIF_R = crate::R<bool, UIF_A>;
impl UIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::CLEAR,
            true => UIF_A::UPDATEPENDING,
        }
    }
    ///Checks if the value of the field is `CLEAR`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UIF_A::CLEAR
    }
    ///Checks if the value of the field is `UPDATEPENDING`
    #[inline(always)]
    pub fn is_update_pending(&self) -> bool {
        *self == UIF_A::UPDATEPENDING
    }
}
///Write proxy for field `UIF`
pub struct UIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No update occurred
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIF_A::CLEAR)
    }
    ///Update interrupt pending.
    #[inline(always)]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UIF_A::UPDATEPENDING)
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
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W {
        CC1OF_W { w: self }
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W {
        BIF_W { w: self }
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W {
        COMIF_W { w: self }
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W {
        CC1IF_W { w: self }
    }
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W {
        UIF_W { w: self }
    }
}
