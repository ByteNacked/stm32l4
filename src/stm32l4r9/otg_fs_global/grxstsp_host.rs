///Reader of register GRXSTSP_Host
pub type R = crate::R<u32, super::GRXSTSP_HOST>;
///Reader of field `PKTSTS`
pub type PKTSTS_R = crate::R<u8, u8>;
///Reader of field `DPID`
pub type DPID_R = crate::R<u8, u8>;
///Reader of field `BCNT`
pub type BCNT_R = crate::R<u16, u16>;
///Reader of field `CHNUM`
pub type CHNUM_R = crate::R<u8, u8>;
impl R {
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
    ///Bits 0:3 - Channel number
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
}
