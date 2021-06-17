///Reader of register FS1R
pub type R = crate::R<u32, super::FS1R>;
///Writer for register FS1R
pub type W = crate::W<u32, super::FS1R>;
///Register FS1R `reset()`'s with value 0
impl crate::ResetValue for super::FS1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FSC0`
pub type FSC0_R = crate::R<bool, bool>;
///Write proxy for field `FSC0`
pub struct FSC0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC0_W<'a> {
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
///Reader of field `FSC1`
pub type FSC1_R = crate::R<bool, bool>;
///Write proxy for field `FSC1`
pub struct FSC1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC1_W<'a> {
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
///Reader of field `FSC2`
pub type FSC2_R = crate::R<bool, bool>;
///Write proxy for field `FSC2`
pub struct FSC2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC2_W<'a> {
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
///Reader of field `FSC3`
pub type FSC3_R = crate::R<bool, bool>;
///Write proxy for field `FSC3`
pub struct FSC3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC3_W<'a> {
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
///Reader of field `FSC4`
pub type FSC4_R = crate::R<bool, bool>;
///Write proxy for field `FSC4`
pub struct FSC4_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC4_W<'a> {
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
///Reader of field `FSC5`
pub type FSC5_R = crate::R<bool, bool>;
///Write proxy for field `FSC5`
pub struct FSC5_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC5_W<'a> {
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
///Reader of field `FSC6`
pub type FSC6_R = crate::R<bool, bool>;
///Write proxy for field `FSC6`
pub struct FSC6_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC6_W<'a> {
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
///Reader of field `FSC7`
pub type FSC7_R = crate::R<bool, bool>;
///Write proxy for field `FSC7`
pub struct FSC7_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC7_W<'a> {
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
///Reader of field `FSC8`
pub type FSC8_R = crate::R<bool, bool>;
///Write proxy for field `FSC8`
pub struct FSC8_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC8_W<'a> {
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
///Reader of field `FSC9`
pub type FSC9_R = crate::R<bool, bool>;
///Write proxy for field `FSC9`
pub struct FSC9_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC9_W<'a> {
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
///Reader of field `FSC10`
pub type FSC10_R = crate::R<bool, bool>;
///Write proxy for field `FSC10`
pub struct FSC10_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC10_W<'a> {
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
///Reader of field `FSC11`
pub type FSC11_R = crate::R<bool, bool>;
///Write proxy for field `FSC11`
pub struct FSC11_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC11_W<'a> {
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
///Reader of field `FSC12`
pub type FSC12_R = crate::R<bool, bool>;
///Write proxy for field `FSC12`
pub struct FSC12_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC12_W<'a> {
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
///Reader of field `FSC13`
pub type FSC13_R = crate::R<bool, bool>;
///Write proxy for field `FSC13`
pub struct FSC13_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC13_W<'a> {
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
impl R {
    ///Bit 0 - Filter scale configuration
    #[inline(always)]
    pub fn fsc0(&self) -> FSC0_R {
        FSC0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Filter scale configuration
    #[inline(always)]
    pub fn fsc1(&self) -> FSC1_R {
        FSC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Filter scale configuration
    #[inline(always)]
    pub fn fsc2(&self) -> FSC2_R {
        FSC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Filter scale configuration
    #[inline(always)]
    pub fn fsc3(&self) -> FSC3_R {
        FSC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Filter scale configuration
    #[inline(always)]
    pub fn fsc4(&self) -> FSC4_R {
        FSC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Filter scale configuration
    #[inline(always)]
    pub fn fsc5(&self) -> FSC5_R {
        FSC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Filter scale configuration
    #[inline(always)]
    pub fn fsc6(&self) -> FSC6_R {
        FSC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Filter scale configuration
    #[inline(always)]
    pub fn fsc7(&self) -> FSC7_R {
        FSC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Filter scale configuration
    #[inline(always)]
    pub fn fsc8(&self) -> FSC8_R {
        FSC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Filter scale configuration
    #[inline(always)]
    pub fn fsc9(&self) -> FSC9_R {
        FSC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Filter scale configuration
    #[inline(always)]
    pub fn fsc10(&self) -> FSC10_R {
        FSC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Filter scale configuration
    #[inline(always)]
    pub fn fsc11(&self) -> FSC11_R {
        FSC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Filter scale configuration
    #[inline(always)]
    pub fn fsc12(&self) -> FSC12_R {
        FSC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Filter scale configuration
    #[inline(always)]
    pub fn fsc13(&self) -> FSC13_R {
        FSC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Filter scale configuration
    #[inline(always)]
    pub fn fsc0(&mut self) -> FSC0_W {
        FSC0_W { w: self }
    }
    ///Bit 1 - Filter scale configuration
    #[inline(always)]
    pub fn fsc1(&mut self) -> FSC1_W {
        FSC1_W { w: self }
    }
    ///Bit 2 - Filter scale configuration
    #[inline(always)]
    pub fn fsc2(&mut self) -> FSC2_W {
        FSC2_W { w: self }
    }
    ///Bit 3 - Filter scale configuration
    #[inline(always)]
    pub fn fsc3(&mut self) -> FSC3_W {
        FSC3_W { w: self }
    }
    ///Bit 4 - Filter scale configuration
    #[inline(always)]
    pub fn fsc4(&mut self) -> FSC4_W {
        FSC4_W { w: self }
    }
    ///Bit 5 - Filter scale configuration
    #[inline(always)]
    pub fn fsc5(&mut self) -> FSC5_W {
        FSC5_W { w: self }
    }
    ///Bit 6 - Filter scale configuration
    #[inline(always)]
    pub fn fsc6(&mut self) -> FSC6_W {
        FSC6_W { w: self }
    }
    ///Bit 7 - Filter scale configuration
    #[inline(always)]
    pub fn fsc7(&mut self) -> FSC7_W {
        FSC7_W { w: self }
    }
    ///Bit 8 - Filter scale configuration
    #[inline(always)]
    pub fn fsc8(&mut self) -> FSC8_W {
        FSC8_W { w: self }
    }
    ///Bit 9 - Filter scale configuration
    #[inline(always)]
    pub fn fsc9(&mut self) -> FSC9_W {
        FSC9_W { w: self }
    }
    ///Bit 10 - Filter scale configuration
    #[inline(always)]
    pub fn fsc10(&mut self) -> FSC10_W {
        FSC10_W { w: self }
    }
    ///Bit 11 - Filter scale configuration
    #[inline(always)]
    pub fn fsc11(&mut self) -> FSC11_W {
        FSC11_W { w: self }
    }
    ///Bit 12 - Filter scale configuration
    #[inline(always)]
    pub fn fsc12(&mut self) -> FSC12_W {
        FSC12_W { w: self }
    }
    ///Bit 13 - Filter scale configuration
    #[inline(always)]
    pub fn fsc13(&mut self) -> FSC13_W {
        FSC13_W { w: self }
    }
}
