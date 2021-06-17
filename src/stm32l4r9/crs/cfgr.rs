///Reader of register CFGR
pub type R = crate::R<u32, super::CFGR>;
///Writer for register CFGR
pub type W = crate::W<u32, super::CFGR>;
///Register CFGR `reset()`'s with value 0x2022_bb7f
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2022_bb7f
    }
}
///Reader of field `SYNCPOL`
pub type SYNCPOL_R = crate::R<bool, bool>;
///Write proxy for field `SYNCPOL`
pub struct SYNCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCPOL_W<'a> {
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
///Reader of field `SYNCSRC`
pub type SYNCSRC_R = crate::R<u8, u8>;
///Write proxy for field `SYNCSRC`
pub struct SYNCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCSRC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///Reader of field `SYNCDIV`
pub type SYNCDIV_R = crate::R<u8, u8>;
///Write proxy for field `SYNCDIV`
pub struct SYNCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
///Reader of field `FELIM`
pub type FELIM_R = crate::R<u8, u8>;
///Write proxy for field `FELIM`
pub struct FELIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FELIM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `RELOAD`
pub type RELOAD_R = crate::R<u16, u16>;
///Write proxy for field `RELOAD`
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    ///Bit 31 - SYNC polarity selection
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bits 28:29 - SYNC signal source selection
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 24:26 - SYNC divider
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    ///Bits 16:23 - Frequency error limit
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 0:15 - Counter reload value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bit 31 - SYNC polarity selection
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W {
        SYNCPOL_W { w: self }
    }
    ///Bits 28:29 - SYNC signal source selection
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W {
        SYNCSRC_W { w: self }
    }
    ///Bits 24:26 - SYNC divider
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SYNCDIV_W {
        SYNCDIV_W { w: self }
    }
    ///Bits 16:23 - Frequency error limit
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W {
        FELIM_W { w: self }
    }
    ///Bits 0:15 - Counter reload value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
