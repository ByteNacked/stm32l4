///Reader of register DOEPINT0
pub type R = crate::R<u32, super::DOEPINT0>;
///Writer for register DOEPINT0
pub type W = crate::W<u32, super::DOEPINT0>;
///Register DOEPINT0 `reset()`'s with value 0x80
impl crate::ResetValue for super::DOEPINT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
///Reader of field `B2BSTUP`
pub type B2BSTUP_R = crate::R<bool, bool>;
///Write proxy for field `B2BSTUP`
pub struct B2BSTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> B2BSTUP_W<'a> {
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
///Reader of field `OTEPDIS`
pub type OTEPDIS_R = crate::R<bool, bool>;
///Write proxy for field `OTEPDIS`
pub struct OTEPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTEPDIS_W<'a> {
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
///Reader of field `STUP`
pub type STUP_R = crate::R<bool, bool>;
///Write proxy for field `STUP`
pub struct STUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STUP_W<'a> {
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
///Reader of field `EPDISD`
pub type EPDISD_R = crate::R<bool, bool>;
///Write proxy for field `EPDISD`
pub struct EPDISD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISD_W<'a> {
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
///Reader of field `XFRC`
pub type XFRC_R = crate::R<bool, bool>;
///Write proxy for field `XFRC`
pub struct XFRC_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRC_W<'a> {
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
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W {
        B2BSTUP_W { w: self }
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W {
        OTEPDIS_W { w: self }
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W {
        STUP_W { w: self }
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W {
        EPDISD_W { w: self }
    }
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W { w: self }
    }
}
