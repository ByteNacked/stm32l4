///Reader of register B3CR
pub type R = crate::R<u32, super::B3CR>;
///Writer for register B3CR
pub type W = crate::W<u32, super::B3CR>;
///Register B3CR `reset()`'s with value 0
impl crate::ResetValue for super::B3CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PBO`
pub type PBO_R = crate::R<u32, u32>;
///Write proxy for field `PBO`
pub struct PBO_W<'a> {
    w: &'a mut W,
}
impl<'a> PBO_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 4)) | (((value as u32) & 0x0007_ffff) << 4);
        self.w
    }
}
///Reader of field `PBBA`
pub type PBBA_R = crate::R<u16, u16>;
///Write proxy for field `PBBA`
pub struct PBBA_W<'a> {
    w: &'a mut W,
}
impl<'a> PBBA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
impl R {
    ///Bits 4:22 - Physical buffer offset
    #[inline(always)]
    pub fn pbo(&self) -> PBO_R {
        PBO_R::new(((self.bits >> 4) & 0x0007_ffff) as u32)
    }
    ///Bits 23:31 - Physical buffer base address
    #[inline(always)]
    pub fn pbba(&self) -> PBBA_R {
        PBBA_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 4:22 - Physical buffer offset
    #[inline(always)]
    pub fn pbo(&mut self) -> PBO_W {
        PBO_W { w: self }
    }
    ///Bits 23:31 - Physical buffer base address
    #[inline(always)]
    pub fn pbba(&mut self) -> PBBA_W {
        PBBA_W { w: self }
    }
}
