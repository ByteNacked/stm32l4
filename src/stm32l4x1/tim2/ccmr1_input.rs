///Reader of register CCMR1_Input
pub type R = crate::R<u32, super::CCMR1_INPUT>;
///Writer for register CCMR1_Input
pub type W = crate::W<u32, super::CCMR1_INPUT>;
///Register CCMR1_Input `reset()`'s with value 0
impl crate::ResetValue for super::CCMR1_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `IC2F`
pub type IC2F_R = crate::R<u8, u8>;
///Write proxy for field `IC2F`
pub struct IC2F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2F_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
///Reader of field `IC2PSC`
pub type IC2PSC_R = crate::R<u8, u8>;
///Write proxy for field `IC2PSC`
pub struct IC2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2PSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
///Capture/compare 2 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC2S_A {
    ///1: CC2 channel is configured as input, IC2 is mapped on TI2
    TI2 = 1,
    ///2: CC2 channel is configured as input, IC2 is mapped on TI1
    TI1 = 2,
    ///3: CC2 channel is configured as input, IC2 is mapped on TRC
    TRC = 3,
}
impl From<CC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC2S_A) -> Self {
        variant as _
    }
}
///Reader of field `CC2S`
pub type CC2S_R = crate::R<u8, CC2S_A>;
impl CC2S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC2S_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CC2S_A::TI2),
            2 => Val(CC2S_A::TI1),
            3 => Val(CC2S_A::TRC),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `TI2`
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC2S_A::TI2
    }
    ///Checks if the value of the field is `TI1`
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC2S_A::TI1
    }
    ///Checks if the value of the field is `TRC`
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC2S_A::TRC
    }
}
///Write proxy for field `CC2S`
pub struct CC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC2S_A::TI2)
    }
    ///CC2 channel is configured as input, IC2 is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC2S_A::TI1)
    }
    ///CC2 channel is configured as input, IC2 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC2S_A::TRC)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
///Input capture 1 filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC1F_A {
    ///0: No filter, sampling is done at fDTS
    NOFILTER = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FCK_INT_N2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FCK_INT_N4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FCK_INT_N8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FDTS_DIV2_N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FDTS_DIV2_N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FDTS_DIV4_N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FDTS_DIV4_N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FDTS_DIV8_N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FDTS_DIV8_N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FDTS_DIV16_N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FDTS_DIV16_N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FDTS_DIV16_N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FDTS_DIV32_N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FDTS_DIV32_N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FDTS_DIV32_N8 = 15,
}
impl From<IC1F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1F_A) -> Self {
        variant as _
    }
}
///Reader of field `IC1F`
pub type IC1F_R = crate::R<u8, IC1F_A>;
impl IC1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC1F_A {
        match self.bits {
            0 => IC1F_A::NOFILTER,
            1 => IC1F_A::FCK_INT_N2,
            2 => IC1F_A::FCK_INT_N4,
            3 => IC1F_A::FCK_INT_N8,
            4 => IC1F_A::FDTS_DIV2_N6,
            5 => IC1F_A::FDTS_DIV2_N8,
            6 => IC1F_A::FDTS_DIV4_N6,
            7 => IC1F_A::FDTS_DIV4_N8,
            8 => IC1F_A::FDTS_DIV8_N6,
            9 => IC1F_A::FDTS_DIV8_N8,
            10 => IC1F_A::FDTS_DIV16_N5,
            11 => IC1F_A::FDTS_DIV16_N6,
            12 => IC1F_A::FDTS_DIV16_N8,
            13 => IC1F_A::FDTS_DIV32_N5,
            14 => IC1F_A::FDTS_DIV32_N6,
            15 => IC1F_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOFILTER`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1F_A::NOFILTER
    }
    ///Checks if the value of the field is `FCK_INT_N2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC1F_A::FCK_INT_N2
    }
    ///Checks if the value of the field is `FCK_INT_N4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC1F_A::FCK_INT_N4
    }
    ///Checks if the value of the field is `FCK_INT_N8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC1F_A::FCK_INT_N8
    }
    ///Checks if the value of the field is `FDTS_DIV2_N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV2_N6
    }
    ///Checks if the value of the field is `FDTS_DIV2_N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV2_N8
    }
    ///Checks if the value of the field is `FDTS_DIV4_N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV4_N6
    }
    ///Checks if the value of the field is `FDTS_DIV4_N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV4_N8
    }
    ///Checks if the value of the field is `FDTS_DIV8_N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV8_N6
    }
    ///Checks if the value of the field is `FDTS_DIV8_N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV8_N8
    }
    ///Checks if the value of the field is `FDTS_DIV16_N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC1F_A::FDTS_DIV16_N5
    }
    ///Checks if the value of the field is `FDTS_DIV16_N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV16_N6
    }
    ///Checks if the value of the field is `FDTS_DIV16_N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV16_N8
    }
    ///Checks if the value of the field is `FDTS_DIV32_N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC1F_A::FDTS_DIV32_N5
    }
    ///Checks if the value of the field is `FDTS_DIV32_N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV32_N6
    }
    ///Checks if the value of the field is `FDTS_DIV32_N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV32_N8
    }
}
///Write proxy for field `IC1F`
pub struct IC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC1F_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC1F_A::NOFILTER)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV2_N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV2_N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV4_N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV4_N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV8_N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV8_N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Reader of field `IC1PSC`
pub type IC1PSC_R = crate::R<u8, u8>;
///Write proxy for field `IC1PSC`
pub struct IC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1PSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
///Capture/Compare 1 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    ///1: CC1 channel is configured as input, IC1 is mapped on TI1
    TI1 = 1,
    ///2: CC1 channel is configured as input, IC1 is mapped on TI2
    TI2 = 2,
    ///3: CC1 channel is configured as input, IC1 is mapped on TRC
    TRC = 3,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
///Reader of field `CC1S`
pub type CC1S_R = crate::R<u8, CC1S_A>;
impl CC1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC1S_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CC1S_A::TI1),
            2 => Val(CC1S_A::TI2),
            3 => Val(CC1S_A::TRC),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `TI1`
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC1S_A::TI1
    }
    ///Checks if the value of the field is `TI2`
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC1S_A::TI2
    }
    ///Checks if the value of the field is `TRC`
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC1S_A::TRC
    }
}
///Write proxy for field `CC1S`
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI1
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC1S_A::TI1)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI2
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC1S_A::TI2)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC1S_A::TRC)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    ///Bits 12:15 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 10:11 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 8:9 - Capture/compare 2 selection
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 12:15 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&mut self) -> IC2F_W {
        IC2F_W { w: self }
    }
    ///Bits 10:11 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&mut self) -> IC2PSC_W {
        IC2PSC_W { w: self }
    }
    ///Bits 8:9 - Capture/compare 2 selection
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W { w: self }
    }
    ///Bits 4:7 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W {
        IC1F_W { w: self }
    }
    ///Bits 2:3 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W {
        IC1PSC_W { w: self }
    }
    ///Bits 0:1 - Capture/Compare 1 selection
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
}
