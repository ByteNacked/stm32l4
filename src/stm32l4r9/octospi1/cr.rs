///Reader of register CR
pub type R = crate::R<u32, super::CR>;
///Writer for register CR
pub type W = crate::W<u32, super::CR>;
///Register CR `reset()`'s with value 0
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Functional mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FMODE_A {
    ///0: Inderect-write mode
    INDIRECTWRITE = 0,
    ///1: Inderect-read mode
    INDIRECTREAD = 1,
    ///2: Automatic status-polling mode
    AUTOPOLLING = 2,
    ///3: Memory-mapped mode
    MEMMAPPED = 3,
}
impl From<FMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FMODE_A) -> Self {
        variant as _
    }
}
///Reader of field `FMODE`
pub type FMODE_R = crate::R<u8, FMODE_A>;
impl FMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMODE_A {
        match self.bits {
            0 => FMODE_A::INDIRECTWRITE,
            1 => FMODE_A::INDIRECTREAD,
            2 => FMODE_A::AUTOPOLLING,
            3 => FMODE_A::MEMMAPPED,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `INDIRECTWRITE`
    #[inline(always)]
    pub fn is_indirect_write(&self) -> bool {
        *self == FMODE_A::INDIRECTWRITE
    }
    ///Checks if the value of the field is `INDIRECTREAD`
    #[inline(always)]
    pub fn is_indirect_read(&self) -> bool {
        *self == FMODE_A::INDIRECTREAD
    }
    ///Checks if the value of the field is `AUTOPOLLING`
    #[inline(always)]
    pub fn is_auto_polling(&self) -> bool {
        *self == FMODE_A::AUTOPOLLING
    }
    ///Checks if the value of the field is `MEMMAPPED`
    #[inline(always)]
    pub fn is_mem_mapped(&self) -> bool {
        *self == FMODE_A::MEMMAPPED
    }
}
///Write proxy for field `FMODE`
pub struct FMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    ///Inderect-write mode
    #[inline(always)]
    pub fn indirect_write(self) -> &'a mut W {
        self.variant(FMODE_A::INDIRECTWRITE)
    }
    ///Inderect-read mode
    #[inline(always)]
    pub fn indirect_read(self) -> &'a mut W {
        self.variant(FMODE_A::INDIRECTREAD)
    }
    ///Automatic status-polling mode
    #[inline(always)]
    pub fn auto_polling(self) -> &'a mut W {
        self.variant(FMODE_A::AUTOPOLLING)
    }
    ///Memory-mapped mode
    #[inline(always)]
    pub fn mem_mapped(self) -> &'a mut W {
        self.variant(FMODE_A::MEMMAPPED)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
///Reader of field `PMM`
pub type PMM_R = crate::R<bool, bool>;
///Write proxy for field `PMM`
pub struct PMM_W<'a> {
    w: &'a mut W,
}
impl<'a> PMM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
///Reader of field `APMS`
pub type APMS_R = crate::R<bool, bool>;
///Write proxy for field `APMS`
pub struct APMS_W<'a> {
    w: &'a mut W,
}
impl<'a> APMS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
///Reader of field `TOIE`
pub type TOIE_R = crate::R<bool, bool>;
///Write proxy for field `TOIE`
pub struct TOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
///Reader of field `SMIE`
pub type SMIE_R = crate::R<bool, bool>;
///Write proxy for field `SMIE`
pub struct SMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMIE_W<'a> {
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
///Reader of field `FTIE`
pub type FTIE_R = crate::R<bool, bool>;
///Write proxy for field `FTIE`
pub struct FTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FTIE_W<'a> {
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
///Reader of field `TCIE`
pub type TCIE_R = crate::R<bool, bool>;
///Write proxy for field `TCIE`
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
///Reader of field `TEIE`
pub type TEIE_R = crate::R<bool, bool>;
///Write proxy for field `TEIE`
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
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
///Reader of field `FTHRES`
pub type FTHRES_R = crate::R<u8, u8>;
///Write proxy for field `FTHRES`
pub struct FTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FTHRES_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
///Reader of field `FSEL`
pub type FSEL_R = crate::R<bool, bool>;
///Write proxy for field `FSEL`
pub struct FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSEL_W<'a> {
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
///Reader of field `DQM`
pub type DQM_R = crate::R<bool, bool>;
///Write proxy for field `DQM`
pub struct DQM_W<'a> {
    w: &'a mut W,
}
impl<'a> DQM_W<'a> {
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
///Reader of field `TCEN`
pub type TCEN_R = crate::R<bool, bool>;
///Write proxy for field `TCEN`
pub struct TCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TCEN_W<'a> {
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
///Reader of field `DMAEN`
pub type DMAEN_R = crate::R<bool, bool>;
///Write proxy for field `DMAEN`
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
///Reader of field `ABORT`
pub type ABORT_R = crate::R<bool, bool>;
///Write proxy for field `ABORT`
pub struct ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ABORT_W<'a> {
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
///Reader of field `EN`
pub type EN_R = crate::R<bool, bool>;
///Write proxy for field `EN`
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    ///Bits 28:29 - Functional mode
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 8:12 - IFO threshold level
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 7 - FLASH memory selection
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - Dual-quad mode
    #[inline(always)]
    pub fn dqm(&self) -> DQM_R {
        DQM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 28:29 - Functional mode
    #[inline(always)]
    pub fn fmode(&mut self) -> FMODE_W {
        FMODE_W { w: self }
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&mut self) -> PMM_W {
        PMM_W { w: self }
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    pub fn apms(&mut self) -> APMS_W {
        APMS_W { w: self }
    }
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W {
        TOIE_W { w: self }
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&mut self) -> SMIE_W {
        SMIE_W { w: self }
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    pub fn ftie(&mut self) -> FTIE_W {
        FTIE_W { w: self }
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    ///Bits 8:12 - IFO threshold level
    #[inline(always)]
    pub fn fthres(&mut self) -> FTHRES_W {
        FTHRES_W { w: self }
    }
    ///Bit 7 - FLASH memory selection
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W {
        FSEL_W { w: self }
    }
    ///Bit 6 - Dual-quad mode
    #[inline(always)]
    pub fn dqm(&mut self) -> DQM_W {
        DQM_W { w: self }
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    pub fn tcen(&mut self) -> TCEN_W {
        TCEN_W { w: self }
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    pub fn abort(&mut self) -> ABORT_W {
        ABORT_W { w: self }
    }
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
