///Reader of register CFGR1
pub type R = crate::R<u32, super::CFGR1>;
///Writer for register CFGR1
pub type W = crate::W<u32, super::CFGR1>;
///Register CFGR1 `reset()`'s with value 0
impl crate::ResetValue for super::CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DFSDMEN`
pub type DFSDMEN_R = crate::R<bool, bool>;
///Write proxy for field `DFSDMEN`
pub struct DFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMEN_W<'a> {
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
///Reader of field `CKOUTSRC`
pub type CKOUTSRC_R = crate::R<bool, bool>;
///Write proxy for field `CKOUTSRC`
pub struct CKOUTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTSRC_W<'a> {
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
///Reader of field `CKOUTDIV`
pub type CKOUTDIV_R = crate::R<u8, u8>;
///Write proxy for field `CKOUTDIV`
pub struct CKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `DATPACK`
pub type DATPACK_R = crate::R<u8, u8>;
///Write proxy for field `DATPACK`
pub struct DATPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATPACK_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
///Reader of field `DATMPX`
pub type DATMPX_R = crate::R<u8, u8>;
///Write proxy for field `DATMPX`
pub struct DATMPX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMPX_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
///Reader of field `CHINSEL`
pub type CHINSEL_R = crate::R<bool, bool>;
///Write proxy for field `CHINSEL`
pub struct CHINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHINSEL_W<'a> {
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
///Reader of field `CHEN`
pub type CHEN_R = crate::R<bool, bool>;
///Write proxy for field `CHEN`
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
///Reader of field `CKABEN`
pub type CKABEN_R = crate::R<bool, bool>;
///Write proxy for field `CKABEN`
pub struct CKABEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKABEN_W<'a> {
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
///Reader of field `SCDEN`
pub type SCDEN_R = crate::R<bool, bool>;
///Write proxy for field `SCDEN`
pub struct SCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDEN_W<'a> {
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
///Reader of field `SPICKSEL`
pub type SPICKSEL_R = crate::R<u8, u8>;
///Write proxy for field `SPICKSEL`
pub struct SPICKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPICKSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `SITP`
pub type SITP_R = crate::R<u8, u8>;
///Write proxy for field `SITP`
pub struct SITP_W<'a> {
    w: &'a mut W,
}
impl<'a> SITP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bit 31 - DFSDMEN
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    ///Bit 30 - CKOUTSRC
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bits 16:23 - CKOUTDIV
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 14:15 - DATPACK
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 12:13 - DATMPX
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bit 8 - CHINSEL
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - CHEN
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - CKABEN
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - SCDEN
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 2:3 - SPICKSEL
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - SITP
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bit 31 - DFSDMEN
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W {
        DFSDMEN_W { w: self }
    }
    ///Bit 30 - CKOUTSRC
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W {
        CKOUTSRC_W { w: self }
    }
    ///Bits 16:23 - CKOUTDIV
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W {
        CKOUTDIV_W { w: self }
    }
    ///Bits 14:15 - DATPACK
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W {
        DATPACK_W { w: self }
    }
    ///Bits 12:13 - DATMPX
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W {
        DATMPX_W { w: self }
    }
    ///Bit 8 - CHINSEL
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W {
        CHINSEL_W { w: self }
    }
    ///Bit 7 - CHEN
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    ///Bit 6 - CKABEN
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W {
        CKABEN_W { w: self }
    }
    ///Bit 5 - SCDEN
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W {
        SCDEN_W { w: self }
    }
    ///Bits 2:3 - SPICKSEL
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W {
        SPICKSEL_W { w: self }
    }
    ///Bits 0:1 - SITP
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W {
        SITP_W { w: self }
    }
}
