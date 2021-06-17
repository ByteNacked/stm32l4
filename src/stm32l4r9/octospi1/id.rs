///Reader of register ID
pub type R = crate::R<u32, super::ID>;
///Reader of field `ID`
pub type ID_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - Identification
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
