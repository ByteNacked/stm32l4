///Reader of register GRXSTSP_Device
pub type R = crate::R<u32, super::GRXSTSP_DEVICE>;
///Reader of field `STSPHST`
pub type STSPHST_R = crate::R<bool, bool>;
///Reader of field `FRMNUM`
pub type FRMNUM_R = crate::R<u8, u8>;
///Reader of field `PKTSTS`
pub type PKTSTS_R = crate::R<u8, u8>;
///Reader of field `DPID`
pub type DPID_R = crate::R<u8, u8>;
///Reader of field `BCNT`
pub type BCNT_R = crate::R<u16, u16>;
///Reader of field `EPNUM`
pub type EPNUM_R = crate::R<u8, u8>;
impl R {
    ///Bit 27 - Status phase start
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bits 21:24 - Frame number
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    ///Bits 17:20 - Packet status
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 15:16 - Data PID
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    ///Bits 4:14 - Byte count
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bits 0:3 - Endpoint number
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
}
