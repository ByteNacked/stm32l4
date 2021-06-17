///Reader of register AHB3ENR
pub type R = crate::R<u32, super::AHB3ENR>;
///Writer for register AHB3ENR
pub type W = crate::W<u32, super::AHB3ENR>;
///Register AHB3ENR `reset()`'s with value 0
impl crate::ResetValue for super::AHB3ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `QSPIEN`
pub type QSPIEN_R = crate::R<bool, bool>;
///Write proxy for field `QSPIEN`
pub struct QSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIEN_W<'a> {
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
    ///Bit 8 - QSPIEN
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    ///Bit 8 - QSPIEN
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W {
        QSPIEN_W { w: self }
    }
}
