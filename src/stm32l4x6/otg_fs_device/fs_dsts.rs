///Reader of register FS_DSTS
pub type R = crate::R<u32, super::FS_DSTS>;
///Reader of field `SUSPSTS`
pub type SUSPSTS_R = crate::R<bool, bool>;
///Reader of field `ENUMSPD`
pub type ENUMSPD_R = crate::R<u8, u8>;
///Reader of field `EERR`
pub type EERR_R = crate::R<bool, bool>;
///Reader of field `FNSOF`
pub type FNSOF_R = crate::R<u16, u16>;
impl R {
    ///Bit 0 - Suspend status
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:2 - Enumerated speed
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    ///Bit 3 - Erratic error
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 8:21 - Frame number of the received SOF
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
