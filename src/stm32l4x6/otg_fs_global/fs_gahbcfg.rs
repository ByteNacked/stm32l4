///Reader of register FS_GAHBCFG
pub type R = crate::R<u32, super::FS_GAHBCFG>;
///Writer for register FS_GAHBCFG
pub type W = crate::W<u32, super::FS_GAHBCFG>;
///Register FS_GAHBCFG `reset()`'s with value 0
impl crate::ResetValue for super::FS_GAHBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `GINT`
pub type GINT_R = crate::R<bool, bool>;
///Write proxy for field `GINT`
pub struct GINT_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT_W<'a> {
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
///Reader of field `TXFELVL`
pub type TXFELVL_R = crate::R<bool, bool>;
///Write proxy for field `TXFELVL`
pub struct TXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFELVL_W<'a> {
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
///Reader of field `PTXFELVL`
pub type PTXFELVL_R = crate::R<bool, bool>;
///Write proxy for field `PTXFELVL`
pub struct PTXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFELVL_W<'a> {
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
impl R {
    ///Bit 0 - Global interrupt mask
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 7 - TxFIFO empty level
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Periodic TxFIFO empty level
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Global interrupt mask
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W {
        GINT_W { w: self }
    }
    ///Bit 7 - TxFIFO empty level
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W {
        TXFELVL_W { w: self }
    }
    ///Bit 8 - Periodic TxFIFO empty level
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W {
        PTXFELVL_W { w: self }
    }
}
