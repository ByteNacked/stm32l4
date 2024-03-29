///Reader of register ISR
pub type R = crate::R<u32, super::ISR>;
///Reader of field `REACK`
pub type REACK_R = crate::R<bool, bool>;
///Reader of field `TEACK`
pub type TEACK_R = crate::R<bool, bool>;
///Reader of field `WUF`
pub type WUF_R = crate::R<bool, bool>;
///Reader of field `RWU`
pub type RWU_R = crate::R<bool, bool>;
///Reader of field `SBKF`
pub type SBKF_R = crate::R<bool, bool>;
///Reader of field `CMF`
pub type CMF_R = crate::R<bool, bool>;
///Reader of field `BUSY`
pub type BUSY_R = crate::R<bool, bool>;
///Reader of field `ABRF`
pub type ABRF_R = crate::R<bool, bool>;
///Reader of field `ABRE`
pub type ABRE_R = crate::R<bool, bool>;
///Reader of field `EOBF`
pub type EOBF_R = crate::R<bool, bool>;
///Reader of field `RTOF`
pub type RTOF_R = crate::R<bool, bool>;
///Reader of field `CTS`
pub type CTS_R = crate::R<bool, bool>;
///Reader of field `CTSIF`
pub type CTSIF_R = crate::R<bool, bool>;
///Reader of field `LBDF`
pub type LBDF_R = crate::R<bool, bool>;
///Reader of field `TXE`
pub type TXE_R = crate::R<bool, bool>;
///Reader of field `TC`
pub type TC_R = crate::R<bool, bool>;
///Reader of field `RXNE`
pub type RXNE_R = crate::R<bool, bool>;
///Reader of field `IDLE`
pub type IDLE_R = crate::R<bool, bool>;
///Reader of field `ORE`
pub type ORE_R = crate::R<bool, bool>;
///Reader of field `NF`
pub type NF_R = crate::R<bool, bool>;
///Reader of field `FE`
pub type FE_R = crate::R<bool, bool>;
///Reader of field `PE`
pub type PE_R = crate::R<bool, bool>;
///Transmission complete before guard time completion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCBGT_A {
    ///0: Transmission not completed
    NOTCOMPLETED = 0,
    ///1: Transmission has completed
    COMPLETED = 1,
}
impl From<TCBGT_A> for bool {
    #[inline(always)]
    fn from(variant: TCBGT_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `TCBGT`
pub type TCBGT_R = crate::R<bool, TCBGT_A>;
impl TCBGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCBGT_A {
        match self.bits {
            false => TCBGT_A::NOTCOMPLETED,
            true => TCBGT_A::COMPLETED,
        }
    }
    ///Checks if the value of the field is `NOTCOMPLETED`
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TCBGT_A::NOTCOMPLETED
    }
    ///Checks if the value of the field is `COMPLETED`
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TCBGT_A::COMPLETED
    }
}
impl R {
    ///Bit 22 - REACK
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 21 - TEACK
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 20 - WUF
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 19 - RWU
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 18 - SBKF
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 17 - CMF
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 16 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 15 - ABRF
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 14 - ABRE
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 12 - EOBF
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - RTOF
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - CTS
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - CTSIF
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - LBDF
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - TXE
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 6 - TC
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - IDLE
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - ORE
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - NF
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - FE
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - PE
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 25 - Transmission complete before guard time completion
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
