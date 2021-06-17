///Reader of register TDTR
pub type R = crate::R<u32, super::TDTR>;
///Writer for register TDTR
pub type W = crate::W<u32, super::TDTR>;
///Register TDTR `reset()`'s with value 0
impl crate::ResetValue for super::TDTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TIME`
pub type TIME_R = crate::R<u16, u16>;
///Write proxy for field `TIME`
pub struct TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
///Reader of field `TGT`
pub type TGT_R = crate::R<bool, bool>;
///Write proxy for field `TGT`
pub struct TGT_W<'a> {
    w: &'a mut W,
}
impl<'a> TGT_W<'a> {
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
///Reader of field `DLC`
pub type DLC_R = crate::R<u8, u8>;
///Write proxy for field `DLC`
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 16:31 - TIME
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    ///Bit 8 - TGT
    #[inline(always)]
    pub fn tgt(&self) -> TGT_R {
        TGT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 0:3 - DLC
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 16:31 - TIME
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W {
        TIME_W { w: self }
    }
    ///Bit 8 - TGT
    #[inline(always)]
    pub fn tgt(&mut self) -> TGT_W {
        TGT_W { w: self }
    }
    ///Bits 0:3 - DLC
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
}
