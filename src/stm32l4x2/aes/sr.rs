///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Reader of field `WRERR`
pub type WRERR_R = crate::R<bool, bool>;
///Reader of field `RDERR`
pub type RDERR_R = crate::R<bool, bool>;
///Reader of field `CCF`
pub type CCF_R = crate::R<bool, bool>;
impl R {
    ///Bit 2 - Write error flag
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Read error flag
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Computation complete flag
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 0x01) != 0)
    }
}
