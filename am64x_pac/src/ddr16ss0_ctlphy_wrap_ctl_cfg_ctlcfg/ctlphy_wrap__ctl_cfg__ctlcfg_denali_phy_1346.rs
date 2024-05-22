#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1346` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1346` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec>;
#[doc = "Field `PHY_CAL_SLOPE_ADJ_PASS2_0` reader - 19:0\\]
defines slope configure for pass2 in pad calibration process"]
pub type PhyCalSlopeAdjPass2_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_SLOPE_ADJ_PASS2_0` writer - 19:0\\]
defines slope configure for pass2 in pad calibration process"]
pub type PhyCalSlopeAdjPass2_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
defines slope configure for pass2 in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_slope_adj_pass2_0(&self) -> PhyCalSlopeAdjPass2_0R {
        PhyCalSlopeAdjPass2_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
defines slope configure for pass2 in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_slope_adj_pass2_0(
        &mut self,
    ) -> PhyCalSlopeAdjPass2_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec> {
        PhyCalSlopeAdjPass2_0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1346\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1346::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1346::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1346::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1346::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1346 to value 0x0026_6272"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1346Spec {
    const RESET_VALUE: u32 = 0x0026_6272;
}
