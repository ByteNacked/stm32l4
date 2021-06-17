///Reader of register IR
pub type R = crate::R<u32, super::IR>;
///Writer for register IR
pub type W = crate::W<u32, super::IR>;
///Register IR `reset()`'s with value 0
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `INSTRUCTION`
pub type INSTRUCTION_R = crate::R<u32, u32>;
///Write proxy for field `INSTRUCTION`
pub struct INSTRUCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRUCTION_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - INSTRUCTION
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - INSTRUCTION
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W {
        INSTRUCTION_W { w: self }
    }
}
