///Reader of register BTR%s
pub type R = crate::R<u32, super::BTR>;
///Writer for register BTR%s
pub type W = crate::W<u32, super::BTR>;
///Register BTR%s `reset()`'s with value 0xffff_ffff
impl crate::ResetValue for super::BTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
///ACCMOD
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACCMOD_A {
    ///0: Access mode A
    A = 0,
    ///1: Access mode B
    B = 1,
    ///2: Access mode C
    C = 2,
    ///3: Access mode D
    D = 3,
}
impl From<ACCMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: ACCMOD_A) -> Self {
        variant as _
    }
}
///Reader of field `ACCMOD`
pub type ACCMOD_R = crate::R<u8, ACCMOD_A>;
impl ACCMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ACCMOD_A {
        match self.bits {
            0 => ACCMOD_A::A,
            1 => ACCMOD_A::B,
            2 => ACCMOD_A::C,
            3 => ACCMOD_A::D,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `A`
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == ACCMOD_A::A
    }
    ///Checks if the value of the field is `B`
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == ACCMOD_A::B
    }
    ///Checks if the value of the field is `C`
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == ACCMOD_A::C
    }
    ///Checks if the value of the field is `D`
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == ACCMOD_A::D
    }
}
///Write proxy for field `ACCMOD`
pub struct ACCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCMOD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ACCMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Access mode A
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(ACCMOD_A::A)
    }
    ///Access mode B
    #[inline(always)]
    pub fn b(self) -> &'a mut W {
        self.variant(ACCMOD_A::B)
    }
    ///Access mode C
    #[inline(always)]
    pub fn c(self) -> &'a mut W {
        self.variant(ACCMOD_A::C)
    }
    ///Access mode D
    #[inline(always)]
    pub fn d(self) -> &'a mut W {
        self.variant(ACCMOD_A::D)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///Reader of field `DATLAT`
pub type DATLAT_R = crate::R<u8, u8>;
///Write proxy for field `DATLAT`
pub struct DATLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLAT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
///Reader of field `CLKDIV`
pub type CLKDIV_R = crate::R<u8, u8>;
///Write proxy for field `CLKDIV`
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
///Reader of field `BUSTURN`
pub type BUSTURN_R = crate::R<u8, u8>;
///Write proxy for field `BUSTURN`
pub struct BUSTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSTURN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
///Reader of field `DATAST`
pub type DATAST_R = crate::R<u8, u8>;
///Write proxy for field `DATAST`
pub struct DATAST_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAST_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
///Reader of field `ADDHLD`
pub type ADDHLD_R = crate::R<u8, u8>;
///Write proxy for field `ADDHLD`
pub struct ADDHLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDHLD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Reader of field `ADDSET`
pub type ADDSET_R = crate::R<u8, u8>;
///Write proxy for field `ADDSET`
pub struct ADDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 24:27 - DATLAT
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 20:23 - CLKDIV
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&mut self) -> ACCMOD_W {
        ACCMOD_W { w: self }
    }
    ///Bits 24:27 - DATLAT
    #[inline(always)]
    pub fn datlat(&mut self) -> DATLAT_W {
        DATLAT_W { w: self }
    }
    ///Bits 20:23 - CLKDIV
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&mut self) -> BUSTURN_W {
        BUSTURN_W { w: self }
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&mut self) -> DATAST_W {
        DATAST_W { w: self }
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&mut self) -> ADDHLD_W {
        ADDHLD_W { w: self }
    }
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&mut self) -> ADDSET_W {
        ADDSET_W { w: self }
    }
}
