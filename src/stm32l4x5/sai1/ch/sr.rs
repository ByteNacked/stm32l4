///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Writer for register SR
pub type W = crate::W<u32, super::SR>;
///Register SR `reset()`'s with value 0
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///FIFO level threshold
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLVL_A {
    ///0: FIFO empty
    EMPTY = 0,
    ///1: FIFO <= 1⁄4 but not empty
    QUARTER1 = 1,
    ///2: 1⁄4 < FIFO <= 1⁄2
    QUARTER2 = 2,
    ///3: 1⁄2 < FIFO <= 3⁄4
    QUARTER3 = 3,
    ///4: 3⁄4 < FIFO but not full
    QUARTER4 = 4,
    ///5: FIFO full
    FULL = 5,
}
impl From<FLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FLVL_A) -> Self {
        variant as _
    }
}
///Reader of field `FLVL`
pub type FLVL_R = crate::R<u8, FLVL_A>;
impl FLVL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLVL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLVL_A::EMPTY),
            1 => Val(FLVL_A::QUARTER1),
            2 => Val(FLVL_A::QUARTER2),
            3 => Val(FLVL_A::QUARTER3),
            4 => Val(FLVL_A::QUARTER4),
            5 => Val(FLVL_A::FULL),
            i => Res(i),
        }
    }
    ///Checks if the value of the field is `EMPTY`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FLVL_A::EMPTY
    }
    ///Checks if the value of the field is `QUARTER1`
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FLVL_A::QUARTER1
    }
    ///Checks if the value of the field is `QUARTER2`
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FLVL_A::QUARTER2
    }
    ///Checks if the value of the field is `QUARTER3`
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FLVL_A::QUARTER3
    }
    ///Checks if the value of the field is `QUARTER4`
    #[inline(always)]
    pub fn is_quarter4(&self) -> bool {
        *self == FLVL_A::QUARTER4
    }
    ///Checks if the value of the field is `FULL`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FLVL_A::FULL
    }
}
///Write proxy for field `FLVL`
pub struct FLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLVL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FLVL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///FIFO empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(FLVL_A::EMPTY)
    }
    ///FIFO <= 1⁄4 but not empty
    #[inline(always)]
    pub fn quarter1(self) -> &'a mut W {
        self.variant(FLVL_A::QUARTER1)
    }
    ///1⁄4 < FIFO <= 1⁄2
    #[inline(always)]
    pub fn quarter2(self) -> &'a mut W {
        self.variant(FLVL_A::QUARTER2)
    }
    ///1⁄2 < FIFO <= 3⁄4
    #[inline(always)]
    pub fn quarter3(self) -> &'a mut W {
        self.variant(FLVL_A::QUARTER3)
    }
    ///3⁄4 < FIFO but not full
    #[inline(always)]
    pub fn quarter4(self) -> &'a mut W {
        self.variant(FLVL_A::QUARTER4)
    }
    ///FIFO full
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FLVL_A::FULL)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
///Late frame synchronization detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSDET_A {
    ///0: No error
    NOERROR = 0,
    ///1: Frame synchronization signal is not present at the right time
    NOSYNC = 1,
}
impl From<LFSDET_A> for bool {
    #[inline(always)]
    fn from(variant: LFSDET_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `LFSDET`
pub type LFSDET_R = crate::R<bool, LFSDET_A>;
impl LFSDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LFSDET_A {
        match self.bits {
            false => LFSDET_A::NOERROR,
            true => LFSDET_A::NOSYNC,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LFSDET_A::NOERROR
    }
    ///Checks if the value of the field is `NOSYNC`
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == LFSDET_A::NOSYNC
    }
}
///Write proxy for field `LFSDET`
pub struct LFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSDET_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LFSDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No error
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LFSDET_A::NOERROR)
    }
    ///Frame synchronization signal is not present at the right time
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut W {
        self.variant(LFSDET_A::NOSYNC)
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
///Anticipated frame synchronization detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFSDET_A {
    ///0: No error
    NOERROR = 0,
    ///1: Frame synchronization signal is detected earlier than expected
    EARLYSYNC = 1,
}
impl From<AFSDET_A> for bool {
    #[inline(always)]
    fn from(variant: AFSDET_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `AFSDET`
pub type AFSDET_R = crate::R<bool, AFSDET_A>;
impl AFSDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFSDET_A {
        match self.bits {
            false => AFSDET_A::NOERROR,
            true => AFSDET_A::EARLYSYNC,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AFSDET_A::NOERROR
    }
    ///Checks if the value of the field is `EARLYSYNC`
    #[inline(always)]
    pub fn is_early_sync(&self) -> bool {
        *self == AFSDET_A::EARLYSYNC
    }
}
///Write proxy for field `AFSDET`
pub struct AFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSDET_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: AFSDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No error
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(AFSDET_A::NOERROR)
    }
    ///Frame synchronization signal is detected earlier than expected
    #[inline(always)]
    pub fn early_sync(self) -> &'a mut W {
        self.variant(AFSDET_A::EARLYSYNC)
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
///Codec not ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNRDY_A {
    ///0: External AC’97 Codec is ready
    READY = 0,
    ///1: External AC’97 Codec is not ready
    NOTREADY = 1,
}
impl From<CNRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CNRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `CNRDY`
pub type CNRDY_R = crate::R<bool, CNRDY_A>;
impl CNRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CNRDY_A {
        match self.bits {
            false => CNRDY_A::READY,
            true => CNRDY_A::NOTREADY,
        }
    }
    ///Checks if the value of the field is `READY`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == CNRDY_A::READY
    }
    ///Checks if the value of the field is `NOTREADY`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == CNRDY_A::NOTREADY
    }
}
///Write proxy for field `CNRDY`
pub struct CNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CNRDY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CNRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///External AC’97 Codec is ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(CNRDY_A::READY)
    }
    ///External AC’97 Codec is not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(CNRDY_A::NOTREADY)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
