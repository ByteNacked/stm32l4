///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ENABLE`
pub type ENABLE_R = crate::R<bool, bool>;
///Write proxy for field `ENABLE`
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Reader of field `EDM`
pub type EDM_R = crate::R<u8, u8>;
///Write proxy for field `EDM`
pub struct EDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EDM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
///Reader of field `FCRC`
pub type FCRC_R = crate::R<u8, u8>;
///Write proxy for field `FCRC`
pub struct FCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Reader of field `VSPOL`
pub type VSPOL_R = crate::R<bool, bool>;
///Write proxy for field `VSPOL`
pub struct VSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSPOL_W<'a> {
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
///Reader of field `HSPOL`
pub type HSPOL_R = crate::R<bool, bool>;
///Write proxy for field `HSPOL`
pub struct HSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPOL_W<'a> {
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
///Reader of field `PCKPOL`
pub type PCKPOL_R = crate::R<bool, bool>;
///Write proxy for field `PCKPOL`
pub struct PCKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCKPOL_W<'a> {
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
///Reader of field `ESS`
pub type ESS_R = crate::R<bool, bool>;
///Write proxy for field `ESS`
pub struct ESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ESS_W<'a> {
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
///Reader of field `JPEG`
pub type JPEG_R = crate::R<bool, bool>;
///Write proxy for field `JPEG`
pub struct JPEG_W<'a> {
    w: &'a mut W,
}
impl<'a> JPEG_W<'a> {
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
///Reader of field `CROP`
pub type CROP_R = crate::R<bool, bool>;
///Write proxy for field `CROP`
pub struct CROP_W<'a> {
    w: &'a mut W,
}
impl<'a> CROP_W<'a> {
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
///Reader of field `CM`
pub type CM_R = crate::R<bool, bool>;
///Write proxy for field `CM`
pub struct CM_W<'a> {
    w: &'a mut W,
}
impl<'a> CM_W<'a> {
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
///Reader of field `CAPTURE`
pub type CAPTURE_R = crate::R<bool, bool>;
///Write proxy for field `CAPTURE`
pub struct CAPTURE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTURE_W<'a> {
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
///Reader of field `OELS`
pub type OELS_R = crate::R<bool, bool>;
///Write proxy for field `OELS`
pub struct OELS_W<'a> {
    w: &'a mut W,
}
impl<'a> OELS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///Reader of field `LSM`
pub type LSM_R = crate::R<bool, bool>;
///Write proxy for field `LSM`
pub struct LSM_W<'a> {
    w: &'a mut W,
}
impl<'a> LSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Reader of field `OEBS`
pub type OEBS_R = crate::R<bool, bool>;
///Write proxy for field `OEBS`
pub struct OEBS_W<'a> {
    w: &'a mut W,
}
impl<'a> OEBS_W<'a> {
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
///Reader of field `BSM`
pub type BSM_R = crate::R<u8, u8>;
///Write proxy for field `BSM`
pub struct BSM_W<'a> {
    w: &'a mut W,
}
impl<'a> BSM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    ///Bit 14 - DCMI enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Capture enable
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 20 - Odd/Even Line Select (Line Select Start)
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - Line Select mode
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - Odd/Even Byte Select (Byte Select Start)
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bits 16:17 - Byte Select mode
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    ///Bit 14 - DCMI enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&mut self) -> EDM_W {
        EDM_W { w: self }
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    pub fn fcrc(&mut self) -> FCRC_W {
        FCRC_W { w: self }
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W {
        VSPOL_W { w: self }
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W {
        HSPOL_W { w: self }
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    pub fn pckpol(&mut self) -> PCKPOL_W {
        PCKPOL_W { w: self }
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    pub fn ess(&mut self) -> ESS_W {
        ESS_W { w: self }
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    pub fn jpeg(&mut self) -> JPEG_W {
        JPEG_W { w: self }
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    pub fn crop(&mut self) -> CROP_W {
        CROP_W { w: self }
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W { w: self }
    }
    ///Bit 0 - Capture enable
    #[inline(always)]
    pub fn capture(&mut self) -> CAPTURE_W {
        CAPTURE_W { w: self }
    }
    ///Bit 20 - Odd/Even Line Select (Line Select Start)
    #[inline(always)]
    pub fn oels(&mut self) -> OELS_W {
        OELS_W { w: self }
    }
    ///Bit 19 - Line Select mode
    #[inline(always)]
    pub fn lsm(&mut self) -> LSM_W {
        LSM_W { w: self }
    }
    ///Bit 18 - Odd/Even Byte Select (Byte Select Start)
    #[inline(always)]
    pub fn oebs(&mut self) -> OEBS_W {
        OEBS_W { w: self }
    }
    ///Bits 16:17 - Byte Select mode
    #[inline(always)]
    pub fn bsm(&mut self) -> BSM_W {
        BSM_W { w: self }
    }
}
