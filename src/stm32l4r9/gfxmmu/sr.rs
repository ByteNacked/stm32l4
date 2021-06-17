///Reader of register SR
pub type R = crate::R<u32, super::SR>;
///Reader of field `B0OF`
pub type B0OF_R = crate::R<bool, bool>;
///Reader of field `B1OF`
pub type B1OF_R = crate::R<bool, bool>;
///Reader of field `B2OF`
pub type B2OF_R = crate::R<bool, bool>;
///Reader of field `B3OF`
pub type B3OF_R = crate::R<bool, bool>;
///Reader of field `AMEF`
pub type AMEF_R = crate::R<bool, bool>;
impl R {
    ///Bit 0 - Buffer 0 overflow flag
    #[inline(always)]
    pub fn b0of(&self) -> B0OF_R {
        B0OF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Buffer 1 overflow flag
    #[inline(always)]
    pub fn b1of(&self) -> B1OF_R {
        B1OF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Buffer 2 overflow flag
    #[inline(always)]
    pub fn b2of(&self) -> B2OF_R {
        B2OF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Buffer 3 overflow flag
    #[inline(always)]
    pub fn b3of(&self) -> B3OF_R {
        B3OF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - AHB master error flag
    #[inline(always)]
    pub fn amef(&self) -> AMEF_R {
        AMEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
