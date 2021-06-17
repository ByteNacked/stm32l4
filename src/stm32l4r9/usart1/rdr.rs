///Reader of register RDR
pub type R = crate::R<u32, super::RDR>;
///Reader of field `RDR`
pub type RDR_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:8 - Receive data value
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
