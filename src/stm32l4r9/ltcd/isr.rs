///Reader of register ISR
pub type R = crate::R<u32, super::ISR>;
///Reader of field `LIF`
pub type LIF_R = crate::R<bool, bool>;
///Reader of field `FUIF`
pub type FUIF_R = crate::R<bool, bool>;
///Reader of field `TERRIF`
pub type TERRIF_R = crate::R<bool, bool>;
///Reader of field `RRIF`
pub type RRIF_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - Line Interrupt flag
    #[inline(always)]
    pub fn lif(&self) -> LIF_R {
        LIF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt flag
    #[inline(always)]
    pub fn fuif(&self) -> FUIF_R {
        FUIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Transfer Error interrupt flag
    #[inline(always)]
    pub fn terrif(&self) -> TERRIF_R {
        TERRIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Register Reload Interrupt Flag
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
