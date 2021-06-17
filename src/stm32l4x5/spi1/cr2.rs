///Reader of register CR2
pub type R = crate::R<u32, super::CR2>;
///Writer for register CR2
pub type W = crate::W<u32, super::CR2>;
///Register CR2 `reset()`'s with value 0
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Rx buffer DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    ///0: Rx buffer DMA disabled
    DISABLED = 0,
    ///1: Rx buffer DMA enabled
    ENABLED = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXDMAEN`
pub type RXDMAEN_R = crate::R<bool, RXDMAEN_A>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::DISABLED,
            true => RXDMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::ENABLED
    }
}
///Write proxy for field `RXDMAEN`
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::DISABLED)
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
///Tx buffer DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    ///0: Tx buffer DMA disabled
    DISABLED = 0,
    ///1: Tx buffer DMA enabled
    ENABLED = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXDMAEN`
pub type TXDMAEN_R = crate::R<bool, TXDMAEN_A>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::DISABLED,
            true => TXDMAEN_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::ENABLED
    }
}
///Write proxy for field `TXDMAEN`
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::DISABLED)
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///SS output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    ///0: SS output is disabled in master mode
    DISABLED = 0,
    ///1: SS output is enabled in master mode
    ENABLED = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `SSOE`
pub type SSOE_R = crate::R<bool, SSOE_A>;
impl SSOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::DISABLED,
            true => SSOE_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE_A::ENABLED
    }
}
///Write proxy for field `SSOE`
pub struct SSOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SSOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::DISABLED)
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
///Reader of field `NSSP`
pub type NSSP_R = crate::R<bool, bool>;
///Write proxy for field `NSSP`
pub struct NSSP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSSP_W<'a> {
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
///Reader of field `FRF`
pub type FRF_R = crate::R<bool, bool>;
///Write proxy for field `FRF`
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
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
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    ///0: Error interrupt masked
    MASKED = 0,
    ///1: Error interrupt not masked
    NOTMASKED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `ERRIE`
pub type ERRIE_R = crate::R<bool, ERRIE_A>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::MASKED,
            true => ERRIE_A::NOTMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE_A::MASKED
    }
    ///Checks if the value of the field is `NOTMASKED`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE_A::NOTMASKED
    }
}
///Write proxy for field `ERRIE`
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIE_A::MASKED)
    }
    ///Error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
///RX buffer not empty interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    ///0: RXE interrupt masked
    MASKED = 0,
    ///1: RXE interrupt not masked
    NOTMASKED = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RXNEIE`
pub type RXNEIE_R = crate::R<bool, RXNEIE_A>;
impl RXNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::MASKED,
            true => RXNEIE_A::NOTMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE_A::MASKED
    }
    ///Checks if the value of the field is `NOTMASKED`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE_A::NOTMASKED
    }
}
///Write proxy for field `RXNEIE`
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXNEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///RXE interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::MASKED)
    }
    ///RXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
///Tx buffer empty interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    ///0: TXE interrupt masked
    MASKED = 0,
    ///1: TXE interrupt not masked
    NOTMASKED = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TXEIE`
pub type TXEIE_R = crate::R<bool, TXEIE_A>;
impl TXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::MASKED,
            true => TXEIE_A::NOTMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE_A::MASKED
    }
    ///Checks if the value of the field is `NOTMASKED`
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE_A::NOTMASKED
    }
}
///Write proxy for field `TXEIE`
pub struct TXEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///TXE interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIE_A::MASKED)
    }
    ///TXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIE_A::NOTMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
///Reader of field `DS`
pub type DS_R = crate::R<u8, u8>;
///Write proxy for field `DS`
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
///Reader of field `FRXTH`
pub type FRXTH_R = crate::R<bool, bool>;
///Write proxy for field `FRXTH`
pub struct FRXTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRXTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
///Reader of field `LDMA_RX`
pub type LDMA_RX_R = crate::R<bool, bool>;
///Write proxy for field `LDMA_RX`
pub struct LDMA_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_RX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
///Reader of field `LDMA_TX`
pub type LDMA_TX_R = crate::R<bool, bool>;
///Write proxy for field `LDMA_TX`
pub struct LDMA_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W {
        SSOE_W { w: self }
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W {
        NSSP_W { w: self }
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W {
        TXEIE_W { w: self }
    }
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&mut self) -> FRXTH_W {
        FRXTH_W { w: self }
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W {
        LDMA_RX_W { w: self }
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W {
        LDMA_TX_W { w: self }
    }
}
