#[doc = "Register `CFG_pll14_CAL_STAT` reader"]
pub type R = crate::R<CfgPll14CalStatSpec>;
#[doc = "Register `CFG_pll14_CAL_STAT` writer"]
pub type W = crate::W<CfgPll14CalStatSpec>;
#[doc = "Field `CAL_OUT` reader - 11:0\\]
Output of the calibration block if cal_byp = 1'b0. If cal_byp = 1'b1 it is a buffer version of cal_in\\[11:0\\]. Can be used to read the phase calibration state to for later use as an override value to bypass skew calibration"]
pub type CalOutR = crate::FieldReader<u16>;
#[doc = "Field `CAL_OUT` writer - 11:0\\]
Output of the calibration block if cal_byp = 1'b0. If cal_byp = 1'b1 it is a buffer version of cal_in\\[11:0\\]. Can be used to read the phase calibration state to for later use as an override value to bypass skew calibration"]
pub type CalOutW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LOCK_CNT` reader - 19:16\\]
Reserved for future use"]
pub type LockCntR = crate::FieldReader;
#[doc = "Field `LOCK_CNT` writer - 19:16\\]
Reserved for future use"]
pub type LockCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CAL_LOCK` reader - 31:31\\]
Reserved for future use"]
pub type CalLockR = crate::BitReader;
#[doc = "Field `CAL_LOCK` writer - 31:31\\]
Reserved for future use"]
pub type CalLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Output of the calibration block if cal_byp = 1'b0. If cal_byp = 1'b1 it is a buffer version of cal_in\\[11:0\\]. Can be used to read the phase calibration state to for later use as an override value to bypass skew calibration"]
    #[inline(always)]
    pub fn cal_out(&self) -> CalOutR {
        CalOutR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reserved for future use"]
    #[inline(always)]
    pub fn lock_cnt(&self) -> LockCntR {
        LockCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved for future use"]
    #[inline(always)]
    pub fn cal_lock(&self) -> CalLockR {
        CalLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Output of the calibration block if cal_byp = 1'b0. If cal_byp = 1'b1 it is a buffer version of cal_in\\[11:0\\]. Can be used to read the phase calibration state to for later use as an override value to bypass skew calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal_out(&mut self) -> CalOutW<CfgPll14CalStatSpec> {
        CalOutW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reserved for future use"]
    #[inline(always)]
    #[must_use]
    pub fn lock_cnt(&mut self) -> LockCntW<CfgPll14CalStatSpec> {
        LockCntW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved for future use"]
    #[inline(always)]
    #[must_use]
    pub fn cal_lock(&mut self) -> CalLockW<CfgPll14CalStatSpec> {
        CalLockW::new(self, 31)
    }
}
#[doc = "CFG_pll14_CAL_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll14_cal_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll14_cal_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll14CalStatSpec;
impl crate::RegisterSpec for CfgPll14CalStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll14_cal_stat::R`](R) reader structure"]
impl crate::Readable for CfgPll14CalStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll14_cal_stat::W`](W) writer structure"]
impl crate::Writable for CfgPll14CalStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll14_CAL_STAT to value 0"]
impl crate::Resettable for CfgPll14CalStatSpec {
    const RESET_VALUE: u32 = 0;
}
