///Reader of register HPTXFSIZ
pub type R = crate::R<u32, super::HPTXFSIZ>;
///Writer for register HPTXFSIZ
pub type W = crate::W<u32, super::HPTXFSIZ>;
///Register HPTXFSIZ `reset()`'s with value 0x0200_0600
impl crate::ResetValue for super::HPTXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0600
    }
}
///Reader of field `PTXSA`
pub type PTXSA_R = crate::R<u16, u16>;
///Write proxy for field `PTXSA`
pub struct PTXSA_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `PTXFSIZ`
pub type PTXFSIZ_R = crate::R<u16, u16>;
///Write proxy for field `PTXFSIZ`
pub struct PTXFSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSIZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Host periodic TxFIFO start address
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Host periodic TxFIFO depth
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PTXFSIZ_R {
        PTXFSIZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Host periodic TxFIFO start address
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W {
        PTXSA_W { w: self }
    }
    ///Bits 16:31 - Host periodic TxFIFO depth
    #[inline(always)]
    pub fn ptxfsiz(&mut self) -> PTXFSIZ_W {
        PTXFSIZ_W { w: self }
    }
}
