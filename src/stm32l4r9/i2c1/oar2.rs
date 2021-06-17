///Reader of register OAR2
pub type R = crate::R<u32, super::OAR2>;
///Writer for register OAR2
pub type W = crate::W<u32, super::OAR2>;
///Register OAR2 `reset()`'s with value 0
impl crate::ResetValue for super::OAR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `OA2`
pub type OA2_R = crate::R<u8, u8>;
///Write proxy for field `OA2`
pub struct OA2_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
///Own Address 2 masks
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA2MSK_A {
    ///0: No mask
    NOMASK = 0,
    ///1: OA2\[1\]
    ///is masked and don’t care. Only OA2\[7:2\]
    ///are compared
    MASK1 = 1,
    ///2: OA2\[2:1\]
    ///are masked and don’t care. Only OA2\[7:3\]
    ///are compared
    MASK2 = 2,
    ///3: OA2\[3:1\]
    ///are masked and don’t care. Only OA2\[7:4\]
    ///are compared
    MASK3 = 3,
    ///4: OA2\[4:1\]
    ///are masked and don’t care. Only OA2\[7:5\]
    ///are compared
    MASK4 = 4,
    ///5: OA2\[5:1\]
    ///are masked and don’t care. Only OA2\[7:6\]
    ///are compared
    MASK5 = 5,
    ///6: OA2\[6:1\]
    ///are masked and don’t care. Only OA2\[7\]
    ///is compared.
    MASK6 = 6,
    ///7: OA2\[7:1\]
    ///are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged
    MASK7 = 7,
}
impl From<OA2MSK_A> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK_A) -> Self {
        variant as _
    }
}
///Reader of field `OA2MSK`
pub type OA2MSK_R = crate::R<u8, OA2MSK_A>;
impl OA2MSK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OA2MSK_A {
        match self.bits {
            0 => OA2MSK_A::NOMASK,
            1 => OA2MSK_A::MASK1,
            2 => OA2MSK_A::MASK2,
            3 => OA2MSK_A::MASK3,
            4 => OA2MSK_A::MASK4,
            5 => OA2MSK_A::MASK5,
            6 => OA2MSK_A::MASK6,
            7 => OA2MSK_A::MASK7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NOMASK`
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == OA2MSK_A::NOMASK
    }
    ///Checks if the value of the field is `MASK1`
    #[inline(always)]
    pub fn is_mask1(&self) -> bool {
        *self == OA2MSK_A::MASK1
    }
    ///Checks if the value of the field is `MASK2`
    #[inline(always)]
    pub fn is_mask2(&self) -> bool {
        *self == OA2MSK_A::MASK2
    }
    ///Checks if the value of the field is `MASK3`
    #[inline(always)]
    pub fn is_mask3(&self) -> bool {
        *self == OA2MSK_A::MASK3
    }
    ///Checks if the value of the field is `MASK4`
    #[inline(always)]
    pub fn is_mask4(&self) -> bool {
        *self == OA2MSK_A::MASK4
    }
    ///Checks if the value of the field is `MASK5`
    #[inline(always)]
    pub fn is_mask5(&self) -> bool {
        *self == OA2MSK_A::MASK5
    }
    ///Checks if the value of the field is `MASK6`
    #[inline(always)]
    pub fn is_mask6(&self) -> bool {
        *self == OA2MSK_A::MASK6
    }
    ///Checks if the value of the field is `MASK7`
    #[inline(always)]
    pub fn is_mask7(&self) -> bool {
        *self == OA2MSK_A::MASK7
    }
}
///Write proxy for field `OA2MSK`
pub struct OA2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2MSK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OA2MSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///No mask
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut W {
        self.variant(OA2MSK_A::NOMASK)
    }
    ///OA2\[1\]
    ///is masked and don’t care. Only OA2\[7:2\]
    ///are compared
    #[inline(always)]
    pub fn mask1(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK1)
    }
    ///OA2\[2:1\]
    ///are masked and don’t care. Only OA2\[7:3\]
    ///are compared
    #[inline(always)]
    pub fn mask2(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK2)
    }
    ///OA2\[3:1\]
    ///are masked and don’t care. Only OA2\[7:4\]
    ///are compared
    #[inline(always)]
    pub fn mask3(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK3)
    }
    ///OA2\[4:1\]
    ///are masked and don’t care. Only OA2\[7:5\]
    ///are compared
    #[inline(always)]
    pub fn mask4(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK4)
    }
    ///OA2\[5:1\]
    ///are masked and don’t care. Only OA2\[7:6\]
    ///are compared
    #[inline(always)]
    pub fn mask5(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK5)
    }
    ///OA2\[6:1\]
    ///are masked and don’t care. Only OA2\[7\]
    ///is compared.
    #[inline(always)]
    pub fn mask6(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK6)
    }
    ///OA2\[7:1\]
    ///are masked and don’t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged
    #[inline(always)]
    pub fn mask7(self) -> &'a mut W {
        self.variant(OA2MSK_A::MASK7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
///Own Address 2 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA2EN_A {
    ///0: Own address 2 disabled. The received slave address OA2 is NACKed
    DISABLED = 0,
    ///1: Own address 2 enabled. The received slave address OA2 is ACKed
    ENABLED = 1,
}
impl From<OA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA2EN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OA2EN`
pub type OA2EN_R = crate::R<bool, OA2EN_A>;
impl OA2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OA2EN_A {
        match self.bits {
            false => OA2EN_A::DISABLED,
            true => OA2EN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OA2EN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OA2EN_A::ENABLED
    }
}
///Write proxy for field `OA2EN`
pub struct OA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OA2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Own address 2 disabled. The received slave address OA2 is NACKed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OA2EN_A::DISABLED)
    }
    ///Own address 2 enabled. The received slave address OA2 is ACKed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OA2EN_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    ///Bits 1:7 - Interface address
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bits 8:10 - Own Address 2 masks
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 15 - Own Address 2 enable
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bits 1:7 - Interface address
    #[inline(always)]
    pub fn oa2(&mut self) -> OA2_W {
        OA2_W { w: self }
    }
    ///Bits 8:10 - Own Address 2 masks
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W {
        OA2MSK_W { w: self }
    }
    ///Bit 15 - Own Address 2 enable
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W {
        OA2EN_W { w: self }
    }
}
