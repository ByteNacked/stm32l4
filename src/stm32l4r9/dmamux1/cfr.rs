///Writer for register CFR
pub type W = crate::W<u32, super::CFR>;
///Register CFR `reset()`'s with value 0
impl crate::ResetValue for super::CFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Write proxy for field `CSOF0`
pub struct CSOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF0_W<'a> {
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
///Write proxy for field `CSOF1`
pub struct CSOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF1_W<'a> {
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
///Write proxy for field `CSOF2`
pub struct CSOF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF2_W<'a> {
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
///Write proxy for field `CSOF3`
pub struct CSOF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF3_W<'a> {
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
///Write proxy for field `CSOF4`
pub struct CSOF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF4_W<'a> {
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
///Write proxy for field `CSOF5`
pub struct CSOF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF5_W<'a> {
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
///Write proxy for field `CSOF6`
pub struct CSOF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF6_W<'a> {
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
///Write proxy for field `CSOF7`
pub struct CSOF7_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF7_W<'a> {
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
///Write proxy for field `CSOF8`
pub struct CSOF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF8_W<'a> {
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
///Write proxy for field `CSOF9`
pub struct CSOF9_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF9_W<'a> {
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
///Write proxy for field `CSOF10`
pub struct CSOF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF10_W<'a> {
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
///Write proxy for field `CSOF11`
pub struct CSOF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF11_W<'a> {
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
///Write proxy for field `CSOF12`
pub struct CSOF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF12_W<'a> {
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
///Write proxy for field `CSOF13`
pub struct CSOF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CSOF13_W<'a> {
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
impl W {
    ///Bit 0 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W {
        CSOF0_W { w: self }
    }
    ///Bit 1 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W {
        CSOF1_W { w: self }
    }
    ///Bit 2 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W {
        CSOF2_W { w: self }
    }
    ///Bit 3 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W {
        CSOF3_W { w: self }
    }
    ///Bit 4 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W {
        CSOF4_W { w: self }
    }
    ///Bit 5 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF5_W {
        CSOF5_W { w: self }
    }
    ///Bit 6 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF6_W {
        CSOF6_W { w: self }
    }
    ///Bit 7 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF7_W {
        CSOF7_W { w: self }
    }
    ///Bit 8 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF8_W {
        CSOF8_W { w: self }
    }
    ///Bit 9 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF9_W {
        CSOF9_W { w: self }
    }
    ///Bit 10 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF10_W {
        CSOF10_W { w: self }
    }
    ///Bit 11 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF11_W {
        CSOF11_W { w: self }
    }
    ///Bit 12 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof12(&mut self) -> CSOF12_W {
        CSOF12_W { w: self }
    }
    ///Bit 13 - Clear synchronization overrun event flag
    #[inline(always)]
    pub fn csof13(&mut self) -> CSOF13_W {
        CSOF13_W { w: self }
    }
}
