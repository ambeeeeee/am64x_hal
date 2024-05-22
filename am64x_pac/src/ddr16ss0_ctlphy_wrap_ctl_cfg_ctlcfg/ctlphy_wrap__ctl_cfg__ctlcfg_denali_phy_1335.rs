#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1335` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1335` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec>;
#[doc = "Field `PHY_CAL_SAMPLE_WAIT_0` reader - 7:0\\]
Pad calibration state machine wait count in pad clock cycles for block 0."]
pub type PhyCalSampleWait0R = crate::FieldReader;
#[doc = "Field `PHY_CAL_SAMPLE_WAIT_0` writer - 7:0\\]
Pad calibration state machine wait count in pad clock cycles for block 0."]
pub type PhyCalSampleWait0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_LP4_BOOT_CAL_CLK_SELECT_0` reader - 10:8\\]
Pad calibration pad clock frequency select setting for LPDDR4 boot frequency for block 0."]
pub type PhyLp4BootCalClkSelect0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_CAL_CLK_SELECT_0` writer - 10:8\\]
Pad calibration pad clock frequency select setting for LPDDR4 boot frequency for block 0."]
pub type PhyLp4BootCalClkSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Pad calibration state machine wait count in pad clock cycles for block 0."]
    #[inline(always)]
    pub fn phy_cal_sample_wait_0(&self) -> PhyCalSampleWait0R {
        PhyCalSampleWait0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Pad calibration pad clock frequency select setting for LPDDR4 boot frequency for block 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_cal_clk_select_0(&self) -> PhyLp4BootCalClkSelect0R {
        PhyLp4BootCalClkSelect0R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Pad calibration state machine wait count in pad clock cycles for block 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_sample_wait_0(
        &mut self,
    ) -> PhyCalSampleWait0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec> {
        PhyCalSampleWait0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Pad calibration pad clock frequency select setting for LPDDR4 boot frequency for block 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_cal_clk_select_0(
        &mut self,
    ) -> PhyLp4BootCalClkSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec> {
        PhyLp4BootCalClkSelect0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1335\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1335::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1335::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1335::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1335::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1335 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1335Spec {
    const RESET_VALUE: u32 = 0;
}
