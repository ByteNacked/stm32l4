///Reader of register DCOUNT
pub type R = crate::R<u32, super::DCOUNT>;
///Reader of field `DATACOUNT`
pub type DATACOUNT_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:24 - Data count value
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
