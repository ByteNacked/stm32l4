///Reader of register ACTRL
pub type R = crate::R<u32, super::ACTRL>;
///Writer for register ACTRL
pub type W = crate::W<u32, super::ACTRL>;
///Register ACTRL `reset()`'s with value 0
impl crate::ResetValue for super::ACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DISMCYCINT`
pub type DISMCYCINT_R = crate::R<bool, bool>;
///Write proxy for field `DISMCYCINT`
pub struct DISMCYCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMCYCINT_W<'a> {
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
///Reader of field `DISDEFWBUF`
pub type DISDEFWBUF_R = crate::R<bool, bool>;
///Write proxy for field `DISDEFWBUF`
pub struct DISDEFWBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> DISDEFWBUF_W<'a> {
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
///Reader of field `DISFOLD`
pub type DISFOLD_R = crate::R<bool, bool>;
///Write proxy for field `DISFOLD`
pub struct DISFOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFOLD_W<'a> {
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
///Reader of field `DISFPCA`
pub type DISFPCA_R = crate::R<bool, bool>;
///Write proxy for field `DISFPCA`
pub struct DISFPCA_W<'a> {
    w: &'a mut W,
}
impl<'a> DISFPCA_W<'a> {
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
///Reader of field `DISOOFP`
pub type DISOOFP_R = crate::R<bool, bool>;
///Write proxy for field `DISOOFP`
pub struct DISOOFP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISOOFP_W<'a> {
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
impl R {
    ///Bit 0 - DISMCYCINT
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - DISDEFWBUF
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - DISFOLD
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 8 - DISFPCA
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - DISOOFP
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - DISMCYCINT
    #[inline(always)]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W {
        DISMCYCINT_W { w: self }
    }
    ///Bit 1 - DISDEFWBUF
    #[inline(always)]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W {
        DISDEFWBUF_W { w: self }
    }
    ///Bit 2 - DISFOLD
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W {
        DISFOLD_W { w: self }
    }
    ///Bit 8 - DISFPCA
    #[inline(always)]
    pub fn disfpca(&mut self) -> DISFPCA_W {
        DISFPCA_W { w: self }
    }
    ///Bit 9 - DISOOFP
    #[inline(always)]
    pub fn disoofp(&mut self) -> DISOOFP_W {
        DISOOFP_W { w: self }
    }
}
