///Reader of register GCCFG
pub type R = crate::R<u32, super::GCCFG>;
///Writer for register GCCFG
pub type W = crate::W<u32, super::GCCFG>;
///Register GCCFG `reset()`'s with value 0
impl crate::ResetValue for super::GCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PWRDWN`
pub type PWRDWN_R = crate::R<bool, bool>;
///Write proxy for field `PWRDWN`
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `VBUSASEN`
pub type VBUSASEN_R = crate::R<bool, bool>;
///Write proxy for field `VBUSASEN`
pub struct VBUSASEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSASEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Reader of field `VBUSBSEN`
pub type VBUSBSEN_R = crate::R<bool, bool>;
///Write proxy for field `VBUSBSEN`
pub struct VBUSBSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSBSEN_W<'a> {
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
///Reader of field `SOFOUTEN`
pub type SOFOUTEN_R = crate::R<bool, bool>;
///Write proxy for field `SOFOUTEN`
pub struct SOFOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFOUTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 18 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusasen(&self) -> VBUSASEN_R {
        VBUSASEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusbsen(&self) -> VBUSBSEN_R {
        VBUSBSEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - SOF output enable
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    ///Bit 18 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusasen(&mut self) -> VBUSASEN_W {
        VBUSASEN_W { w: self }
    }
    ///Bit 19 - Enable the VBUS sensing device
    #[inline(always)]
    pub fn vbusbsen(&mut self) -> VBUSBSEN_W {
        VBUSBSEN_W { w: self }
    }
    ///Bit 20 - SOF output enable
    #[inline(always)]
    pub fn sofouten(&mut self) -> SOFOUTEN_W {
        SOFOUTEN_W { w: self }
    }
}
