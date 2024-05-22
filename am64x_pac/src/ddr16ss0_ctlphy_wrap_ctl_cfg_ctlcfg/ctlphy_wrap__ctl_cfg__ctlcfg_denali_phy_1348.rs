#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1348` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1348` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec>;
#[doc = "Field `PHY_CAL_SW_CAL_CFG_0` reader - 22:0\\]
defines firmware based pad calibration process"]
pub type PhyCalSwCalCfg0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_SW_CAL_CFG_0` writer - 22:0\\]
defines firmware based pad calibration process"]
pub type PhyCalSwCalCfg0W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PU_MAX_DELTA_0` reader - 29:24\\]
Pad calibration pass1 pu results won't update if out of max delta range ."]
pub type PhyCalRangePass1PuMaxDelta0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_RANGE_PASS1_PU_MAX_DELTA_0` writer - 29:24\\]
Pad calibration pass1 pu results won't update if out of max delta range ."]
pub type PhyCalRangePass1PuMaxDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:22 - 22:0\\]
defines firmware based pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_sw_cal_cfg_0(&self) -> PhyCalSwCalCfg0R {
        PhyCalSwCalCfg0R::new(self.bits & 0x007f_ffff)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Pad calibration pass1 pu results won't update if out of max delta range ."]
    #[inline(always)]
    pub fn phy_cal_range_pass1_pu_max_delta_0(&self) -> PhyCalRangePass1PuMaxDelta0R {
        PhyCalRangePass1PuMaxDelta0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:22 - 22:0\\]
defines firmware based pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_sw_cal_cfg_0(
        &mut self,
    ) -> PhyCalSwCalCfg0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec> {
        PhyCalSwCalCfg0W::new(self, 0)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Pad calibration pass1 pu results won't update if out of max delta range ."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_range_pass1_pu_max_delta_0(
        &mut self,
    ) -> PhyCalRangePass1PuMaxDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec> {
        PhyCalRangePass1PuMaxDelta0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1348\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1348::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1348::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1348::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1348::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1348 to value 0x6300_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1348Spec {
    const RESET_VALUE: u32 = 0x6300_0000;
}
