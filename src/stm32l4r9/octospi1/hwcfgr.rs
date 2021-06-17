///Reader of register HWCFGR
pub type R = crate::R<u32, super::HWCFGR>;
///Reader of field `AXI`
pub type AXI_R = crate::R<u8, u8>;
///Reader of field `FIFO`
pub type FIFO_R = crate::R<u8, u8>;
///Reader of field `PRES`
pub type PRES_R = crate::R<u8, u8>;
///Reader of field `IDL`
pub type IDL_R = crate::R<u8, u8>;
///Reader of field `MMW`
pub type MMW_R = crate::R<u8, u8>;
///Reader of field `MST`
pub type MST_R = crate::R<u8, u8>;
impl R {
    ///Bits 0:3 - AXI interface
    #[inline(always)]
    pub fn axi(&self) -> AXI_R {
        AXI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:11 - FIFO depth
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    ///Bits 12:19 - Prescaler
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bits 20:23 - ID Length
    #[inline(always)]
    pub fn idl(&self) -> IDL_R {
        IDL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Memory map write
    #[inline(always)]
    pub fn mmw(&self) -> MMW_R {
        MMW_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Master
    #[inline(always)]
    pub fn mst(&self) -> MST_R {
        MST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
