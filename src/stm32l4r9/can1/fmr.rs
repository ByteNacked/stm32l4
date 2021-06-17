///Reader of register FMR
pub type R = crate::R<u32, super::FMR>;
///Writer for register FMR
pub type W = crate::W<u32, super::FMR>;
///Register FMR `reset()`'s with value 0x2a1c_0e01
impl crate::ResetValue for super::FMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2a1c_0e01
    }
}
///Reader of field `FINIT`
pub type FINIT_R = crate::R<bool, bool>;
///Write proxy for field `FINIT`
pub struct FINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FINIT_W<'a> {
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
///Reader of field `CANSB`
pub type CANSB_R = crate::R<u8, u8>;
///Write proxy for field `CANSB`
pub struct CANSB_W<'a> {
    w: &'a mut W,
}
impl<'a> CANSB_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    ///Bit 0 - Filter initialization mode
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 8:13 - CAN start bank
    #[inline(always)]
    pub fn cansb(&self) -> CANSB_R {
        CANSB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 0 - Filter initialization mode
    #[inline(always)]
    pub fn finit(&mut self) -> FINIT_W {
        FINIT_W { w: self }
    }
    ///Bits 8:13 - CAN start bank
    #[inline(always)]
    pub fn cansb(&mut self) -> CANSB_W {
        CANSB_W { w: self }
    }
}