///FIFO request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQ_A {
    ///0: No FIFO request
    NOREQUEST = 0,
    ///1: FIFO request to read or to write the SAI_xDR
    REQUEST = 1,
}
impl From<FREQ_A> for bool {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `FREQ`
pub type FREQ_R = crate::R<bool, FREQ_A>;
impl FREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FREQ_A {
        match self.bits {
            false => FREQ_A::NOREQUEST,
            true => FREQ_A::REQUEST,
        }
    }
    ///Checks if the value of the field is `NOREQUEST`
    #[inline(always)]
    pub fn is_no_request(&self) -> bool {
        *self == FREQ_A::NOREQUEST
    }
    ///Checks if the value of the field is `REQUEST`
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == FREQ_A::REQUEST
    }
}
///Write proxy for field `FREQ`
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FREQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No FIFO request
    #[inline(always)]
    pub fn no_request(self) -> &'a mut W {
        self.variant(FREQ_A::NOREQUEST)
    }
    ///FIFO request to read or to write the SAI_xDR
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(FREQ_A::REQUEST)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Wrong clock configuration flag. This bit is read only
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCKCFG_A {
    ///0: Clock configuration is correct
    CORRECT = 0,
    ///1: Clock configuration does not respect the rule concerning the frame length specification
    WRONG = 1,
}
impl From<WCKCFG_A> for bool {
    #[inline(always)]
    fn from(variant: WCKCFG_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `WCKCFG`
pub type WCKCFG_R = crate::R<bool, WCKCFG_A>;
impl WCKCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WCKCFG_A {
        match self.bits {
            false => WCKCFG_A::CORRECT,
            true => WCKCFG_A::WRONG,
        }
    }
    ///Checks if the value of the field is `CORRECT`
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == WCKCFG_A::CORRECT
    }
    ///Checks if the value of the field is `WRONG`
    #[inline(always)]
    pub fn is_wrong(&self) -> bool {
        *self == WCKCFG_A::WRONG
    }
}
///Write proxy for field `WCKCFG`
pub struct WCKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WCKCFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WCKCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///Clock configuration is correct
    #[inline(always)]
    pub fn correct(self) -> &'a mut W {
        self.variant(WCKCFG_A::CORRECT)
    }
    ///Clock configuration does not respect the rule concerning the frame length specification
    #[inline(always)]
    pub fn wrong(self) -> &'a mut W {
        self.variant(WCKCFG_A::WRONG)
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
///Mute detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEDET_A {
    ///0: No MUTE detection on the SD input line
    NOMUTE = 0,
    ///1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame
    MUTE = 1,
}
impl From<MUTEDET_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEDET_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `MUTEDET`
pub type MUTEDET_R = crate::R<bool, MUTEDET_A>;
impl MUTEDET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUTEDET_A {
        match self.bits {
            false => MUTEDET_A::NOMUTE,
            true => MUTEDET_A::MUTE,
        }
    }
    ///Checks if the value of the field is `NOMUTE`
    #[inline(always)]
    pub fn is_no_mute(&self) -> bool {
        *self == MUTEDET_A::NOMUTE
    }
    ///Checks if the value of the field is `MUTE`
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == MUTEDET_A::MUTE
    }
}
///Write proxy for field `MUTEDET`
pub struct MUTEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEDET_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MUTEDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No MUTE detection on the SD input line
    #[inline(always)]
    pub fn no_mute(self) -> &'a mut W {
        self.variant(MUTEDET_A::NOMUTE)
    }
    ///MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio frame
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MUTEDET_A::MUTE)
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
///Overrun / underrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRUDR_A {
    ///0: No overrun/underrun error
    NOERROR = 0,
    ///1: Overrun/underrun error detection
    OVERRUN = 1,
}
impl From<OVRUDR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRUDR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `OVRUDR`
pub type OVRUDR_R = crate::R<bool, OVRUDR_A>;
impl OVRUDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRUDR_A {
        match self.bits {
            false => OVRUDR_A::NOERROR,
            true => OVRUDR_A::OVERRUN,
        }
    }
    ///Checks if the value of the field is `NOERROR`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OVRUDR_A::NOERROR
    }
    ///Checks if the value of the field is `OVERRUN`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRUDR_A::OVERRUN
    }
}
///Write proxy for field `OVRUDR`
pub struct OVRUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRUDR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRUDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///No overrun/underrun error
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(OVRUDR_A::NOERROR)
    }
    ///Overrun/underrun error detection
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVRUDR_A::OVERRUN)
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
impl R {
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&self) -> FLVL_R {
        FLVL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&self) -> LFSDET_R {
        LFSDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&self) -> AFSDET_R {
        AFSDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&self) -> CNRDY_R {
        CNRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Wrong clock configuration flag. This bit is read only
    #[inline(always)]
    pub fn wckcfg(&self) -> WCKCFG_R {
        WCKCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&self) -> MUTEDET_R {
        MUTEDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&self) -> OVRUDR_R {
        OVRUDR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 16:18 - FIFO level threshold
    #[inline(always)]
    pub fn flvl(&mut self) -> FLVL_W {
        FLVL_W { w: self }
    }
    ///Bit 6 - Late frame synchronization detection
    #[inline(always)]
    pub fn lfsdet(&mut self) -> LFSDET_W {
        LFSDET_W { w: self }
    }
    ///Bit 5 - Anticipated frame synchronization detection
    #[inline(always)]
    pub fn afsdet(&mut self) -> AFSDET_W {
        AFSDET_W { w: self }
    }
    ///Bit 4 - Codec not ready
    #[inline(always)]
    pub fn cnrdy(&mut self) -> CNRDY_W {
        CNRDY_W { w: self }
    }
    ///Bit 3 - FIFO request
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    ///Bit 2 - Wrong clock configuration flag. This bit is read only
    #[inline(always)]
    pub fn wckcfg(&mut self) -> WCKCFG_W {
        WCKCFG_W { w: self }
    }
    ///Bit 1 - Mute detection
    #[inline(always)]
    pub fn mutedet(&mut self) -> MUTEDET_W {
        MUTEDET_W { w: self }
    }
    ///Bit 0 - Overrun / underrun
    #[inline(always)]
    pub fn ovrudr(&mut self) -> OVRUDR_W {
        OVRUDR_W { w: self }
    }
}
