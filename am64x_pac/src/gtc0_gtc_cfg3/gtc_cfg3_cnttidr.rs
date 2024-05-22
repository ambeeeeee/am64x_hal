#[doc = "Register `GTC_CFG3_CNTTIDR` reader"]
pub type R = crate::R<GtcCfg3CnttidrSpec>;
#[doc = "Register `GTC_CFG3_CNTTIDR` writer"]
pub type W = crate::W<GtcCfg3CnttidrSpec>;
#[doc = "Field `CNTTIDR_FRAME0` reader - 3:0\\]
Indicates the features of timer frame0. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame0R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME0` writer - 3:0\\]
Indicates the features of timer frame0. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTIDR_FRAME1` reader - 7:4\\]
Indicates the features of timer frame1. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame1R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME1` writer - 7:4\\]
Indicates the features of timer frame1. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTIDR_FRAME2` reader - 11:8\\]
Indicates the features of timer frame2. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame2R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME2` writer - 11:8\\]
Indicates the features of timer frame2. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTIDR_FRAME3` reader - 15:12\\]
Indicates the features of timer frame3. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame3R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME3` writer - 15:12\\]
Indicates the features of timer frame3. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTIDR_FRAME4` reader - 19:16\\]
Indicates the features of timer frame4. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame4R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME4` writer - 19:16\\]
Indicates the features of timer frame4. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTIDR_FRAME5` reader - 23:20\\]
Indicates the features of timer frame5. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame5R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME5` writer - 23:20\\]
Indicates the features of timer frame5. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTIDR_FRAME6` reader - 27:24\\]
Indicates the features of timer frame6. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame6R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME6` writer - 27:24\\]
Indicates the features of timer frame6. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNTTIDR_FRAME7` reader - 31:28\\]
Indicates the features of timer frame7. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame7R = crate::FieldReader;
#[doc = "Field `CNTTIDR_FRAME7` writer - 31:28\\]
Indicates the features of timer frame7. Each 4 bit field has the following meaning:"]
pub type CnttidrFrame7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the features of timer frame0. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame0(&self) -> CnttidrFrame0R {
        CnttidrFrame0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the features of timer frame1. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame1(&self) -> CnttidrFrame1R {
        CnttidrFrame1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the features of timer frame2. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame2(&self) -> CnttidrFrame2R {
        CnttidrFrame2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates the features of timer frame3. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame3(&self) -> CnttidrFrame3R {
        CnttidrFrame3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the features of timer frame4. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame4(&self) -> CnttidrFrame4R {
        CnttidrFrame4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the features of timer frame5. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame5(&self) -> CnttidrFrame5R {
        CnttidrFrame5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the features of timer frame6. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame6(&self) -> CnttidrFrame6R {
        CnttidrFrame6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the features of timer frame7. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    pub fn cnttidr_frame7(&self) -> CnttidrFrame7R {
        CnttidrFrame7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the features of timer frame0. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame0(&mut self) -> CnttidrFrame0W<GtcCfg3CnttidrSpec> {
        CnttidrFrame0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the features of timer frame1. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame1(&mut self) -> CnttidrFrame1W<GtcCfg3CnttidrSpec> {
        CnttidrFrame1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the features of timer frame2. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame2(&mut self) -> CnttidrFrame2W<GtcCfg3CnttidrSpec> {
        CnttidrFrame2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Indicates the features of timer frame3. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame3(&mut self) -> CnttidrFrame3W<GtcCfg3CnttidrSpec> {
        CnttidrFrame3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Indicates the features of timer frame4. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame4(&mut self) -> CnttidrFrame4W<GtcCfg3CnttidrSpec> {
        CnttidrFrame4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Indicates the features of timer frame5. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame5(&mut self) -> CnttidrFrame5W<GtcCfg3CnttidrSpec> {
        CnttidrFrame5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates the features of timer frame6. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame6(&mut self) -> CnttidrFrame6W<GtcCfg3CnttidrSpec> {
        CnttidrFrame6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates the features of timer frame7. Each 4 bit field has the following meaning:"]
    #[inline(always)]
    #[must_use]
    pub fn cnttidr_frame7(&mut self) -> CnttidrFrame7W<GtcCfg3CnttidrSpec> {
        CnttidrFrame7W::new(self, 28)
    }
}
#[doc = "GTC_CFG3_CNTTIDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg3_cnttidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg3_cnttidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg3CnttidrSpec;
impl crate::RegisterSpec for GtcCfg3CnttidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg3_cnttidr::R`](R) reader structure"]
impl crate::Readable for GtcCfg3CnttidrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg3_cnttidr::W`](W) writer structure"]
impl crate::Writable for GtcCfg3CnttidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG3_CNTTIDR to value 0"]
impl crate::Resettable for GtcCfg3CnttidrSpec {
    const RESET_VALUE: u32 = 0;
}
