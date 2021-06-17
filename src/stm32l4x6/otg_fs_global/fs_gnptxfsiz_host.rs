///Reader of register FS_GNPTXFSIZ_Host
pub type R = crate::R<u32, super::FS_GNPTXFSIZ_HOST>;
///Writer for register FS_GNPTXFSIZ_Host
pub type W = crate::W<u32, super::FS_GNPTXFSIZ_HOST>;
///Register FS_GNPTXFSIZ_Host `reset()`'s with value 0x0200
impl crate::ResetValue for super::FS_GNPTXFSIZ_HOST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
///Reader of field `NPTXFSA`
pub type NPTXFSA_R = crate::R<u16, u16>;
///Write proxy for field `NPTXFSA`
pub struct NPTXFSA_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `NPTXFD`
pub type NPTXFD_R = crate::R<u16, u16>;
///Write proxy for field `NPTXFD`
pub struct NPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Non-periodic transmit RAM start address
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Non-periodic TxFIFO depth
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Non-periodic transmit RAM start address
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W {
        NPTXFSA_W { w: self }
    }
    ///Bits 16:31 - Non-periodic TxFIFO depth
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NPTXFD_W {
        NPTXFD_W { w: self }
    }
}
