///Reader of register PECR
pub type R = crate::R<u32, super::PECR>;
///Reader of field `PEC`
pub type PEC_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:7 - Packet error checking register
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
