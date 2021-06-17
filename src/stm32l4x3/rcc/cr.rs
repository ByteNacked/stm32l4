///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0x63
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x63
    }
}
///Reader of field `PLLSAI1RDY`
pub type PLLSAI1RDY_R = crate::R<bool, bool>;
///Reader of field `PLLSAI1ON`
pub type PLLSAI1ON_R = crate::R<bool, bool>;
///Write proxy for field `PLLSAI1ON`
pub struct PLLSAI1ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI1ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
///Reader of field `PLLRDY`
pub type PLLRDY_R = crate::R<bool, bool>;
///Reader of field `PLLON`
pub type PLLON_R = crate::R<bool, bool>;
///Write proxy for field `PLLON`
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
///Write proxy for field `CSSON`
pub struct CSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
///Reader of field `HSEBYP`
pub type HSEBYP_R = crate::R<bool, bool>;
///Write proxy for field `HSEBYP`
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
///Reader of field `HSERDY`
pub type HSERDY_R = crate::R<bool, bool>;
///Reader of field `HSEON`
pub type HSEON_R = crate::R<bool, bool>;
///Write proxy for field `HSEON`
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
///Reader of field `HSIASFS`
pub type HSIASFS_R = crate::R<bool, bool>;
///Write proxy for field `HSIASFS`
pub struct HSIASFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIASFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
///Reader of field `HSIRDY`
pub type HSIRDY_R = crate::R<bool, bool>;
///Reader of field `HSIKERON`
pub type HSIKERON_R = crate::R<bool, bool>;
///Write proxy for field `HSIKERON`
pub struct HSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIKERON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
///Reader of field `HSION`
pub type HSION_R = crate::R<bool, bool>;
///Write proxy for field `HSION`
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
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
///MSI clock ranges
///
///Value on reset: 6
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    ///0: range 0 around 100 kHz
    RANGE100K = 0,
    ///1: range 1 around 200 kHz
    RANGE200K = 1,
    ///2: range 2 around 400 kHz
    RANGE400K = 2,
    ///3: range 3 around 800 kHz
    RANGE800K = 3,
    ///4: range 4 around 1 MHz
    RANGE1M = 4,
    ///5: range 5 around 2 MHz
    RANGE2M = 5,
    ///6: range 6 around 4 MHz
    RANGE4M = 6,
    ///7: range 7 around 8 MHz
    RANGE8M = 7,
    ///8: range 8 around 16 MHz
    RANGE16M = 8,
    ///9: range 9 around 24 MHz
    RANGE24M = 9,
    ///10: range 10 around 32 MHz
    RANGE32M = 10,
    ///11: range 11 around 48 MHz
    RANGE48M = 11,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
///Reader of field `MSIRANGE`
pub type MSIRANGE_R = crate::R<u8, MSIRANGE_A>;
impl MSIRANGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MSIRANGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MSIRANGE_A::RANGE100K),
            1 => Val(MSIRANGE_A::RANGE200K),
            2 => Val(MSIRANGE_A::RANGE400K),
            3 => Val(MSIRANGE_A::RANGE800K),
            4 => Val(MSIRANGE_A::RANGE1M),
            5 => Val(MSIRANGE_A::RANGE2M),
            6 => Val(MSIRANGE_A::RANGE4M),
            7 => Val(MSIRANGE_A::RANGE8M),
            8 => Val(MSIRANGE_A::RANGE16M),
            9 => Val(MSIRANGE_A::RANGE24M),
            10 => Val(MSIRANGE_A::RANGE32M),
            11 => Val(MSIRANGE_A::RANGE48M),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `RANGE100K`
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGE_A::RANGE100K
    }
    ///Checks if the value of the field is `RANGE200K`
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGE_A::RANGE200K
    }
    ///Checks if the value of the field is `RANGE400K`
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGE_A::RANGE400K
    }
    ///Checks if the value of the field is `RANGE800K`
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGE_A::RANGE800K
    }
    ///Checks if the value of the field is `RANGE1M`
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGE_A::RANGE1M
    }
    ///Checks if the value of the field is `RANGE2M`
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGE_A::RANGE2M
    }
    ///Checks if the value of the field is `RANGE4M`
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGE_A::RANGE4M
    }
    ///Checks if the value of the field is `RANGE8M`
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGE_A::RANGE8M
    }
    ///Checks if the value of the field is `RANGE16M`
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGE_A::RANGE16M
    }
    ///Checks if the value of the field is `RANGE24M`
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGE_A::RANGE24M
    }
    ///Checks if the value of the field is `RANGE32M`
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGE_A::RANGE32M
    }
    ///Checks if the value of the field is `RANGE48M`
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGE_A::RANGE48M
    }
}
///Write proxy for field `MSIRANGE`
pub struct MSIRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRANGE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MSIRANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///range 0 around 100 kHz
    #[inline(always)]
    pub fn range100k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE100K)
    }
    ///range 1 around 200 kHz
    #[inline(always)]
    pub fn range200k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE200K)
    }
    ///range 2 around 400 kHz
    #[inline(always)]
    pub fn range400k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE400K)
    }
    ///range 3 around 800 kHz
    #[inline(always)]
    pub fn range800k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE800K)
    }
    ///range 4 around 1 MHz
    #[inline(always)]
    pub fn range1m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE1M)
    }
    ///range 5 around 2 MHz
    #[inline(always)]
    pub fn range2m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE2M)
    }
    ///range 6 around 4 MHz
    #[inline(always)]
    pub fn range4m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE4M)
    }
    ///range 7 around 8 MHz
    #[inline(always)]
    pub fn range8m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE8M)
    }
    ///range 8 around 16 MHz
    #[inline(always)]
    pub fn range16m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE16M)
    }
    ///range 9 around 24 MHz
    #[inline(always)]
    pub fn range24m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE24M)
    }
    ///range 10 around 32 MHz
    #[inline(always)]
    pub fn range32m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE32M)
    }
    ///range 11 around 48 MHz
    #[inline(always)]
    pub fn range48m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::RANGE48M)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
///Write proxy for field `MSIRGSEL`
pub struct MSIRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIRGSEL_W<'a> {
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
///Reader of field `MSIPLLEN`
pub type MSIPLLEN_R = crate::R<bool, bool>;
///Write proxy for field `MSIPLLEN`
pub struct MSIPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIPLLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `MSIRDY`
pub type MSIRDY_R = crate::R<bool, bool>;
///Reader of field `MSION`
pub type MSION_R = crate::R<bool, bool>;
///Write proxy for field `MSION`
pub struct MSION_W<'a> {
    w: &'a mut W,
}
impl<'a> MSION_W<'a> {
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
impl R {
    ///Bit 27 - SAI1 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 25 - Main PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W {
        PLLSAI1ON_W { w: self }
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W {
        CSSON_W { w: self }
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HSIASFS_W {
        HSIASFS_W { w: self }
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W {
        HSIKERON_W { w: self }
    }
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W {
        MSIRANGE_W { w: self }
    }
    ///Bit 3 - MSI clock range selection
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W {
        MSIRGSEL_W { w: self }
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&mut self) -> MSIPLLEN_W {
        MSIPLLEN_W { w: self }
    }
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W {
        MSION_W { w: self }
    }
}
