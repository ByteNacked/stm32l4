///Reader of register CFGR2
pub type R = crate::R<u32, super::CFGR2>;
///Writer for register CFGR2
pub type W = crate::W<u32, super::CFGR2>;
///Register CFGR2 `reset()`'s with value 0
impl crate::ResetValue for super::CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ROVSM`
pub type ROVSM_R = crate::R<bool, bool>;
///Write proxy for field `ROVSM`
pub struct ROVSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Reader of field `TOVS`
pub type TOVS_R = crate::R<bool, bool>;
///Write proxy for field `TOVS`
pub struct TOVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVS_W<'a> {
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
///Reader of field `OVSS`
pub type OVSS_R = crate::R<u8, u8>;
///Write proxy for field `OVSS`
pub struct OVSS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
///Reader of field `OVSR`
pub type OVSR_R = crate::R<u8, u8>;
///Write proxy for field `OVSR`
pub struct OVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
///Reader of field `JOVSE`
pub type JOVSE_R = crate::R<bool, bool>;
///Write proxy for field `JOVSE`
pub struct JOVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> JOVSE_W<'a> {
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
///Reader of field `ROVSE`
pub type ROVSE_R = crate::R<bool, bool>;
///Write proxy for field `ROVSE`
pub struct ROVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSE_W<'a> {
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
    ///Bit 10 - EXTEN
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - EXTSEL
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bits 5:8 - ALIGN
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bits 2:4 - RES
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - DMAEN
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 10 - EXTEN
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W {
        ROVSM_W { w: self }
    }
    ///Bit 9 - EXTSEL
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W { w: self }
    }
    ///Bits 5:8 - ALIGN
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W { w: self }
    }
    ///Bits 2:4 - RES
    #[inline(always)]
    pub fn ovsr(&mut self) -> OVSR_W {
        OVSR_W { w: self }
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W {
        JOVSE_W { w: self }
    }
    ///Bit 0 - DMAEN
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W {
        ROVSE_W { w: self }
    }
}
