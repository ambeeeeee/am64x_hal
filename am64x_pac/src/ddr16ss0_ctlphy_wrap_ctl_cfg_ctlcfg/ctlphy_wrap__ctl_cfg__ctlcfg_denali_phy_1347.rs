#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1347` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1347` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec>;
#[doc = "Field `PHY_CAL_TWO_PASS_CFG_0` reader - 24:0\\]
defines cal_en configure in pad calibration process"]
pub type PhyCalTwoPassCfg0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_TWO_PASS_CFG_0` writer - 24:0\\]
defines cal_en configure in pad calibration process"]
pub type PhyCalTwoPassCfg0W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - 24:0\\]
defines cal_en configure in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_two_pass_cfg_0(&self) -> PhyCalTwoPassCfg0R {
        PhyCalTwoPassCfg0R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - 24:0\\]
defines cal_en configure in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_two_pass_cfg_0(
        &mut self,
    ) -> PhyCalTwoPassCfg0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec> {
        PhyCalTwoPassCfg0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1347\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1347::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1347::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1347::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1347::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1347 to value 0x1320_8728"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1347Spec {
    const RESET_VALUE: u32 = 0x1320_8728;
}
