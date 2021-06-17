///Reader of register BRR
pub type R = crate::R<u32, super::BRR>;
///Writer for register BRR
pub type W = crate::W<u32, super::BRR>;
///Register BRR `reset()`'s with value 0x01
impl crate::ResetValue for super::BRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
///Reader of field `BR`
pub type BR_R = crate::R<u8, u8>;
///Write proxy for field `BR`
pub struct BR_W<'a> {
    w: &'a mut W,
}
impl<'a> BR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    ///Bits 0:5 - Bitrate prescaler
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Bitrate prescaler
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W { w: self }
    }
}
