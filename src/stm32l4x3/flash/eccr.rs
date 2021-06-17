///Reader of register ECCR
pub type R = crate::R<u32, super::ECCR>;
///Writer for register ECCR
pub type W = crate::W<u32, super::ECCR>;
///Register ECCR `reset()`'s with value 0
impl crate::ResetValue for super::ECCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ADDR_ECC`
pub type ADDR_ECC_R = crate::R<u32, u32>;
///Reader of field `BK_ECC`
pub type BK_ECC_R = crate::R<bool, bool>;
///Reader of field `SYSF_ECC`
pub type SYSF_ECC_R = crate::R<bool, bool>;
///Reader of field `ECCIE`
pub type ECCIE_R = crate::R<bool, bool>;
///Write proxy for field `ECCIE`
pub struct ECCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Reader of field `ECCC`
pub type ECCC_R = crate::R<bool, bool>;
///Write proxy for field `ECCC`
pub struct ECCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCC_W<'a> {
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
///Reader of field `ECCD`
pub type ECCD_R = crate::R<bool, bool>;
///Write proxy for field `ECCD`
pub struct ECCD_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCD_W<'a> {
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
impl R {
    ///Bits 0:18 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x0007_ffff) as u32)
    }
    ///Bit 19 - ECC fail bank
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - System Flash ECC fail
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable
    #[inline(always)]
    pub fn eccie(&mut self) -> ECCIE_W {
        ECCIE_W { w: self }
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&mut self) -> ECCC_W {
        ECCC_W { w: self }
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&mut self) -> ECCD_W {
        ECCD_W { w: self }
    }
}
