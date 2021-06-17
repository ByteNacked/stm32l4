///Reader of register BTABLE
pub type R = crate::R<u32, super::BTABLE>;
///Writer for register BTABLE
pub type W = crate::W<u32, super::BTABLE>;
///Register BTABLE `reset()`'s with value 0
impl crate::ResetValue for super::BTABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `BTABLE`
pub type BTABLE_R = crate::R<u16, u16>;
///Write proxy for field `BTABLE`
pub struct BTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTABLE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | (((value as u32) & 0x1fff) << 3);
        self.w
    }
}
impl R {
    ///Bits 3:15 - Buffer table
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 3:15 - Buffer table
    #[inline(always)]
    pub fn btable(&mut self) -> BTABLE_W {
        BTABLE_W { w: self }
    }
}
