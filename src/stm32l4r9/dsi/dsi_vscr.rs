///Reader of register DSI_VSCR
pub type R = crate::R<u32, super::DSI_VSCR>;
///Writer for register DSI_VSCR
pub type W = crate::W<u32, super::DSI_VSCR>;
///Register DSI_VSCR `reset()`'s with value 0
impl crate::ResetValue for super::DSI_VSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `EN`
pub type EN_R = crate::R<bool, bool>;
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
///Reader of field `UR`
pub type UR_R = crate::R<bool, bool>;
///Write proxy for field `UR`
pub struct UR_W<'a> {
    w: &'a mut W,
}
impl<'a> UR_W<'a> {
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
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 8 - Update Register
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    ///Bit 8 - Update Register
    #[inline(always)]
    pub fn ur(&mut self) -> UR_W {
        UR_W { w: self }
    }
}
