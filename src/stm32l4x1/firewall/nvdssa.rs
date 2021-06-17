///Reader of register NVDSSA
pub type R = crate::R<u32, super::NVDSSA>;
///Writer for register NVDSSA
pub type W = crate::W<u32, super::NVDSSA>;
///Register NVDSSA `reset()`'s with value 0
impl crate::ResetValue for super::NVDSSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `ADD`
pub type ADD_R = crate::R<u16, u16>;
///Write proxy for field `ADD`
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
impl R {
    ///Bits 8:23 - Non-volatile data segment start address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 8:23 - Non-volatile data segment start address
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
}
