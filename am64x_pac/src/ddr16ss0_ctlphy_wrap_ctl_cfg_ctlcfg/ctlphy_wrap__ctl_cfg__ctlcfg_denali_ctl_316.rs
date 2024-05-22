#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_316` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_316` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec>;
#[doc = "Field `ZQ_CAL_START_MAP_1` reader - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=1"]
pub type ZqCalStartMap1R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_START_MAP_1` writer - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=1"]
pub type ZqCalStartMap1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ZQ_CAL_LATCH_MAP_1` reader - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=1"]
pub type ZqCalLatchMap1R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_LATCH_MAP_1` writer - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=1"]
pub type ZqCalLatchMap1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANK_DIFF_0` reader - 17:16\\]
Encoded number of banks on the DRAM\\[s\\]."]
pub type BankDiff0R = crate::FieldReader;
#[doc = "Field `BANK_DIFF_0` writer - 17:16\\]
Encoded number of banks on the DRAM\\[s\\]."]
pub type BankDiff0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BANK_DIFF_1` reader - 25:24\\]
Encoded number of banks on the DRAM\\[s\\]."]
pub type BankDiff1R = crate::FieldReader;
#[doc = "Field `BANK_DIFF_1` writer - 25:24\\]
Encoded number of banks on the DRAM\\[s\\]."]
pub type BankDiff1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=1"]
    #[inline(always)]
    pub fn zq_cal_start_map_1(&self) -> ZqCalStartMap1R {
        ZqCalStartMap1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=1"]
    #[inline(always)]
    pub fn zq_cal_latch_map_1(&self) -> ZqCalLatchMap1R {
        ZqCalLatchMap1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Encoded number of banks on the DRAM\\[s\\]."]
    #[inline(always)]
    pub fn bank_diff_0(&self) -> BankDiff0R {
        BankDiff0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Encoded number of banks on the DRAM\\[s\\]."]
    #[inline(always)]
    pub fn bank_diff_1(&self) -> BankDiff1R {
        BankDiff1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=1"]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_start_map_1(
        &mut self,
    ) -> ZqCalStartMap1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec> {
        ZqCalStartMap1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=1"]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_latch_map_1(
        &mut self,
    ) -> ZqCalLatchMap1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec> {
        ZqCalLatchMap1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Encoded number of banks on the DRAM\\[s\\]."]
    #[inline(always)]
    #[must_use]
    pub fn bank_diff_0(&mut self) -> BankDiff0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec> {
        BankDiff0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Encoded number of banks on the DRAM\\[s\\]."]
    #[inline(always)]
    #[must_use]
    pub fn bank_diff_1(&mut self) -> BankDiff1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec> {
        BankDiff1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_316\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_316::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_316::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_316::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_316::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_316 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl316Spec {
    const RESET_VALUE: u32 = 0;
}
