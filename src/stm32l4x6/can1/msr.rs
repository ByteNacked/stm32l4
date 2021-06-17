///Reader of register MSR
pub type R = crate::R<u32, super::MSR>;
///Writer for register MSR
pub type W = crate::W<u32, super::MSR>;
///Register MSR `reset()`'s with value 0x0c02
impl crate::ResetValue for super::MSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c02
    }
}
///Reader of field `RX`
pub type RX_R = crate::R<bool, bool>;
///Reader of field `SAMP`
pub type SAMP_R = crate::R<bool, bool>;
///Reader of field `RXM`
pub type RXM_R = crate::R<bool, bool>;
///Reader of field `TXM`
pub type TXM_R = crate::R<bool, bool>;
///Reader of field `SLAKI`
pub type SLAKI_R = crate::R<bool, bool>;
///Write proxy for field `SLAKI`
pub struct SLAKI_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAKI_W<'a> {
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
///Reader of field `WKUI`
pub type WKUI_R = crate::R<bool, bool>;
///Write proxy for field `WKUI`
pub struct WKUI_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUI_W<'a> {
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
///Reader of field `ERRI`
pub type ERRI_R = crate::R<bool, bool>;
///Write proxy for field `ERRI`
pub struct ERRI_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRI_W<'a> {
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
///Reader of field `SLAK`
pub type SLAK_R = crate::R<bool, bool>;
///Reader of field `INAK`
pub type INAK_R = crate::R<bool, bool>;
impl R {
    ///Bit 11 - RX
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - SAMP
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - RXM
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - TXM
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 4 - SLAKI
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - WKUI
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - ERRI
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - SLAK
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - INAK
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 4 - SLAKI
    #[inline(always)]
    pub fn slaki(&mut self) -> SLAKI_W {
        SLAKI_W { w: self }
    }
    ///Bit 3 - WKUI
    #[inline(always)]
    pub fn wkui(&mut self) -> WKUI_W {
        WKUI_W { w: self }
    }
    ///Bit 2 - ERRI
    #[inline(always)]
    pub fn erri(&mut self) -> ERRI_W {
        ERRI_W { w: self }
    }
}
