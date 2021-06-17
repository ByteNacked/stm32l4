///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TEF`
pub type TEF_R = crate::R<bool, bool>;
///Write proxy for field `TEF`
pub struct TEF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEF_W<'a> {
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
///Reader of field `TCF`
pub type TCF_R = crate::R<bool, bool>;
///Write proxy for field `TCF`
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
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
///Reader of field `FTF`
pub type FTF_R = crate::R<bool, bool>;
///Write proxy for field `FTF`
pub struct FTF_W<'a> {
    w: &'a mut W,
}
impl<'a> FTF_W<'a> {
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
///Reader of field `SMF`
pub type SMF_R = crate::R<bool, bool>;
///Write proxy for field `SMF`
pub struct SMF_W<'a> {
    w: &'a mut W,
}
impl<'a> SMF_W<'a> {
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
///Reader of field `TOF`
pub type TOF_R = crate::R<bool, bool>;
///Write proxy for field `TOF`
pub struct TOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOF_W<'a> {
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
///Reader of field `BUSY`
pub type BUSY_R = crate::R<bool, bool>;
///Write proxy for field `BUSY`
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
///Reader of field `FLEVEL`
pub type FLEVEL_R = crate::R<u8, u8>;
///Write proxy for field `FLEVEL`
pub struct FLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEVEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bits 8:13 - FIFO level
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&mut self) -> TEF_W {
        TEF_W { w: self }
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W {
        FTF_W { w: self }
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    pub fn smf(&mut self) -> SMF_W {
        SMF_W { w: self }
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W {
        TOF_W { w: self }
    }
    ///Bit 5 - BUSY
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    ///Bits 8:13 - FIFO level
    #[inline(always)]
    pub fn flevel(&mut self) -> FLEVEL_W {
        FLEVEL_W { w: self }
    }
}
