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
///Early wakeup interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIF_A {
    ///1: The EWI Interrupt Service Routine has been triggered
    PENDING = 1,
    ///0: The EWI Interrupt Service Routine has been serviced
    FINISHED = 0,
}
impl From<EWIF_A> for bool {
    #[inline(always)]
    fn from(variant: EWIF_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `EWIF`
pub type EWIF_R = crate::R<bool, EWIF_A>;
impl EWIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWIF_A {
        match self.bits {
            true => EWIF_A::PENDING,
            false => EWIF_A::FINISHED,
        }
    }
    ///Checks if the value of the field is `PENDING`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIF_A::PENDING
    }
    ///Checks if the value of the field is `FINISHED`
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == EWIF_A::FINISHED
    }
}
///Early wakeup interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIF_AW {
    ///0: The EWI Interrupt Service Routine has been serviced
    FINISHED = 0,
}
impl From<EWIF_AW> for bool {
    #[inline(always)]
    fn from(variant: EWIF_AW) -> Self {
        variant as u8 != 0
    }
}
///Write proxy for field `EWIF`
pub struct EWIF_W<'a> {
    w: &'a mut W,
}
impl<'a> EWIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWIF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    ///The EWI Interrupt Service Routine has been serviced
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(EWIF_AW::FINISHED)
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
    ///Bit 0 - Early wakeup interrupt flag
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Early wakeup interrupt flag
    #[inline(always)]
    pub fn ewif(&mut self) -> EWIF_W {
        EWIF_W { w: self }
    }
}
