#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_97` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_97` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_0` reader - 9:0\\]
Interim backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlBackStep0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_BACK_STEP_0` writer - 9:0\\]
Interim backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlBackStep0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_0` reader - 25:16\\]
Final backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlFinalStep0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_FINAL_STEP_0` writer - 25:16\\]
Final backup step delay used in gate training algorithm for slice 0."]
pub type PhyGtlvlFinalStep0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Interim backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_back_step_0(&self) -> PhyGtlvlBackStep0R {
        PhyGtlvlBackStep0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Final backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_final_step_0(&self) -> PhyGtlvlFinalStep0R {
        PhyGtlvlFinalStep0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Interim backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_back_step_0(
        &mut self,
    ) -> PhyGtlvlBackStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec> {
        PhyGtlvlBackStep0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Final backup step delay used in gate training algorithm for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_final_step_0(
        &mut self,
    ) -> PhyGtlvlFinalStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec> {
        PhyGtlvlFinalStep0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_97\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_97::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_97::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_97::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_97::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_97 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy97Spec {
    const RESET_VALUE: u32 = 0;
}
