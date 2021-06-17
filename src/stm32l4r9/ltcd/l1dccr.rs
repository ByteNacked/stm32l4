///Reader of register L1DCCR
pub type R = crate::R<u32, super::L1DCCR>;
///Writer for register L1DCCR
pub type W = crate::W<u32, super::L1DCCR>;
///Register L1DCCR `reset()`'s with value 0
impl crate::ResetValue for super::L1DCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `DCBLUE`
pub type DCBLUE_R = crate::R<u8, u8>;
///Write proxy for field `DCBLUE`
pub struct DCBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCBLUE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
///Reader of field `DCGREEN`
pub type DCGREEN_R = crate::R<u8, u8>;
///Write proxy for field `DCGREEN`
pub struct DCGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCGREEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `DCRED`
pub type DCRED_R = crate::R<u8, u8>;
///Write proxy for field `DCRED`
pub struct DCRED_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRED_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
///Reader of field `DCALPHA`
pub type DCALPHA_R = crate::R<u8, u8>;
///Write proxy for field `DCALPHA`
pub struct DCALPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCALPHA_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Default Color Blue
    #[inline(always)]
    pub fn dcblue(&self) -> DCBLUE_R {
        DCBLUE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Default Color Green
    #[inline(always)]
    pub fn dcgreen(&self) -> DCGREEN_R {
        DCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Default Color Red
    #[inline(always)]
    pub fn dcred(&self) -> DCRED_R {
        DCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Default Color Alpha
    #[inline(always)]
    pub fn dcalpha(&self) -> DCALPHA_R {
        DCALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Default Color Blue
    #[inline(always)]
    pub fn dcblue(&mut self) -> DCBLUE_W {
        DCBLUE_W { w: self }
    }
    ///Bits 8:15 - Default Color Green
    #[inline(always)]
    pub fn dcgreen(&mut self) -> DCGREEN_W {
        DCGREEN_W { w: self }
    }
    ///Bits 16:23 - Default Color Red
    #[inline(always)]
    pub fn dcred(&mut self) -> DCRED_W {
        DCRED_W { w: self }
    }
    ///Bits 24:31 - Default Color Alpha
    #[inline(always)]
    pub fn dcalpha(&mut self) -> DCALPHA_W {
        DCALPHA_W { w: self }
    }
}
