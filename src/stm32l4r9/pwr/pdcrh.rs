///Reader of register PDCRH
pub type R = crate::R<u32, super::PDCRH>;
///Writer for register PDCRH
pub type W = crate::W<u32, super::PDCRH>;
///Register PDCRH `reset()`'s with value 0
impl crate::ResetValue for super::PDCRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PD1`
pub type PD1_R = crate::R<bool, bool>;
///Write proxy for field `PD1`
pub struct PD1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD1_W<'a> {
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
///Reader of field `PD0`
pub type PD0_R = crate::R<bool, bool>;
///Write proxy for field `PD0`
pub struct PD0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD0_W<'a> {
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
    ///Bit 1 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 1 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W { w: self }
    }
    ///Bit 0 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W { w: self }
    }
}
