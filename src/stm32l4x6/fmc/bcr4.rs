///Reader of register BCR4
pub type R = crate::R<u32, super::BCR4>;
///Writer for register BCR4
pub type W = crate::W<u32, super::BCR4>;
///Register BCR4 `reset()`'s with value 0x30d0
impl crate::ResetValue for super::BCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30d0
    }
}
///Reader of field `CBURSTRW`
pub type CBURSTRW_R = crate::R<bool, bool>;
///Write proxy for field `CBURSTRW`
pub struct CBURSTRW_W<'a> {
    w: &'a mut W,
}
impl<'a> CBURSTRW_W<'a> {
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
///Reader of field `ASYNCWAIT`
pub type ASYNCWAIT_R = crate::R<bool, bool>;
///Write proxy for field `ASYNCWAIT`
pub struct ASYNCWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCWAIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Reader of field `EXTMOD`
pub type EXTMOD_R = crate::R<bool, bool>;
///Write proxy for field `EXTMOD`
pub struct EXTMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMOD_W<'a> {
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
///Reader of field `WAITEN`
pub type WAITEN_R = crate::R<bool, bool>;
///Write proxy for field `WAITEN`
pub struct WAITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `WREN`
pub type WREN_R = crate::R<bool, bool>;
///Write proxy for field `WREN`
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
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
///Reader of field `WAITCFG`
pub type WAITCFG_R = crate::R<bool, bool>;
///Write proxy for field `WAITCFG`
pub struct WAITCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITCFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `WRAPMOD`
pub type WRAPMOD_R = crate::R<bool, bool>;
///Write proxy for field `WRAPMOD`
pub struct WRAPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAPMOD_W<'a> {
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
///Reader of field `WAITPOL`
pub type WAITPOL_R = crate::R<bool, bool>;
///Write proxy for field `WAITPOL`
pub struct WAITPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITPOL_W<'a> {
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
///Reader of field `BURSTEN`
pub type BURSTEN_R = crate::R<bool, bool>;
///Write proxy for field `BURSTEN`
pub struct BURSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTEN_W<'a> {
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
///Reader of field `FACCEN`
pub type FACCEN_R = crate::R<bool, bool>;
///Write proxy for field `FACCEN`
pub struct FACCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FACCEN_W<'a> {
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
///Reader of field `MWID`
pub type MWID_R = crate::R<u8, u8>;
///Write proxy for field `MWID`
pub struct MWID_W<'a> {
    w: &'a mut W,
}
impl<'a> MWID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
///Reader of field `MTYP`
pub type MTYP_R = crate::R<u8, u8>;
///Write proxy for field `MTYP`
pub struct MTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> MTYP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Reader of field `MUXEN`
pub type MUXEN_R = crate::R<bool, bool>;
///Write proxy for field `MUXEN`
pub struct MUXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MUXEN_W<'a> {
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
///Reader of field `MBKEN`
pub type MBKEN_R = crate::R<bool, bool>;
///Write proxy for field `MBKEN`
pub struct MBKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MBKEN_W<'a> {
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
    ///Bit 19 - CBURSTRW
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 15 - ASYNCWAIT
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - EXTMOD
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - WAITEN
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - WREN
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - WAITCFG
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - WRAPMOD
    #[inline(always)]
    pub fn wrapmod(&self) -> WRAPMOD_R {
        WRAPMOD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - WAITPOL
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - BURSTEN
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 6 - FACCEN
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 4:5 - MWID
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bits 2:3 - MTYP
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bit 1 - MUXEN
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - MBKEN
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 19 - CBURSTRW
    #[inline(always)]
    pub fn cburstrw(&mut self) -> CBURSTRW_W {
        CBURSTRW_W { w: self }
    }
    ///Bit 15 - ASYNCWAIT
    #[inline(always)]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W {
        ASYNCWAIT_W { w: self }
    }
    ///Bit 14 - EXTMOD
    #[inline(always)]
    pub fn extmod(&mut self) -> EXTMOD_W {
        EXTMOD_W { w: self }
    }
    ///Bit 13 - WAITEN
    #[inline(always)]
    pub fn waiten(&mut self) -> WAITEN_W {
        WAITEN_W { w: self }
    }
    ///Bit 12 - WREN
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    ///Bit 11 - WAITCFG
    #[inline(always)]
    pub fn waitcfg(&mut self) -> WAITCFG_W {
        WAITCFG_W { w: self }
    }
    ///Bit 10 - WRAPMOD
    #[inline(always)]
    pub fn wrapmod(&mut self) -> WRAPMOD_W {
        WRAPMOD_W { w: self }
    }
    ///Bit 9 - WAITPOL
    #[inline(always)]
    pub fn waitpol(&mut self) -> WAITPOL_W {
        WAITPOL_W { w: self }
    }
    ///Bit 8 - BURSTEN
    #[inline(always)]
    pub fn bursten(&mut self) -> BURSTEN_W {
        BURSTEN_W { w: self }
    }
    ///Bit 6 - FACCEN
    #[inline(always)]
    pub fn faccen(&mut self) -> FACCEN_W {
        FACCEN_W { w: self }
    }
    ///Bits 4:5 - MWID
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W {
        MWID_W { w: self }
    }
    ///Bits 2:3 - MTYP
    #[inline(always)]
    pub fn mtyp(&mut self) -> MTYP_W {
        MTYP_W { w: self }
    }
    ///Bit 1 - MUXEN
    #[inline(always)]
    pub fn muxen(&mut self) -> MUXEN_W {
        MUXEN_W { w: self }
    }
    ///Bit 0 - MBKEN
    #[inline(always)]
    pub fn mbken(&mut self) -> MBKEN_W {
        MBKEN_W { w: self }
    }
}
