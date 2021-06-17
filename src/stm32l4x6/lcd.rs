///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - frame control register
    pub fcr: FCR,
    ///0x08 - status register
    pub sr: SR,
    ///0x0c - clear register
    pub clr: CLR,
    _reserved4: [u8; 4usize],
    ///0x14 - display memory
    pub ram_com0: RAM_COM0,
    _reserved5: [u8; 4usize],
    ///0x1c - display memory
    pub ram_com1: RAM_COM1,
    _reserved6: [u8; 4usize],
    ///0x24 - display memory
    pub ram_com2: RAM_COM2,
    _reserved7: [u8; 4usize],
    ///0x2c - display memory
    pub ram_com3: RAM_COM3,
    _reserved8: [u8; 4usize],
    ///0x34 - display memory
    pub ram_com4: RAM_COM4,
    _reserved9: [u8; 4usize],
    ///0x3c - display memory
    pub ram_com5: RAM_COM5,
    _reserved10: [u8; 4usize],
    ///0x44 - display memory
    pub ram_com6: RAM_COM6,
    _reserved11: [u8; 4usize],
    ///0x4c - display memory
    pub ram_com7: RAM_COM7,
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](cr) module
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
///`read()` method returns [cr::R](cr::R) reader structure
impl crate::Readable for CR {}
///`write(|w| ..)` method takes [cr::W](cr::W) writer structure
impl crate::Writable for CR {}
///control register
pub mod cr;
///frame control register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fcr](fcr) module
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
///`read()` method returns [fcr::R](fcr::R) reader structure
impl crate::Readable for FCR {}
///`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure
impl crate::Writable for FCR {}
///frame control register
pub mod fcr;
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](sr) module
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
///`read()` method returns [sr::R](sr::R) reader structure
impl crate::Readable for SR {}
///`write(|w| ..)` method takes [sr::W](sr::W) writer structure
impl crate::Writable for SR {}
///status register
pub mod sr;
///clear register
///
///This register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clr](clr) module
pub type CLR = crate::Reg<u32, _CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR;
///`write(|w| ..)` method takes [clr::W](clr::W) writer structure
impl crate::Writable for CLR {}
///clear register
pub mod clr;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com0](ram_com0) module
pub type RAM_COM0 = crate::Reg<u32, _RAM_COM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM0;
///`read()` method returns [ram_com0::R](ram_com0::R) reader structure
impl crate::Readable for RAM_COM0 {}
///`write(|w| ..)` method takes [ram_com0::W](ram_com0::W) writer structure
impl crate::Writable for RAM_COM0 {}
///display memory
pub mod ram_com0;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com1](ram_com1) module
pub type RAM_COM1 = crate::Reg<u32, _RAM_COM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM1;
///`read()` method returns [ram_com1::R](ram_com1::R) reader structure
impl crate::Readable for RAM_COM1 {}
///`write(|w| ..)` method takes [ram_com1::W](ram_com1::W) writer structure
impl crate::Writable for RAM_COM1 {}
///display memory
pub mod ram_com1;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com2](ram_com2) module
pub type RAM_COM2 = crate::Reg<u32, _RAM_COM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM2;
///`read()` method returns [ram_com2::R](ram_com2::R) reader structure
impl crate::Readable for RAM_COM2 {}
///`write(|w| ..)` method takes [ram_com2::W](ram_com2::W) writer structure
impl crate::Writable for RAM_COM2 {}
///display memory
pub mod ram_com2;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com3](ram_com3) module
pub type RAM_COM3 = crate::Reg<u32, _RAM_COM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM3;
///`read()` method returns [ram_com3::R](ram_com3::R) reader structure
impl crate::Readable for RAM_COM3 {}
///`write(|w| ..)` method takes [ram_com3::W](ram_com3::W) writer structure
impl crate::Writable for RAM_COM3 {}
///display memory
pub mod ram_com3;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com4](ram_com4) module
pub type RAM_COM4 = crate::Reg<u32, _RAM_COM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM4;
///`read()` method returns [ram_com4::R](ram_com4::R) reader structure
impl crate::Readable for RAM_COM4 {}
///`write(|w| ..)` method takes [ram_com4::W](ram_com4::W) writer structure
impl crate::Writable for RAM_COM4 {}
///display memory
pub mod ram_com4;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com5](ram_com5) module
pub type RAM_COM5 = crate::Reg<u32, _RAM_COM5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM5;
///`read()` method returns [ram_com5::R](ram_com5::R) reader structure
impl crate::Readable for RAM_COM5 {}
///`write(|w| ..)` method takes [ram_com5::W](ram_com5::W) writer structure
impl crate::Writable for RAM_COM5 {}
///display memory
pub mod ram_com5;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com6](ram_com6) module
pub type RAM_COM6 = crate::Reg<u32, _RAM_COM6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM6;
///`read()` method returns [ram_com6::R](ram_com6::R) reader structure
impl crate::Readable for RAM_COM6 {}
///`write(|w| ..)` method takes [ram_com6::W](ram_com6::W) writer structure
impl crate::Writable for RAM_COM6 {}
///display memory
pub mod ram_com6;
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com7](ram_com7) module
pub type RAM_COM7 = crate::Reg<u32, _RAM_COM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM_COM7;
///`read()` method returns [ram_com7::R](ram_com7::R) reader structure
impl crate::Readable for RAM_COM7 {}
///`write(|w| ..)` method takes [ram_com7::W](ram_com7::W) writer structure
impl crate::Writable for RAM_COM7 {}
///display memory
pub mod ram_com7;
