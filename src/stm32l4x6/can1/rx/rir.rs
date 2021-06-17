///Reader of register RIR
pub type R = crate::R<u32, super::RIR>;
///Reader of field `STID`
pub type STID_R = crate::R<u16, u16>;
///Reader of field `EXID`
pub type EXID_R = crate::R<u32, u32>;
///IDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    ///0: Standard identifier
    STANDARD = 0,
    ///1: Extended identifier
    EXTENDED = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `IDE`
pub type IDE_R = crate::R<bool, IDE_A>;
impl IDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::STANDARD,
            true => IDE_A::EXTENDED,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IDE_A::STANDARD
    }
    ///Checks if the value of the field is `EXTENDED`
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == IDE_A::EXTENDED
    }
}
///RTR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    ///0: Data frame
    DATA = 0,
    ///1: Remote frame
    REMOTE = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
///Reader of field `RTR`
pub type RTR_R = crate::R<bool, RTR_A>;
impl RTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::DATA,
            true => RTR_A::REMOTE,
        }
    }
    ///Checks if the value of the field is `DATA`
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == RTR_A::DATA
    }
    ///Checks if the value of the field is `REMOTE`
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        *self == RTR_A::REMOTE
    }
}
impl R {
    ///Bits 21:31 - STID
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    ///Bits 3:20 - EXID
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    ///Bit 2 - IDE
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - RTR
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
