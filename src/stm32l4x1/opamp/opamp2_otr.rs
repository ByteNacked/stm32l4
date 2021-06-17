///Reader of register OPAMP2_OTR
pub type R = crate::R<u32, super::OPAMP2_OTR>;
///Writer for register OPAMP2_OTR
pub type W = crate::W<u32, super::OPAMP2_OTR>;
///Register OPAMP2_OTR `reset()`'s with value 0
impl crate::ResetValue for super::OPAMP2_OTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `TRIMOFFSETN`
pub type TRIMOFFSETN_R = crate::R<u8, u8>;
///Write proxy for field `TRIMOFFSETN`
pub struct TRIMOFFSETN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
///Reader of field `TRIMOFFSETP`
pub type TRIMOFFSETP_R = crate::R<u8, u8>;
///Write proxy for field `TRIMOFFSETP`
pub struct TRIMOFFSETP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMOFFSETP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W {
        TRIMOFFSETN_W { w: self }
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W {
        TRIMOFFSETP_W { w: self }
    }
}
