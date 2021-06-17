///Reader of register HPRT
pub type R = crate::R<u32, super::HPRT>;
///Writer for register HPRT
pub type W = crate::W<u32, super::HPRT>;
///Register HPRT `reset()`'s with value 0
impl crate::ResetValue for super::HPRT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PCSTS`
pub type PCSTS_R = crate::R<bool, bool>;
///Reader of field `PCDET`
pub type PCDET_R = crate::R<bool, bool>;
///Write proxy for field `PCDET`
pub struct PCDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PCDET_W<'a> {
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
///Reader of field `PENA`
pub type PENA_R = crate::R<bool, bool>;
///Write proxy for field `PENA`
pub struct PENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PENA_W<'a> {
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
///Reader of field `PENCHNG`
pub type PENCHNG_R = crate::R<bool, bool>;
///Write proxy for field `PENCHNG`
pub struct PENCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PENCHNG_W<'a> {
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
///Reader of field `POCA`
pub type POCA_R = crate::R<bool, bool>;
///Reader of field `POCCHNG`
pub type POCCHNG_R = crate::R<bool, bool>;
///Write proxy for field `POCCHNG`
pub struct POCCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> POCCHNG_W<'a> {
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
///Reader of field `PRES`
pub type PRES_R = crate::R<bool, bool>;
///Write proxy for field `PRES`
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
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
///Reader of field `PSUSP`
pub type PSUSP_R = crate::R<bool, bool>;
///Write proxy for field `PSUSP`
pub struct PSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PSUSP_W<'a> {
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
///Reader of field `PRST`
pub type PRST_R = crate::R<bool, bool>;
///Write proxy for field `PRST`
pub struct PRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST_W<'a> {
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
///Reader of field `PLSTS`
pub type PLSTS_R = crate::R<u8, u8>;
///Reader of field `PPWR`
pub type PPWR_R = crate::R<bool, bool>;
///Write proxy for field `PPWR`
pub struct PPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PPWR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `PTCTL`
pub type PTCTL_R = crate::R<u8, u8>;
///Write proxy for field `PTCTL`
pub struct PTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCTL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | (((value as u32) & 0x0f) << 13);
        self.w
    }
}
///Reader of field `PSPD`
pub type PSPD_R = crate::R<u8, u8>;
impl R {
    ///Bit 0 - Port connect status
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Port connect detected
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Port enable
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Port enable/disable change
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Port overcurrent active
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Port overcurrent change
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Port resume
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Port suspend
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Port reset
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 10:11 - Port line status
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bit 12 - Port power
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bits 13:16 - Port test control
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:18 - Port speed
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    ///Bit 1 - Port connect detected
    #[inline(always)]
    pub fn pcdet(&mut self) -> PCDET_W {
        PCDET_W { w: self }
    }
    ///Bit 2 - Port enable
    #[inline(always)]
    pub fn pena(&mut self) -> PENA_W {
        PENA_W { w: self }
    }
    ///Bit 3 - Port enable/disable change
    #[inline(always)]
    pub fn penchng(&mut self) -> PENCHNG_W {
        PENCHNG_W { w: self }
    }
    ///Bit 5 - Port overcurrent change
    #[inline(always)]
    pub fn pocchng(&mut self) -> POCCHNG_W {
        POCCHNG_W { w: self }
    }
    ///Bit 6 - Port resume
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    ///Bit 7 - Port suspend
    #[inline(always)]
    pub fn psusp(&mut self) -> PSUSP_W {
        PSUSP_W { w: self }
    }
    ///Bit 8 - Port reset
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W {
        PRST_W { w: self }
    }
    ///Bit 12 - Port power
    #[inline(always)]
    pub fn ppwr(&mut self) -> PPWR_W {
        PPWR_W { w: self }
    }
    ///Bits 13:16 - Port test control
    #[inline(always)]
    pub fn ptctl(&mut self) -> PTCTL_W {
        PTCTL_W { w: self }
    }
}
