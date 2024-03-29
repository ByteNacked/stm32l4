///Reader of register FS_GRSTCTL
pub type R = crate::R<u32, super::FS_GRSTCTL>;
///Writer for register FS_GRSTCTL
pub type W = crate::W<u32, super::FS_GRSTCTL>;
///Register FS_GRSTCTL `reset()`'s with value 0x2000_0000
impl crate::ResetValue for super::FS_GRSTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_0000
    }
}
///Reader of field `CSRST`
pub type CSRST_R = crate::R<bool, bool>;
///Write proxy for field `CSRST`
pub struct CSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRST_W<'a> {
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
///Reader of field `HSRST`
pub type HSRST_R = crate::R<bool, bool>;
///Write proxy for field `HSRST`
pub struct HSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRST_W<'a> {
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
///Reader of field `FCRST`
pub type FCRST_R = crate::R<bool, bool>;
///Write proxy for field `FCRST`
pub struct FCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FCRST_W<'a> {
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
///Reader of field `RXFFLSH`
pub type RXFFLSH_R = crate::R<bool, bool>;
///Write proxy for field `RXFFLSH`
pub struct RXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFLSH_W<'a> {
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
///Reader of field `TXFFLSH`
pub type TXFFLSH_R = crate::R<bool, bool>;
///Write proxy for field `TXFFLSH`
pub struct TXFFLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFFLSH_W<'a> {
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
///Reader of field `TXFNUM`
pub type TXFNUM_R = crate::R<u8, u8>;
///Write proxy for field `TXFNUM`
pub struct TXFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFNUM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
///Reader of field `AHBIDL`
pub type AHBIDL_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - HCLK soft reset
    #[inline(always)]
    pub fn hsrst(&self) -> HSRST_R {
        HSRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    pub fn fcrst(&self) -> FCRST_R {
        FCRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 4 - RxFIFO flush
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - TxFIFO flush
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 6:10 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 31 - AHB master idle
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Core soft reset
    #[inline(always)]
    pub fn csrst(&mut self) -> CSRST_W {
        CSRST_W { w: self }
    }
    ///Bit 1 - HCLK soft reset
    #[inline(always)]
    pub fn hsrst(&mut self) -> HSRST_W {
        HSRST_W { w: self }
    }
    ///Bit 2 - Host frame counter reset
    #[inline(always)]
    pub fn fcrst(&mut self) -> FCRST_W {
        FCRST_W { w: self }
    }
    ///Bit 4 - RxFIFO flush
    #[inline(always)]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W {
        RXFFLSH_W { w: self }
    }
    ///Bit 5 - TxFIFO flush
    #[inline(always)]
    pub fn txfflsh(&mut self) -> TXFFLSH_W {
        TXFFLSH_W { w: self }
    }
    ///Bits 6:10 - TxFIFO number
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W {
        TXFNUM_W { w: self }
    }
}
