#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_278` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_278` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec>;
#[doc = "Field `FSP_PHY_UPDATE_MRW` reader - 0:0\\]
Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
pub type FspPhyUpdateMrwR = crate::BitReader;
#[doc = "Field `FSP_PHY_UPDATE_MRW` writer - 0:0\\]
Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
pub type FspPhyUpdateMrwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_ALWAYS_WRITE_FSP` reader - 8:8\\]
Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
pub type DfsAlwaysWriteFspR = crate::BitReader;
#[doc = "Field `DFS_ALWAYS_WRITE_FSP` writer - 8:8\\]
Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
pub type DfsAlwaysWriteFspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP_STATUS` reader - 16:16\\]
Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
pub type FspStatusR = crate::BitReader;
#[doc = "Field `FSP_STATUS` writer - 16:16\\]
Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
pub type FspStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSP_OP_CURRENT` reader - 24:24\\]
Reports which FSP set the memory is currently using."]
pub type FspOpCurrentR = crate::BitReader;
#[doc = "Field `FSP_OP_CURRENT` writer - 24:24\\]
Reports which FSP set the memory is currently using."]
pub type FspOpCurrentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
    #[inline(always)]
    pub fn fsp_phy_update_mrw(&self) -> FspPhyUpdateMrwR {
        FspPhyUpdateMrwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
    #[inline(always)]
    pub fn dfs_always_write_fsp(&self) -> DfsAlwaysWriteFspR {
        DfsAlwaysWriteFspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
    #[inline(always)]
    pub fn fsp_status(&self) -> FspStatusR {
        FspStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Reports which FSP set the memory is currently using."]
    #[inline(always)]
    pub fn fsp_op_current(&self) -> FspOpCurrentR {
        FspOpCurrentR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Identifies the logic responsible for updating MR12 and MR14 in memory. Clear to 0 for the controller, or set to 1 for the PHY or PI."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_phy_update_mrw(
        &mut self,
    ) -> FspPhyUpdateMrwW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec> {
        FspPhyUpdateMrwW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Forces all FSP mode registers to be written by the controller during a DFS event. Set to 1 to force the write."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_always_write_fsp(
        &mut self,
    ) -> DfsAlwaysWriteFspW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec> {
        DfsAlwaysWriteFspW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates that a DFS event caused the FSP mode registers to be updated. Value of 1 means that the FSP mode registers were changed."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_status(&mut self) -> FspStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec> {
        FspStatusW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Reports which FSP set the memory is currently using."]
    #[inline(always)]
    #[must_use]
    pub fn fsp_op_current(&mut self) -> FspOpCurrentW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec> {
        FspOpCurrentW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_278\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_278::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_278::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_278::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_278::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_278 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl278Spec {
    const RESET_VALUE: u32 = 0;
}
