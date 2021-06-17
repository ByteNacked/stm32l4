///receive FIFO mailbox identifier register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rir](rir) module
pub type RIR = crate::Reg<u32, _RIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIR;
///`read()` method returns [rir::R](rir::R) reader structure
impl crate::Readable for RIR {}
///receive FIFO mailbox identifier register
pub mod rir;
///mailbox data high register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdtr](rdtr) module
pub type RDTR = crate::Reg<u32, _RDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDTR;
///`read()` method returns [rdtr::R](rdtr::R) reader structure
impl crate::Readable for RDTR {}
///mailbox data high register
pub mod rdtr;
///mailbox data high register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdlr](rdlr) module
pub type RDLR = crate::Reg<u32, _RDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDLR;
///`read()` method returns [rdlr::R](rdlr::R) reader structure
impl crate::Readable for RDLR {}
///mailbox data high register
pub mod rdlr;
///receive FIFO mailbox data high register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdhr](rdhr) module
pub type RDHR = crate::Reg<u32, _RDHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDHR;
///`read()` method returns [rdhr::R](rdhr::R) reader structure
impl crate::Readable for RDHR {}
///receive FIFO mailbox data high register
pub mod rdhr;
