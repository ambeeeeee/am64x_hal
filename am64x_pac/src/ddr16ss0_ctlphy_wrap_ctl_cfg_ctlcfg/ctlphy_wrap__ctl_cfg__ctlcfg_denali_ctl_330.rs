#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_330` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_330` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec>;
#[doc = "Field `CONTROLLER_BUSY` reader - 0:0\\]
Indicator that the controller is processing a command. Evaluates all ports for outstanding transactions. Value of 1 indicates controller busy. READ-ONLY"]
pub type ControllerBusyR = crate::BitReader;
#[doc = "Field `CONTROLLER_BUSY` writer - 0:0\\]
Indicator that the controller is processing a command. Evaluates all ports for outstanding transactions. Value of 1 indicates controller busy. READ-ONLY"]
pub type ControllerBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLUPD_REQ` reader - 8:8\\]
Assert the DFI controller-initiated update request signal dfi_ctrlupd_req. Set to 1 to trigger. WRITE-ONLY"]
pub type CtrlupdReqR = crate::BitReader;
#[doc = "Field `CTRLUPD_REQ` writer - 8:8\\]
Assert the DFI controller-initiated update request signal dfi_ctrlupd_req. Set to 1 to trigger. WRITE-ONLY"]
pub type CtrlupdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLUPD_REQ_PER_AREF_EN` reader - 16:16\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
pub type CtrlupdReqPerArefEnR = crate::BitReader;
#[doc = "Field `CTRLUPD_REQ_PER_AREF_EN` writer - 16:16\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
pub type CtrlupdReqPerArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLUPD_AREF_HP_ENABLE` reader - 24:24\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every high priority refresh when executing as a subtask request. Set to 1 to enable."]
pub type CtrlupdArefHpEnableR = crate::BitReader;
#[doc = "Field `CTRLUPD_AREF_HP_ENABLE` writer - 24:24\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every high priority refresh when executing as a subtask request. Set to 1 to enable."]
pub type CtrlupdArefHpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicator that the controller is processing a command. Evaluates all ports for outstanding transactions. Value of 1 indicates controller busy. READ-ONLY"]
    #[inline(always)]
    pub fn controller_busy(&self) -> ControllerBusyR {
        ControllerBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Assert the DFI controller-initiated update request signal dfi_ctrlupd_req. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn ctrlupd_req(&self) -> CtrlupdReqR {
        CtrlupdReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
    #[inline(always)]
    pub fn ctrlupd_req_per_aref_en(&self) -> CtrlupdReqPerArefEnR {
        CtrlupdReqPerArefEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every high priority refresh when executing as a subtask request. Set to 1 to enable."]
    #[inline(always)]
    pub fn ctrlupd_aref_hp_enable(&self) -> CtrlupdArefHpEnableR {
        CtrlupdArefHpEnableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicator that the controller is processing a command. Evaluates all ports for outstanding transactions. Value of 1 indicates controller busy. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn controller_busy(&mut self) -> ControllerBusyW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec> {
        ControllerBusyW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Assert the DFI controller-initiated update request signal dfi_ctrlupd_req. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd_req(&mut self) -> CtrlupdReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec> {
        CtrlupdReqW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd_req_per_aref_en(
        &mut self,
    ) -> CtrlupdReqPerArefEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec> {
        CtrlupdReqPerArefEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable an automatic controller-initiated update \\[dfi_ctrlupd_req\\]
after every high priority refresh when executing as a subtask request. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd_aref_hp_enable(
        &mut self,
    ) -> CtrlupdArefHpEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec> {
        CtrlupdArefHpEnableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_330\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_330::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_330::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_330::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_330::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_330 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl330Spec {
    const RESET_VALUE: u32 = 0;
}
