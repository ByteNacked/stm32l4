///Reader of register FIFO
pub type R = crate::R<u32, super::FIFO>;
///Writer for register FIFO
pub type W = crate::W<u32, super::FIFO>;
///Register FIFO `reset()`'s with value 0
impl crate::ResetValue for super::FIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `FIFOData`
pub type FIFODATA_R = crate::R<u32, u32>;
///Write proxy for field `FIFOData`
pub struct FIFODATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODATA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Receive and transmit FIFO data
    #[inline(always)]
    pub fn fifodata(&mut self) -> FIFODATA_W {
        FIFODATA_W { w: self }
    }
}
