///Reader of register ECCR
pub type R = crate::R<u32, super::ECCR>;
///Reader of field `ECC`
pub type ECC_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - ECCx
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
