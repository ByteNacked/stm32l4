///Reader of register DFSDM_FLT1CHGR
pub type R = crate::R<u32, super::DFSDM_FLT1CHGR>;
///Writer for register DFSDM_FLT1CHGR
pub type W = crate::W<u32, super::DFSDM_FLT1CHGR>;
///Register DFSDM_FLT1CHGR `reset()`'s with value 0x01
impl crate::ResetValue for super::DFSDM_FLT1CHGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
///Reader of field `JCHG`
pub type JCHG_R = crate::R<u8, u8>;
///Write proxy for field `JCHG`
pub struct JCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> JCHG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Injected channel group selection
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Injected channel group selection
    #[inline(always)]
    pub fn jchg(&mut self) -> JCHG_W {
        JCHG_W { w: self }
    }
}
