///Reader of register DIEPMSK
pub type R = crate::R<u32, super::DIEPMSK>;
///Writer for register DIEPMSK
pub type W = crate::W<u32, super::DIEPMSK>;
///Register DIEPMSK `reset()`'s with value 0
impl crate::ResetValue for super::DIEPMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `XFRCM`
pub type XFRCM_R = crate::R<bool, bool>;
///Write proxy for field `XFRCM`
pub struct XFRCM_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRCM_W<'a> {
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
///Reader of field `EPDM`
pub type EPDM_R = crate::R<bool, bool>;
///Write proxy for field `EPDM`
pub struct EPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDM_W<'a> {
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
///Reader of field `TOM`
pub type TOM_R = crate::R<bool, bool>;
///Write proxy for field `TOM`
pub struct TOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TOM_W<'a> {
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
///Reader of field `ITTXFEMSK`
pub type ITTXFEMSK_R = crate::R<bool, bool>;
///Write proxy for field `ITTXFEMSK`
pub struct ITTXFEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ITTXFEMSK_W<'a> {
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
///Reader of field `INEPNMM`
pub type INEPNMM_R = crate::R<bool, bool>;
///Write proxy for field `INEPNMM`
pub struct INEPNMM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNMM_W<'a> {
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
///Reader of field `INEPNEM`
pub type INEPNEM_R = crate::R<bool, bool>;
///Write proxy for field `INEPNEM`
pub struct INEPNEM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNEM_W<'a> {
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
impl R {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - Timeout condition mask (Non-isochronous endpoints)
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - IN token received with EP mismatch mask
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - IN endpoint NAK effective mask
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W {
        EPDM_W { w: self }
    }
    ///Bit 3 - Timeout condition mask (Non-isochronous endpoints)
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W {
        TOM_W { w: self }
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W {
        ITTXFEMSK_W { w: self }
    }
    ///Bit 5 - IN token received with EP mismatch mask
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W {
        INEPNMM_W { w: self }
    }
    ///Bit 6 - IN endpoint NAK effective mask
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W {
        INEPNEM_W { w: self }
    }
}
