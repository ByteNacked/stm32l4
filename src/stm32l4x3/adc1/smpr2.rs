///Reader of register SMPR2
pub type R = crate::R<u32, super::SMPR2>;
///Writer for register SMPR2
pub type W = crate::W<u32, super::SMPR2>;
///Register SMPR2 `reset()`'s with value 0
impl crate::ResetValue for super::SMPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `SMP18`
pub type SMP18_R = crate::R<u8, u8>;
///Write proxy for field `SMP18`
pub struct SMP18_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP18_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
///Reader of field `SMP17`
pub type SMP17_R = crate::R<u8, u8>;
///Write proxy for field `SMP17`
pub struct SMP17_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP17_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
///Reader of field `SMP16`
pub type SMP16_R = crate::R<u8, u8>;
///Write proxy for field `SMP16`
pub struct SMP16_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP16_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
///Reader of field `SMP15`
pub type SMP15_R = crate::R<u8, u8>;
///Write proxy for field `SMP15`
pub struct SMP15_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP15_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
///Reader of field `SMP14`
pub type SMP14_R = crate::R<u8, u8>;
///Write proxy for field `SMP14`
pub struct SMP14_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP14_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
///Reader of field `SMP13`
pub type SMP13_R = crate::R<u8, u8>;
///Write proxy for field `SMP13`
pub struct SMP13_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP13_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
///Reader of field `SMP12`
pub type SMP12_R = crate::R<u8, u8>;
///Write proxy for field `SMP12`
pub struct SMP12_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP12_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
///Reader of field `SMP11`
pub type SMP11_R = crate::R<u8, u8>;
///Write proxy for field `SMP11`
pub struct SMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP11_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
///Reader of field `SMP10`
pub type SMP10_R = crate::R<u8, u8>;
///Write proxy for field `SMP10`
pub struct SMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP10_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    ///Bits 24:26 - Channel 18 sampling time selection
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    ///Bits 21:23 - Channel 17 sampling time selection
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    ///Bits 18:20 - Channel 16 sampling time selection
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    ///Bits 15:17 - Channel 15 sampling time selection
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    ///Bits 12:14 - Channel 14 sampling time selection
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    ///Bits 9:11 - Channel 13 sampling time selection
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    ///Bits 6:8 - Channel 12 sampling time selection
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    ///Bits 3:5 - Channel 11 sampling time selection
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    ///Bits 0:2 - Channel 10 sampling time selection
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 24:26 - Channel 18 sampling time selection
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W {
        SMP18_W { w: self }
    }
    ///Bits 21:23 - Channel 17 sampling time selection
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W {
        SMP17_W { w: self }
    }
    ///Bits 18:20 - Channel 16 sampling time selection
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W {
        SMP16_W { w: self }
    }
    ///Bits 15:17 - Channel 15 sampling time selection
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W {
        SMP15_W { w: self }
    }
    ///Bits 12:14 - Channel 14 sampling time selection
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W {
        SMP14_W { w: self }
    }
    ///Bits 9:11 - Channel 13 sampling time selection
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W {
        SMP13_W { w: self }
    }
    ///Bits 6:8 - Channel 12 sampling time selection
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W {
        SMP12_W { w: self }
    }
    ///Bits 3:5 - Channel 11 sampling time selection
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W {
        SMP11_W { w: self }
    }
    ///Bits 0:2 - Channel 10 sampling time selection
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W {
        SMP10_W { w: self }
    }
}
