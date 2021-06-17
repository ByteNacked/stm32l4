///Reader of register VER
pub type R = crate::R<u32, super::VER>;
///Reader of field `VER`
pub type VER_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:7 - Version
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
