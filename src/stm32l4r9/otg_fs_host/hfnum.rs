///Reader of register HFNUM
pub type R = crate::R<u32, super::HFNUM>;
///Reader of field `FRNUM`
pub type FRNUM_R = crate::R<u16, u16>;
///Reader of field `FTREM`
pub type FTREM_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:15 - Frame number
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Frame time remaining
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
