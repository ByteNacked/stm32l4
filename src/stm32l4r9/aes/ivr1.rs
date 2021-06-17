///Reader of register IVR1
pub type R = crate::R<u32, super::IVR1>;
///Writer for register IVR1
pub type W = crate::W<u32, super::IVR1>;
///Register IVR1 `reset()`'s with value 0
impl crate::ResetValue for super::IVR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `AES_IVR1`
pub type AES_IVR1_R = crate::R<u32, u32>;
///Write proxy for field `AES_IVR1`
pub struct AES_IVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_IVR1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    pub fn aes_ivr1(&self) -> AES_IVR1_R {
        AES_IVR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    pub fn aes_ivr1(&mut self) -> AES_IVR1_W {
        AES_IVR1_W { w: self }
    }
}
