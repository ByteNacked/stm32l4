///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0x01
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
///Reader of field `BUSY`
pub type BUSY_R = crate::R<bool, bool>;
///Reader of field `DMAS`
pub type DMAS_R = crate::R<bool, bool>;
///Reader of field `DCIS`
pub type DCIS_R = crate::R<bool, bool>;
///Write proxy for field `DCIS`
pub struct DCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCIS_W<'a> {
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
///Reader of field `DINIS`
pub type DINIS_R = crate::R<bool, bool>;
///Write proxy for field `DINIS`
pub struct DINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DINIS_W<'a> {
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
    ///Bit 3 - Busy bit
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - DMA Status
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Digest calculation completion interrupt status
    #[inline(always)]
    pub fn dcis(&self) -> DCIS_R {
        DCIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Data input interrupt status
    #[inline(always)]
    pub fn dinis(&self) -> DINIS_R {
        DINIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Digest calculation completion interrupt status
    #[inline(always)]
    pub fn dcis(&mut self) -> DCIS_W {
        DCIS_W { w: self }
    }
    ///Bit 0 - Data input interrupt status
    #[inline(always)]
    pub fn dinis(&mut self) -> DINIS_W {
        DINIS_W { w: self }
    }
}
