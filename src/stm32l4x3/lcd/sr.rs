///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0x20
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
///Reader of field `FCRSF`
pub type FCRSF_R = crate::R<bool, bool>;
///Reader of field `RDY`
pub type RDY_R = crate::R<bool, bool>;
///Reader of field `UDD`
pub type UDD_R = crate::R<bool, bool>;
///Write proxy for field `UDR`
pub struct UDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UDR_W<'a> {
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
///Reader of field `SOF`
pub type SOF_R = crate::R<bool, bool>;
///Reader of field `ENS`
pub type ENS_R = crate::R<bool, bool>;
impl R {
    ///Bit 5 - LCD Frame Control Register Synchronization flag
    #[inline(always)]
    pub fn fcrsf(&self) -> FCRSF_R {
        FCRSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Ready flag
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Update Display Done
    #[inline(always)]
    pub fn udd(&self) -> UDD_R {
        UDD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 1 - Start of frame flag
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - ENS
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Update display request
    #[inline(always)]
    pub fn udr(&mut self) -> UDR_W {
        UDR_W { w: self }
    }
}
