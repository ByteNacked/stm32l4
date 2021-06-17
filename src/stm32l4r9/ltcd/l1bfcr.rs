///Reader of register L1BFCR
pub type R = crate::R<u32, super::L1BFCR>;
///Writer for register L1BFCR
pub type W = crate::W<u32, super::L1BFCR>;
///Register L1BFCR `reset()`'s with value 0
impl crate::ResetValue for super::L1BFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `BF2`
pub type BF2_R = crate::R<u8, u8>;
///Write proxy for field `BF2`
pub struct BF2_W<'a> {
    w: &'a mut W,
}
impl<'a> BF2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
///Reader of field `BF1`
pub type BF1_R = crate::R<u8, u8>;
///Write proxy for field `BF1`
pub struct BF1_W<'a> {
    w: &'a mut W,
}
impl<'a> BF1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Blending Factor 2
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 8:10 - Blending Factor 1
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Blending Factor 2
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W {
        BF2_W { w: self }
    }
    ///Bits 8:10 - Blending Factor 1
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W {
        BF1_W { w: self }
    }
}
