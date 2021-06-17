///Reader of register MID
pub type R = crate::R<u32, super::MID>;
///Reader of field `MID`
pub type MID_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - Magic ID
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
