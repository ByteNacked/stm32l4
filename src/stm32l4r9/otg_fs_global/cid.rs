///Reader of register CID
pub type R = crate::R<u32, super::CID>;
///Writer for register CID
pub type W = crate::W<u32, super::CID>;
///Register CID `reset()`'s with value 0x1000
impl crate::ResetValue for super::CID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
///Reader of field `PRODUCT_ID`
pub type PRODUCT_ID_R = crate::R<u32, u32>;
///Write proxy for field `PRODUCT_ID`
pub struct PRODUCT_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> PRODUCT_ID_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Product ID field
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Product ID field
    #[inline(always)]
    pub fn product_id(&mut self) -> PRODUCT_ID_W {
        PRODUCT_ID_W { w: self }
    }
}
