///Reader of register DSI_VR
pub type R = crate::R<u32, super::DSI_VR>;
///Reader of field `VERSION`
pub type VERSION_R = crate::R<u32, u32>;
impl R {
    ///Bits 0:31 - Version of the DSI Host
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
