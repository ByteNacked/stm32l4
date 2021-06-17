///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Reader of field `FNE`
pub type FNE_R = crate::R<bool, bool>;
///Reader of field `VSYNC`
pub type VSYNC_R = crate::R<bool, bool>;
///Reader of field `HSYNC`
pub type HSYNC_R = crate::R<bool, bool>;
impl R {
    ///Bit 2 - FIFO not empty
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - VSYNC
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - HSYNC
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 0x01) != 0)
    }
}
