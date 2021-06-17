///Reader of register DR
pub type R = crate::R<u32, super::DR>;
///Reader of field `RNDATA`
pub type RNDATA_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - Random data
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
