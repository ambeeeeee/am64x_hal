#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1043` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1043` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec>;
#[doc = "Field `PHY_ADR_CALVL_OBS2_2` reader - 31:0\\]
Observation register contains periodic CA training bits for slice 2. READ-ONLY"]
pub type PhyAdrCalvlObs2_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_OBS2_2` writer - 31:0\\]
Observation register contains periodic CA training bits for slice 2. READ-ONLY"]
pub type PhyAdrCalvlObs2_2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Observation register contains periodic CA training bits for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_calvl_obs2_2(&self) -> PhyAdrCalvlObs2_2R {
        PhyAdrCalvlObs2_2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Observation register contains periodic CA training bits for slice 2. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_obs2_2(
        &mut self,
    ) -> PhyAdrCalvlObs2_2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec> {
        PhyAdrCalvlObs2_2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1043\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1043::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1043::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1043::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1043::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1043 to value 0x0256"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1043Spec {
    const RESET_VALUE: u32 = 0x0256;
}
