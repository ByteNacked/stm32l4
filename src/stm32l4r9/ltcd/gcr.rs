///Reader of register GCR
pub type R = crate::R<u32, super::GCR>;
///Writer for register GCR
pub type W = crate::W<u32, super::GCR>;
///Register GCR `reset()`'s with value 0x2220
impl crate::ResetValue for super::GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2220
    }
}
///Reader of field `LTDCEN`
pub type LTDCEN_R = crate::R<bool, bool>;
///Write proxy for field `LTDCEN`
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
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
///Reader of field `DBW`
pub type DBW_R = crate::R<u8, u8>;
///Reader of field `DGW`
pub type DGW_R = crate::R<u8, u8>;
///Reader of field `DRW`
pub type DRW_R = crate::R<u8, u8>;
///Reader of field `DEN`
pub type DEN_R = crate::R<bool, bool>;
///Write proxy for field `DEN`
pub struct DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEN_W<'a> {
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
///Reader of field `PCPOL`
pub type PCPOL_R = crate::R<bool, bool>;
///Write proxy for field `PCPOL`
pub struct PCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
///Reader of field `DEPOL`
pub type DEPOL_R = crate::R<bool, bool>;
///Write proxy for field `DEPOL`
pub struct DEPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - LCD-TFT controller enable bit
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 4:6 - Dither Blue Width
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bits 8:10 - Dither Green Width
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bits 12:14 - Dither Red Width
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bit 16 - Dither Enable
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 28 - Pixel Clock Polarity
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - Not Data Enable Polarity
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - Vertical Synchronization Polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Horizontal Synchronization Polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - LCD-TFT controller enable bit
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
    }
    ///Bit 16 - Dither Enable
    #[inline(always)]
    pub fn den(&mut self) -> DEN_W {
        DEN_W { w: self }
    }
    ///Bit 28 - Pixel Clock Polarity
    #[inline(always)]
    pub fn pcpol(&mut self) -> PCPOL_W {
        PCPOL_W { w: self }
    }
    ///Bit 29 - Not Data Enable Polarity
    #[inline(always)]
    pub fn depol(&mut self) -> DEPOL_W {
        DEPOL_W { w: self }
    }
    ///Bit 30 - Vertical Synchronization Polarity
    #[inline(always)]
    pub fn vspol(&mut self) -> VSPOL_W {
        VSPOL_W { w: self }
    }
    ///Bit 31 - Horizontal Synchronization Polarity
    #[inline(always)]
    pub fn hspol(&mut self) -> HSPOL_W {
        HSPOL_W { w: self }
    }
}
