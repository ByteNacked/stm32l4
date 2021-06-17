///Reader of register FPCCR
pub type R = crate::R<u32, super::FPCCR>;
///Writer for register FPCCR
pub type W = crate::W<u32, super::FPCCR>;
///Register FPCCR `reset()`'s with value 0
impl crate::ResetValue for super::FPCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `LSPACT`
pub type LSPACT_R = crate::R<bool, bool>;
///Write proxy for field `LSPACT`
pub struct LSPACT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPACT_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Reader of field `USER`
pub type USER_R = crate::R<bool, bool>;
///Write proxy for field `USER`
pub struct USER_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `THREAD`
pub type THREAD_R = crate::R<bool, bool>;
///Write proxy for field `THREAD`
pub struct THREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> THREAD_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `HFRDY`
pub type HFRDY_R = crate::R<bool, bool>;
///Write proxy for field `HFRDY`
pub struct HFRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRDY_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///Reader of field `MMRDY`
pub type MMRDY_R = crate::R<bool, bool>;
///Write proxy for field `MMRDY`
pub struct MMRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MMRDY_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///Reader of field `BFRDY`
pub type BFRDY_R = crate::R<bool, bool>;
///Write proxy for field `BFRDY`
pub struct BFRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BFRDY_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Reader of field `MONRDY`
pub type MONRDY_R = crate::R<bool, bool>;
///Write proxy for field `MONRDY`
pub struct MONRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MONRDY_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
///Reader of field `LSPEN`
pub type LSPEN_R = crate::R<bool, bool>;
///Write proxy for field `LSPEN`
pub struct LSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPEN_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
///Reader of field `ASPEN`
pub type ASPEN_R = crate::R<bool, bool>;
///Write proxy for field `ASPEN`
pub struct ASPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASPEN_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - LSPACT
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - USER
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 3 - THREAD
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - HFRDY
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - MMRDY
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - BFRDY
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 8 - MONRDY
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 30 - LSPEN
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - ASPEN
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - LSPACT
    #[inline(always)]
    pub fn lspact(&mut self) -> LSPACT_W {
        LSPACT_W { w: self }
    }
    ///Bit 1 - USER
    #[inline(always)]
    pub fn user(&mut self) -> USER_W {
        USER_W { w: self }
    }
    ///Bit 3 - THREAD
    #[inline(always)]
    pub fn thread(&mut self) -> THREAD_W {
        THREAD_W { w: self }
    }
    ///Bit 4 - HFRDY
    #[inline(always)]
    pub fn hfrdy(&mut self) -> HFRDY_W {
        HFRDY_W { w: self }
    }
    ///Bit 5 - MMRDY
    #[inline(always)]
    pub fn mmrdy(&mut self) -> MMRDY_W {
        MMRDY_W { w: self }
    }
    ///Bit 6 - BFRDY
    #[inline(always)]
    pub fn bfrdy(&mut self) -> BFRDY_W {
        BFRDY_W { w: self }
    }
    ///Bit 8 - MONRDY
    #[inline(always)]
    pub fn monrdy(&mut self) -> MONRDY_W {
        MONRDY_W { w: self }
    }
    ///Bit 30 - LSPEN
    #[inline(always)]
    pub fn lspen(&mut self) -> LSPEN_W {
        LSPEN_W { w: self }
    }
    ///Bit 31 - ASPEN
    #[inline(always)]
    pub fn aspen(&mut self) -> ASPEN_W {
        ASPEN_W { w: self }
    }
}
