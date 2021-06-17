///Reader of register DFSDM_FLT1AWSR
pub type R = crate::R<u32, super::DFSDM_FLT1AWSR>;
///Reader of field `AWHTF`
pub type AWHTF_R = crate::R<u8, u8>;
///Reader of field `AWLTF`
pub type AWLTF_R = crate::R<u8, u8>;
impl R {
    ///Bits 8:15 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 0:7 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
}
