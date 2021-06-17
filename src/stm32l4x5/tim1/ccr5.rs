///Reader of register CCR5
pub type R = crate::R<u32, super::CCR5>;
///Writer for register CCR5
pub type W = crate::W<u32, super::CCR5>;
///Register CCR5 `reset()`'s with value 0
impl crate::ResetValue for super::CCR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `CCR`
pub type CCR_R = crate::R<u16, u16>;
///Write proxy for field `CCR`
pub struct CCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
///Reader of field `GC5C1`
pub type GC5C1_R = crate::R<bool, bool>;
///Write proxy for field `GC5C1`
pub struct GC5C1_W<'a> {
    w: &'a mut W,
}
impl<'a> GC5C1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
///Reader of field `GC5C2`
pub type GC5C2_R = crate::R<bool, bool>;
///Write proxy for field `GC5C2`
pub struct GC5C2_W<'a> {
    w: &'a mut W,
}
impl<'a> GC5C2_W<'a> {
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
///Reader of field `GC5C3`
pub type GC5C3_R = crate::R<bool, bool>;
///Write proxy for field `GC5C3`
pub struct GC5C3_W<'a> {
    w: &'a mut W,
}
impl<'a> GC5C3_W<'a> {
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
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 29 - Group Channel 5 and Channel 1
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - Group Channel 5 and Channel 2
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Group Channel 5 and Channel 3
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W { w: self }
    }
    ///Bit 29 - Group Channel 5 and Channel 1
    #[inline(always)]
    pub fn gc5c1(&mut self) -> GC5C1_W {
        GC5C1_W { w: self }
    }
    ///Bit 30 - Group Channel 5 and Channel 2
    #[inline(always)]
    pub fn gc5c2(&mut self) -> GC5C2_W {
        GC5C2_W { w: self }
    }
    ///Bit 31 - Group Channel 5 and Channel 3
    #[inline(always)]
    pub fn gc5c3(&mut self) -> GC5C3_W {
        GC5C3_W { w: self }
    }
}
