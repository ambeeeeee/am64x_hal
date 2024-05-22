#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_295` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_295` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec>;
#[doc = "Field `PI_ZQ_CAL_LATCH_MAP_0` reader - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type PiZqCalLatchMap0R = crate::FieldReader;
#[doc = "Field `PI_ZQ_CAL_LATCH_MAP_0` writer - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type PiZqCalLatchMap0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ZQ_CAL_START_MAP_1` reader - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type PiZqCalStartMap1R = crate::FieldReader;
#[doc = "Field `PI_ZQ_CAL_START_MAP_1` writer - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type PiZqCalStartMap1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ZQ_CAL_LATCH_MAP_1` reader - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type PiZqCalLatchMap1R = crate::FieldReader;
#[doc = "Field `PI_ZQ_CAL_LATCH_MAP_1` writer - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
pub type PiZqCalLatchMap1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    pub fn pi_zq_cal_latch_map_0(&self) -> PiZqCalLatchMap0R {
        PiZqCalLatchMap0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    pub fn pi_zq_cal_start_map_1(&self) -> PiZqCalStartMap1R {
        PiZqCalStartMap1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    pub fn pi_zq_cal_latch_map_1(&self) -> PiZqCalLatchMap1R {
        PiZqCalLatchMap1R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_zq_cal_latch_map_0(
        &mut self,
    ) -> PiZqCalLatchMap0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec> {
        PiZqCalLatchMap0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 1 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_zq_cal_start_map_1(
        &mut self,
    ) -> PiZqCalStartMap1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec> {
        PiZqCalStartMap1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 1 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_zq_cal_latch_map_1(
        &mut self,
    ) -> PiZqCalLatchMap1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec> {
        PiZqCalLatchMap1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_295\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_295::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_295::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_295::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_295::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_295 to value 0x0002_0201"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi295Spec {
    const RESET_VALUE: u32 = 0x0002_0201;
}
