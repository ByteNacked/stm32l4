///Reader of register CPSR
pub type R = crate::R<u32, super::CPSR>;
///Reader of field `CYPOS`
pub type CYPOS_R = crate::R<u16, u16>;
///Reader of field `CXPOS`
pub type CXPOS_R = crate::R<u16, u16>;
impl R {
    ///Bits 0:15 - Current Y Position
    #[inline(always)]
    pub fn cypos(&self) -> CYPOS_R {
        CYPOS_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Current X Position
    #[inline(always)]
    pub fn cxpos(&self) -> CXPOS_R {
        CXPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
