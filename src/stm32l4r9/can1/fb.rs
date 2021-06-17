///Filter bank 0 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fr1](fr1) module
pub type FR1 = crate::Reg<u32, _FR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR1;
///`read()` method returns [fr1::R](fr1::R) reader structure
impl crate::Readable for FR1 {}
///`write(|w| ..)` method takes [fr1::W](fr1::W) writer structure
impl crate::Writable for FR1 {}
///Filter bank 0 register 1
pub mod fr1;
///Filter bank 0 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fr2](fr2) module
pub type FR2 = crate::Reg<u32, _FR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FR2;
///`read()` method returns [fr2::R](fr2::R) reader structure
impl crate::Readable for FR2 {}
///`write(|w| ..)` method takes [fr2::W](fr2::W) writer structure
impl crate::Writable for FR2 {}
///Filter bank 0 register 2
pub mod fr2;
