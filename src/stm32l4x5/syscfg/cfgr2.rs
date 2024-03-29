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
///Reader of field `SPF`
pub type SPF_R = crate::R<bool, bool>;
///Write proxy for field `SPF`
pub struct SPF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPF_W<'a> {
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
///Write proxy for field `ECCL`
pub struct ECCL_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCL_W<'a> {
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
///Write proxy for field `PVDL`
pub struct PVDL_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDL_W<'a> {
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
///Write proxy for field `SPL`
pub struct SPL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPL_W<'a> {
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
///Write proxy for field `CLL`
pub struct CLL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLL_W<'a> {
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
    ///Bit 8 - SRAM2 parity error flag
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 8 - SRAM2 parity error flag
    #[inline(always)]
    pub fn spf(&mut self) -> SPF_W {
        SPF_W { w: self }
    }
    ///Bit 3 - ECC Lock
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W {
        ECCL_W { w: self }
    }
    ///Bit 2 - PVD lock enable bit
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W {
        PVDL_W { w: self }
    }
    ///Bit 1 - SRAM2 parity lock bit
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W {
        SPL_W { w: self }
    }
    ///Bit 0 - Cortex LOCKUP (Hardfault) output enable bit
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W {
        CLL_W { w: self }
    }
}
