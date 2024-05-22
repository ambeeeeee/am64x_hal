#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1345` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1345` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec>;
#[doc = "Field `PHY_ADRCTL_PVT_MAP_0` reader - 7:0\\]
defines slope configure in pad calibration process"]
pub type PhyAdrctlPvtMap0R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_PVT_MAP_0` writer - 7:0\\]
defines slope configure in pad calibration process"]
pub type PhyAdrctlPvtMap0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_CAL_SLOPE_ADJ_0` reader - 27:8\\]
defines slope configure in pad calibration process"]
pub type PhyCalSlopeAdj0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_CAL_SLOPE_ADJ_0` writer - 27:8\\]
defines slope configure in pad calibration process"]
pub type PhyCalSlopeAdj0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
defines slope configure in pad calibration process"]
    #[inline(always)]
    pub fn phy_adrctl_pvt_map_0(&self) -> PhyAdrctlPvtMap0R {
        PhyAdrctlPvtMap0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:27 - 27:8\\]
defines slope configure in pad calibration process"]
    #[inline(always)]
    pub fn phy_cal_slope_adj_0(&self) -> PhyCalSlopeAdj0R {
        PhyCalSlopeAdj0R::new((self.bits >> 8) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
defines slope configure in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_pvt_map_0(
        &mut self,
    ) -> PhyAdrctlPvtMap0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec> {
        PhyAdrctlPvtMap0W::new(self, 0)
    }
    #[doc = "Bits 8:27 - 27:8\\]
defines slope configure in pad calibration process"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_slope_adj_0(
        &mut self,
    ) -> PhyCalSlopeAdj0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec> {
        PhyCalSlopeAdj0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1345\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1345::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1345::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1345::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1345::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1345 to value 0x2662_7200"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1345Spec {
    const RESET_VALUE: u32 = 0x2662_7200;
}
