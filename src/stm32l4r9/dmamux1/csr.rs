///Reader of register CSR
pub type R = crate::R<u32, super::CSR>;
///Reader of field `SOF0`
pub type SOF0_R = crate::R<bool, bool>;
///Reader of field `SOF1`
pub type SOF1_R = crate::R<bool, bool>;
///Reader of field `SOF2`
pub type SOF2_R = crate::R<bool, bool>;
///Reader of field `SOF3`
pub type SOF3_R = crate::R<bool, bool>;
///Reader of field `SOF4`
pub type SOF4_R = crate::R<bool, bool>;
///Reader of field `SOF5`
pub type SOF5_R = crate::R<bool, bool>;
///Reader of field `SOF6`
pub type SOF6_R = crate::R<bool, bool>;
///Reader of field `SOF7`
pub type SOF7_R = crate::R<bool, bool>;
///Reader of field `SOF8`
pub type SOF8_R = crate::R<bool, bool>;
///Reader of field `SOF9`
pub type SOF9_R = crate::R<bool, bool>;
///Reader of field `SOF10`
pub type SOF10_R = crate::R<bool, bool>;
///Reader of field `SOF11`
pub type SOF11_R = crate::R<bool, bool>;
///Reader of field `SOF12`
pub type SOF12_R = crate::R<bool, bool>;
///Reader of field `SOF13`
pub type SOF13_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Synchronization overrun event flag
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
