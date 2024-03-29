///Reader of register FS_DIEPTXF2
pub type R = crate::R<u32, super::FS_DIEPTXF2>;
///Writer for register FS_DIEPTXF2
pub type W = crate::W<u32, super::FS_DIEPTXF2>;
///Register FS_DIEPTXF2 `reset()`'s with value 0x0200_0400
impl crate::ResetValue for super::FS_DIEPTXF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0400
    }
}
///Reader of field `INEPTXSA`
pub type INEPTXSA_R = crate::R<u16, u16>;
///Write proxy for field `INEPTXSA`
pub struct INEPTXSA_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXSA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `INEPTXFD`
pub type INEPTXFD_R = crate::R<u16, u16>;
///Write proxy for field `INEPTXFD`
pub struct INEPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXFD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - IN endpoint FIFO3 transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - IN endpoint FIFO3 transmit RAM start address
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W {
        INEPTXSA_W { w: self }
    }
    ///Bits 16:31 - IN endpoint TxFIFO depth
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W {
        INEPTXFD_W { w: self }
    }
}
