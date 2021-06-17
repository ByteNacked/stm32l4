///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dr: [u8; 4usize],
    ///0x04 - Independent data register
    pub idr: IDR,
    ///0x08 - Control register
    pub cr: CR,
    _reserved3: [u8; 4usize],
    ///0x10 - Initial CRC value
    pub init: INIT,
    ///0x14 - polynomial
    pub pol: POL,
}
impl RegisterBlock {
    ///0x00 - Data register - half-word sized
    #[inline(always)]
    pub fn dr16(&self) -> &DR16 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DR16) }
    }
    ///0x00 - Data register - half-word sized
    #[inline(always)]
    pub fn dr16_mut(&self) -> &mut DR16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DR16) }
    }
    ///0x00 - Data register - byte sized
    #[inline(always)]
    pub fn dr8(&self) -> &DR8 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DR8) }
    }
    ///0x00 - Data register - byte sized
    #[inline(always)]
    pub fn dr8_mut(&self) -> &mut DR8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DR8) }
    }
    ///0x00 - Data register
    #[inline(always)]
    pub fn dr(&self) -> &DR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DR) }
    }
    ///0x00 - Data register
    #[inline(always)]
    pub fn dr_mut(&self) -> &mut DR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DR) }
    }
}
///Data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](dr) module
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
///`read()` method returns [dr::R](dr::R) reader structure
impl crate::Readable for DR {}
///`write(|w| ..)` method takes [dr::W](dr::W) writer structure
impl crate::Writable for DR {}
///Data register
pub mod dr;
///Independent data register
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idr](idr) module
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
///`read()` method returns [idr::R](idr::R) reader structure
impl crate::Readable for IDR {}
///`write(|w| ..)` method takes [idr::W](idr::W) writer structure
impl crate::Writable for IDR {}
///Independent data register
pub mod idr;
///Control register
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
///Control register
pub mod cr;
///Initial CRC value
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [init](init) module
pub type INIT = crate::Reg<u32, _INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT;
///`read()` method returns [init::R](init::R) reader structure
impl crate::Readable for INIT {}
///`write(|w| ..)` method takes [init::W](init::W) writer structure
impl crate::Writable for INIT {}
///Initial CRC value
pub mod init;
///polynomial
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pol](pol) module
pub type POL = crate::Reg<u32, _POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POL;
///`read()` method returns [pol::R](pol::R) reader structure
impl crate::Readable for POL {}
///`write(|w| ..)` method takes [pol::W](pol::W) writer structure
impl crate::Writable for POL {}
///polynomial
pub mod pol;
///Data register - byte sized
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr8](dr8) module
pub type DR8 = crate::Reg<u8, _DR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR8;
///`read()` method returns [dr8::R](dr8::R) reader structure
impl crate::Readable for DR8 {}
///`write(|w| ..)` method takes [dr8::W](dr8::W) writer structure
impl crate::Writable for DR8 {}
///Data register - byte sized
pub mod dr8;
///Data register - half-word sized
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr16](dr16) module
pub type DR16 = crate::Reg<u16, _DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR16;
///`read()` method returns [dr16::R](dr16::R) reader structure
impl crate::Readable for DR16 {}
///`write(|w| ..)` method takes [dr16::W](dr16::W) writer structure
impl crate::Writable for DR16 {}
///Data register - half-word sized
pub mod dr16;
