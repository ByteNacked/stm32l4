///Reader of register PDCRD
pub type R = crate::R<u32, super::PDCRD>;
///Writer for register PDCRD
pub type W = crate::W<u32, super::PDCRD>;
///Register PDCRD `reset()`'s with value 0
impl crate::ResetValue for super::PDCRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PD15`
pub type PD15_R = crate::R<bool, bool>;
///Write proxy for field `PD15`
pub struct PD15_W<'a> {
    w: &'a mut W,
}
impl<'a> PD15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
///Reader of field `PD14`
pub type PD14_R = crate::R<bool, bool>;
///Write proxy for field `PD14`
pub struct PD14_W<'a> {
    w: &'a mut W,
}
impl<'a> PD14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
///Reader of field `PD13`
pub type PD13_R = crate::R<bool, bool>;
///Write proxy for field `PD13`
pub struct PD13_W<'a> {
    w: &'a mut W,
}
impl<'a> PD13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `PD12`
pub type PD12_R = crate::R<bool, bool>;
///Write proxy for field `PD12`
pub struct PD12_W<'a> {
    w: &'a mut W,
}
impl<'a> PD12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `PD11`
pub type PD11_R = crate::R<bool, bool>;
///Write proxy for field `PD11`
pub struct PD11_W<'a> {
    w: &'a mut W,
}
impl<'a> PD11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `PD10`
pub type PD10_R = crate::R<bool, bool>;
///Write proxy for field `PD10`
pub struct PD10_W<'a> {
    w: &'a mut W,
}
impl<'a> PD10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
///Reader of field `PD9`
pub type PD9_R = crate::R<bool, bool>;
///Write proxy for field `PD9`
pub struct PD9_W<'a> {
    w: &'a mut W,
}
impl<'a> PD9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `PD8`
pub type PD8_R = crate::R<bool, bool>;
///Write proxy for field `PD8`
pub struct PD8_W<'a> {
    w: &'a mut W,
}
impl<'a> PD8_W<'a> {
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
///Reader of field `PD7`
pub type PD7_R = crate::R<bool, bool>;
///Write proxy for field `PD7`
pub struct PD7_W<'a> {
    w: &'a mut W,
}
impl<'a> PD7_W<'a> {
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
///Reader of field `PD6`
pub type PD6_R = crate::R<bool, bool>;
///Write proxy for field `PD6`
pub struct PD6_W<'a> {
    w: &'a mut W,
}
impl<'a> PD6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Reader of field `PD5`
pub type PD5_R = crate::R<bool, bool>;
///Write proxy for field `PD5`
pub struct PD5_W<'a> {
    w: &'a mut W,
}
impl<'a> PD5_W<'a> {
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
///Reader of field `PD4`
pub type PD4_R = crate::R<bool, bool>;
///Write proxy for field `PD4`
pub struct PD4_W<'a> {
    w: &'a mut W,
}
impl<'a> PD4_W<'a> {
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
///Reader of field `PD3`
pub type PD3_R = crate::R<bool, bool>;
///Write proxy for field `PD3`
pub struct PD3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD3_W<'a> {
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
///Reader of field `PD2`
pub type PD2_R = crate::R<bool, bool>;
///Write proxy for field `PD2`
pub struct PD2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD2_W<'a> {
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
    ///Bit 15 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 13 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 12 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 15 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W {
        PD15_W { w: self }
    }
    ///Bit 14 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W {
        PD14_W { w: self }
    }
    ///Bit 13 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W {
        PD13_W { w: self }
    }
    ///Bit 12 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W {
        PD12_W { w: self }
    }
    ///Bit 11 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W {
        PD11_W { w: self }
    }
    ///Bit 10 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W {
        PD10_W { w: self }
    }
    ///Bit 9 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W {
        PD9_W { w: self }
    }
    ///Bit 8 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W {
        PD8_W { w: self }
    }
    ///Bit 7 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W {
        PD7_W { w: self }
    }
    ///Bit 6 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W {
        PD6_W { w: self }
    }
    ///Bit 5 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W {
        PD5_W { w: self }
    }
    ///Bit 4 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W {
        PD4_W { w: self }
    }
    ///Bit 3 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W {
        PD3_W { w: self }
    }
    ///Bit 2 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W {
        PD2_W { w: self }
    }
    ///Bit 1 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W {
        PD1_W { w: self }
    }
    ///Bit 0 - Port D pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W {
        PD0_W { w: self }
    }
}
