///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Reader of field `FLEVEL`
pub type FLEVEL_R = crate::R<u8, u8>;
///Reader of field `BUSY`
pub type BUSY_R = crate::R<bool, bool>;
///Reader of field `TOF`
pub type TOF_R = crate::R<bool, bool>;
///Reader of field `SMF`
pub type SMF_R = crate::R<bool, bool>;
///Reader of field `FTF`
pub type FTF_R = crate::R<bool, bool>;
///Reader of field `TCF`
pub type TCF_R = crate::R<bool, bool>;
///Reader of field `TEF`
pub type TEF_R = crate::R<bool, bool>;
impl R {
    ///Bits 8:14 - FIFO level
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 5 - Busy
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Timeout flag
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Status match flag
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - FIFO threshold flag
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Transfer complete flag
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Transfer error flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 0x01) != 0)
    }
}
