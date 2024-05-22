#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_315` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_315` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec>;
#[doc = "Field `NO_ZQ_INIT` reader - 0:0\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
pub type NoZqInitR = crate::BitReader;
#[doc = "Field `NO_ZQ_INIT` writer - 0:0\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
pub type NoZqInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZQCS_ROTATE` reader - 8:8\\]
For memories that perform ZQ short commands \\[ZQCS\\], selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For ZQ start/latch memories, this parameter is ignored."]
pub type ZqcsRotateR = crate::BitReader;
#[doc = "Field `ZQCS_ROTATE` writer - 8:8\\]
For memories that perform ZQ short commands \\[ZQCS\\], selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For ZQ start/latch memories, this parameter is ignored."]
pub type ZqcsRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZQ_CAL_START_MAP_0` reader - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=0"]
pub type ZqCalStartMap0R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_START_MAP_0` writer - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=0"]
pub type ZqCalStartMap0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ZQ_CAL_LATCH_MAP_0` reader - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=0"]
pub type ZqCalLatchMap0R = crate::FieldReader;
#[doc = "Field `ZQ_CAL_LATCH_MAP_0` writer - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=0"]
pub type ZqCalLatchMap0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_zq_init(&self) -> NoZqInitR {
        NoZqInitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
For memories that perform ZQ short commands \\[ZQCS\\], selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For ZQ start/latch memories, this parameter is ignored."]
    #[inline(always)]
    pub fn zqcs_rotate(&self) -> ZqcsRotateR {
        ZqcsRotateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=0"]
    #[inline(always)]
    pub fn zq_cal_start_map_0(&self) -> ZqCalStartMap0R {
        ZqCalStartMap0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=0"]
    #[inline(always)]
    pub fn zq_cal_latch_map_0(&self) -> ZqCalLatchMap0R {
        ZqCalLatchMap0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_zq_init(&mut self) -> NoZqInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec> {
        NoZqInitW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
For memories that perform ZQ short commands \\[ZQCS\\], selects whether a ZQCS command will calibrate just one chip select or all chip selects. When rotation is off, all chip selects will be calibrated, requiring a longer time frame, but ZQ calibration will need to be performed less frequently. Set to 1 for rotating CS. For ZQ start/latch memories, this parameter is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn zqcs_rotate(&mut self) -> ZqcsRotateW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec> {
        ZqcsRotateW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands. CS=0"]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_start_map_0(
        &mut self,
    ) -> ZqCalStartMap0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec> {
        ZqCalStartMap0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration latch commands simultaneously on iteration 0 of the ZQ LATCH initialization and periodic command sequences. Clear to all zeros for no ZQ LATCH commands. CS=0"]
    #[inline(always)]
    #[must_use]
    pub fn zq_cal_latch_map_0(
        &mut self,
    ) -> ZqCalLatchMap0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec> {
        ZqCalLatchMap0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_315\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_315::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_315::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_315::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_315::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_315 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl315Spec {
    const RESET_VALUE: u32 = 0;
}
