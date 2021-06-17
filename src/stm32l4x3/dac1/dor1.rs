///Reader of register DOR1
pub type R = crate::R<u32, super::DOR1>;
///Reader of field `DACC1DOR`
pub type DACC1DOR_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:11 - DAC channel1 data output
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
