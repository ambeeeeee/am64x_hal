#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_173` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_173` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec>;
#[doc = "Field `LPC_SR_CTRLUPD_EN` reader - 0:0\\]
Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrCtrlupdEnR = crate::BitReader;
#[doc = "Field `LPC_SR_CTRLUPD_EN` writer - 0:0\\]
Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrCtrlupdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPC_SR_PHYUPD_EN` reader - 8:8\\]
Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhyupdEnR = crate::BitReader;
#[doc = "Field `LPC_SR_PHYUPD_EN` writer - 8:8\\]
Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhyupdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPC_SR_PHYMSTR_EN` reader - 16:16\\]
Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhymstrEnR = crate::BitReader;
#[doc = "Field `LPC_SR_PHYMSTR_EN` writer - 16:16\\]
Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrPhymstrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPC_SR_EXIT_CMD_EN` reader - 24:24\\]
Enable LPC to execute any of the commands on self-refresh exit while exiting. Set to 1 to enable."]
pub type LpcSrExitCmdEnR = crate::BitReader;
#[doc = "Field `LPC_SR_EXIT_CMD_EN` writer - 24:24\\]
Enable LPC to execute any of the commands on self-refresh exit while exiting. Set to 1 to enable."]
pub type LpcSrExitCmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_ctrlupd_en(&self) -> LpcSrCtrlupdEnR {
        LpcSrCtrlupdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_phyupd_en(&self) -> LpcSrPhyupdEnR {
        LpcSrPhyupdEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_phymstr_en(&self) -> LpcSrPhymstrEnR {
        LpcSrPhymstrEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable LPC to execute any of the commands on self-refresh exit while exiting. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_exit_cmd_en(&self) -> LpcSrExitCmdEnR {
        LpcSrExitCmdEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable LPC to execute a DFI control update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_ctrlupd_en(
        &mut self,
    ) -> LpcSrCtrlupdEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec> {
        LpcSrCtrlupdEnW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable LPC to execute a DFI PHY update on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_phyupd_en(&mut self) -> LpcSrPhyupdEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec> {
        LpcSrPhyupdEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable LPC to execute a DFI PHY Master request on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_phymstr_en(
        &mut self,
    ) -> LpcSrPhymstrEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec> {
        LpcSrPhymstrEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable LPC to execute any of the commands on self-refresh exit while exiting. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_exit_cmd_en(
        &mut self,
    ) -> LpcSrExitCmdEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec> {
        LpcSrExitCmdEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_173\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_173::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_173::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_173::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_173::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_173 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl173Spec {
    const RESET_VALUE: u32 = 0;
}
