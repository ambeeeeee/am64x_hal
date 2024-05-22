#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_163` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_163` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec>;
#[doc = "Field `PI_MASK_INIT_COMPLETE` reader - 0:0\\]
Enable the masking of the dfi_init_complete signal back to the controller, 1: mask."]
pub type PiMaskInitCompleteR = crate::BitReader;
#[doc = "Field `PI_MASK_INIT_COMPLETE` writer - 0:0\\]
Enable the masking of the dfi_init_complete signal back to the controller, 1: mask."]
pub type PiMaskInitCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DISCONNECT_MC` reader - 8:8\\]
PI disconnects the controller from the PHY, 1: disconnect"]
pub type PiDisconnectMcR = crate::BitReader;
#[doc = "Field `PI_DISCONNECT_MC` writer - 8:8\\]
PI disconnects the controller from the PHY, 1: disconnect"]
pub type PiDisconnectMcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DISABLE_PHYMSTR_REQ` reader - 16:16\\]
PI mask dfi_phymstr_req to the controller and get dfi bus without dfi_phymstr_ack, 1: disconnect"]
pub type PiDisablePhymstrReqR = crate::BitReader;
#[doc = "Field `PI_DISABLE_PHYMSTR_REQ` writer - 16:16\\]
PI mask dfi_phymstr_req to the controller and get dfi bus without dfi_phymstr_ack, 1: disconnect"]
pub type PiDisablePhymstrReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_NOTCARE_MC_INIT_START` reader - 24:24\\]
Defines whether PI waits for the controller to initiate dfi_init_start before PI memory initialization, 1: wait for dfi_init_start"]
pub type PiNotcareMcInitStartR = crate::BitReader;
#[doc = "Field `PI_NOTCARE_MC_INIT_START` writer - 24:24\\]
Defines whether PI waits for the controller to initiate dfi_init_start before PI memory initialization, 1: wait for dfi_init_start"]
pub type PiNotcareMcInitStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable the masking of the dfi_init_complete signal back to the controller, 1: mask."]
    #[inline(always)]
    pub fn pi_mask_init_complete(&self) -> PiMaskInitCompleteR {
        PiMaskInitCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PI disconnects the controller from the PHY, 1: disconnect"]
    #[inline(always)]
    pub fn pi_disconnect_mc(&self) -> PiDisconnectMcR {
        PiDisconnectMcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
PI mask dfi_phymstr_req to the controller and get dfi bus without dfi_phymstr_ack, 1: disconnect"]
    #[inline(always)]
    pub fn pi_disable_phymstr_req(&self) -> PiDisablePhymstrReqR {
        PiDisablePhymstrReqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Defines whether PI waits for the controller to initiate dfi_init_start before PI memory initialization, 1: wait for dfi_init_start"]
    #[inline(always)]
    pub fn pi_notcare_mc_init_start(&self) -> PiNotcareMcInitStartR {
        PiNotcareMcInitStartR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable the masking of the dfi_init_complete signal back to the controller, 1: mask."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mask_init_complete(
        &mut self,
    ) -> PiMaskInitCompleteW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec> {
        PiMaskInitCompleteW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PI disconnects the controller from the PHY, 1: disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn pi_disconnect_mc(&mut self) -> PiDisconnectMcW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec> {
        PiDisconnectMcW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
PI mask dfi_phymstr_req to the controller and get dfi bus without dfi_phymstr_ack, 1: disconnect"]
    #[inline(always)]
    #[must_use]
    pub fn pi_disable_phymstr_req(
        &mut self,
    ) -> PiDisablePhymstrReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec> {
        PiDisablePhymstrReqW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Defines whether PI waits for the controller to initiate dfi_init_start before PI memory initialization, 1: wait for dfi_init_start"]
    #[inline(always)]
    #[must_use]
    pub fn pi_notcare_mc_init_start(
        &mut self,
    ) -> PiNotcareMcInitStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec> {
        PiNotcareMcInitStartW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_163\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_163::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_163::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_163::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_163::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_163 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi163Spec {
    const RESET_VALUE: u32 = 0;
}
