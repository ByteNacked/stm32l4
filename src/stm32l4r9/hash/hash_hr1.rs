///Reader of register HASH_HR1
pub type R = crate::R<u32, super::HASH_HR1>;
///Reader of field `H1`
pub type H1_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - H1
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
