///Reader of register FS_PCGCCTL
pub type R = crate::R<u32, super::FS_PCGCCTL>;
///Writer for register FS_PCGCCTL
pub type W = crate::W<u32, super::FS_PCGCCTL>;
///Register FS_PCGCCTL `reset()`'s with value 0
impl crate::ResetValue for super::FS_PCGCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `STPPCLK`
pub type STPPCLK_R = crate::R<bool, bool>;
///Write proxy for field `STPPCLK`
pub struct STPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STPPCLK_W<'a> {
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
///Reader of field `GATEHCLK`
pub type GATEHCLK_R = crate::R<bool, bool>;
///Write proxy for field `GATEHCLK`
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
///Reader of field `PHYSUSP`
pub type PHYSUSP_R = crate::R<bool, bool>;
///Write proxy for field `PHYSUSP`
pub struct PHYSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSUSP_W<'a> {
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
impl R {
    ///Bit 0 - Stop PHY clock
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Gate HCLK
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - PHY Suspended
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Stop PHY clock
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W {
        STPPCLK_W { w: self }
    }
    ///Bit 1 - Gate HCLK
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
    ///Bit 4 - PHY Suspended
    #[inline(always)]
    pub fn physusp(&mut self) -> PHYSUSP_W {
        PHYSUSP_W { w: self }
    }
}
