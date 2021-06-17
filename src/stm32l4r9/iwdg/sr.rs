///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Reader of field `WVU`
pub type WVU_R = crate::R<bool, bool>;
///Reader of field `RVU`
pub type RVU_R = crate::R<bool, bool>;
///Reader of field `PVU`
pub type PVU_R = crate::R<bool, bool>;
impl R {
    ///Bit 2 - Watchdog counter window value update
    #[inline(always)]
    pub fn wvu(&self) -> WVU_R {
        WVU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Watchdog counter reload value update
    #[inline(always)]
    pub fn rvu(&self) -> RVU_R {
        RVU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Watchdog prescaler value update
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 0x01) != 0)
    }
}
