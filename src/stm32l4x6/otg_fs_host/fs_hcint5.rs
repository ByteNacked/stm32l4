///Reader of register FS_HCINT5
pub type R = crate::R<u32, super::FS_HCINT5>;
///Writer for register FS_HCINT5
pub type W = crate::W<u32, super::FS_HCINT5>;
///Register FS_HCINT5 `reset()`'s with value 0
impl crate::ResetValue for super::FS_HCINT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `XFRC`
pub type XFRC_R = crate::R<bool, bool>;
///Write proxy for field `XFRC`
pub struct XFRC_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRC_W<'a> {
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
///Reader of field `CHH`
pub type CHH_R = crate::R<bool, bool>;
///Write proxy for field `CHH`
pub struct CHH_W<'a> {
    w: &'a mut W,
}
impl<'a> CHH_W<'a> {
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
///Reader of field `STALL`
pub type STALL_R = crate::R<bool, bool>;
///Write proxy for field `STALL`
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
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
///Reader of field `NAK`
pub type NAK_R = crate::R<bool, bool>;
///Write proxy for field `NAK`
pub struct NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NAK_W<'a> {
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
///Reader of field `ACK`
pub type ACK_R = crate::R<bool, bool>;
///Write proxy for field `ACK`
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
///Reader of field `TXERR`
pub type TXERR_R = crate::R<bool, bool>;
///Write proxy for field `TXERR`
pub struct TXERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERR_W<'a> {
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
///Reader of field `BBERR`
pub type BBERR_R = crate::R<bool, bool>;
///Write proxy for field `BBERR`
pub struct BBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BBERR_W<'a> {
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
///Reader of field `FRMOR`
pub type FRMOR_R = crate::R<bool, bool>;
///Write proxy for field `FRMOR`
pub struct FRMOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMOR_W<'a> {
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
///Reader of field `DTERR`
pub type DTERR_R = crate::R<bool, bool>;
///Write proxy for field `DTERR`
pub struct DTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTERR_W<'a> {
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
impl R {
    ///Bit 0 - Transfer completed
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Channel halted
    #[inline(always)]
    pub fn chh(&self) -> CHH_R {
        CHH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - STALL response received interrupt
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - NAK response received interrupt
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - ACK response received/transmitted interrupt
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 7 - Transaction error
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Babble error
    #[inline(always)]
    pub fn bberr(&self) -> BBERR_R {
        BBERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Frame overrun
    #[inline(always)]
    pub fn frmor(&self) -> FRMOR_R {
        FRMOR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Data toggle error
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Transfer completed
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W {
        XFRC_W { w: self }
    }
    ///Bit 1 - Channel halted
    #[inline(always)]
    pub fn chh(&mut self) -> CHH_W {
        CHH_W { w: self }
    }
    ///Bit 3 - STALL response received interrupt
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
    ///Bit 4 - NAK response received interrupt
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W {
        NAK_W { w: self }
    }
    ///Bit 5 - ACK response received/transmitted interrupt
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    ///Bit 7 - Transaction error
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W {
        TXERR_W { w: self }
    }
    ///Bit 8 - Babble error
    #[inline(always)]
    pub fn bberr(&mut self) -> BBERR_W {
        BBERR_W { w: self }
    }
    ///Bit 9 - Frame overrun
    #[inline(always)]
    pub fn frmor(&mut self) -> FRMOR_W {
        FRMOR_W { w: self }
    }
    ///Bit 10 - Data toggle error
    #[inline(always)]
    pub fn dterr(&mut self) -> DTERR_W {
        DTERR_W { w: self }
    }
}
