///Reader of register FCR
pub type R = crate::R<u32, super::FCR>;
///Writer for register FCR
pub type W = crate::W<u32, super::FCR>;
///Register FCR `reset()`'s with value 0
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
///Reader of field `PS`
pub type PS_R = crate::R<u8, u8>;
///Write proxy for field `PS`
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
///Reader of field `DIV`
pub type DIV_R = crate::R<u8, u8>;
///Write proxy for field `DIV`
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
///Reader of field `BLINK`
pub type BLINK_R = crate::R<u8, u8>;
///Write proxy for field `BLINK`
pub struct BLINK_W<'a> {
    w: &'a mut W,
}
impl<'a> BLINK_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
///Reader of field `BLINKF`
pub type BLINKF_R = crate::R<u8, u8>;
///Write proxy for field `BLINKF`
pub struct BLINKF_W<'a> {
    w: &'a mut W,
}
impl<'a> BLINKF_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
///Reader of field `CC`
pub type CC_R = crate::R<u8, u8>;
///Write proxy for field `CC`
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
///Reader of field `DEAD`
pub type DEAD_R = crate::R<u8, u8>;
///Write proxy for field `DEAD`
pub struct DEAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
///Reader of field `PON`
pub type PON_R = crate::R<u8, u8>;
///Write proxy for field `PON`
pub struct PON_W<'a> {
    w: &'a mut W,
}
impl<'a> PON_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
///Reader of field `UDDIE`
pub type UDDIE_R = crate::R<bool, bool>;
///Write proxy for field `UDDIE`
pub struct UDDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDDIE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
///Reader of field `SOFIE`
pub type SOFIE_R = crate::R<bool, bool>;
///Write proxy for field `SOFIE`
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
///Reader of field `HD`
pub type HD_R = crate::R<bool, bool>;
///Write proxy for field `HD`
pub struct HD_W<'a> {
    w: &'a mut W,
}
impl<'a> HD_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    ///Bits 22:25 - PS 16-bit prescaler
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bits 18:21 - DIV clock divider
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    pub fn blinkf(&self) -> BLINKF_R {
        BLINKF_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    ///Bits 10:12 - Contrast control
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    ///Bits 7:9 - Dead time duration
    #[inline(always)]
    pub fn dead(&self) -> DEAD_R {
        DEAD_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    ///Bits 4:6 - Pulse ON duration
    #[inline(always)]
    pub fn pon(&self) -> PON_R {
        PON_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 3 - Update display done interrupt enable
    #[inline(always)]
    pub fn uddie(&self) -> UDDIE_R {
        UDDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 1 - Start of frame interrupt enable
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - High drive enable
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bits 22:25 - PS 16-bit prescaler
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    ///Bits 18:21 - DIV clock divider
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    ///Bits 16:17 - Blink mode selection
    #[inline(always)]
    pub fn blink(&mut self) -> BLINK_W {
        BLINK_W { w: self }
    }
    ///Bits 13:15 - Blink frequency selection
    #[inline(always)]
    pub fn blinkf(&mut self) -> BLINKF_W {
        BLINKF_W { w: self }
    }
    ///Bits 10:12 - Contrast control
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
    ///Bits 7:9 - Dead time duration
    #[inline(always)]
    pub fn dead(&mut self) -> DEAD_W {
        DEAD_W { w: self }
    }
    ///Bits 4:6 - Pulse ON duration
    #[inline(always)]
    pub fn pon(&mut self) -> PON_W {
        PON_W { w: self }
    }
    ///Bit 3 - Update display done interrupt enable
    #[inline(always)]
    pub fn uddie(&mut self) -> UDDIE_W {
        UDDIE_W { w: self }
    }
    ///Bit 1 - Start of frame interrupt enable
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    ///Bit 0 - High drive enable
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W {
        HD_W { w: self }
    }
}
