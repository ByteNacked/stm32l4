///Reader of register DSI_CLCR
pub type R = crate::R<u32, super::DSI_CLCR>;
///Writer for register DSI_CLCR
pub type W = crate::W<u32, super::DSI_CLCR>;
///Register DSI_CLCR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_CLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DPCC`
pub type DPCC_R = crate::R<bool, bool>;
///Write proxy for field `DPCC`
pub struct DPCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DPCC_W<'a> {
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
///Reader of field `ACR`
pub type ACR_R = crate::R<bool, bool>;
///Write proxy for field `ACR`
pub struct ACR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACR_W<'a> {
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
impl R {
    ///Bit 0 - D-PHY Clock Control
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Automatic Clock lane Control
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - D-PHY Clock Control
    #[inline(always)]
    pub fn dpcc(&mut self) -> DPCC_W {
        DPCC_W { w: self }
    }
    ///Bit 1 - Automatic Clock lane Control
    #[inline(always)]
    pub fn acr(&mut self) -> ACR_W {
        ACR_W { w: self }
    }
}
