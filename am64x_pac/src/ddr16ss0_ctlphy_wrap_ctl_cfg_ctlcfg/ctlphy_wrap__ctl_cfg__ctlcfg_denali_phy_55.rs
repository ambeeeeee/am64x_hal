#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_55` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_55` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_LE_DLY_OBS_0` reader - 9:0\\]
Observation register containing read leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyRdlvlRddqsDqLeDlyObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_LE_DLY_OBS_0` writer - 9:0\\]
Observation register containing read leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyRdlvlRddqsDqLeDlyObs0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_TE_DLY_OBS_0` reader - 25:16\\]
Observation register containing read leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyRdlvlRddqsDqTeDlyObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_TE_DLY_OBS_0` writer - 25:16\\]
Observation register containing read leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
pub type PhyRdlvlRddqsDqTeDlyObs0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Observation register containing read leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_le_dly_obs_0(&self) -> PhyRdlvlRddqsDqLeDlyObs0R {
        PhyRdlvlRddqsDqLeDlyObs0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Observation register containing read leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_te_dly_obs_0(&self) -> PhyRdlvlRddqsDqTeDlyObs0R {
        PhyRdlvlRddqsDqTeDlyObs0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Observation register containing read leveling data window leading edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_le_dly_obs_0(
        &mut self,
    ) -> PhyRdlvlRddqsDqLeDlyObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec> {
        PhyRdlvlRddqsDqLeDlyObs0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Observation register containing read leveling data window trailing edge slave delay setting for slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_te_dly_obs_0(
        &mut self,
    ) -> PhyRdlvlRddqsDqTeDlyObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec> {
        PhyRdlvlRddqsDqTeDlyObs0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_55::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_55::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_55 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy55Spec {
    const RESET_VALUE: u32 = 0;
}
