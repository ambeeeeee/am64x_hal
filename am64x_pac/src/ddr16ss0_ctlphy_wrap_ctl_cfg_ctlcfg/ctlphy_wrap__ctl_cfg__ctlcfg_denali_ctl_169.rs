#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_169` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_169` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec>;
#[doc = "Field `LP_AUTO_SR_SHORT_IDLE` reader - 11:0\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into the self-refresh short or self-refresh power-down short \\[with or without memory clock gating\\]
low power states."]
pub type LpAutoSrShortIdleR = crate::FieldReader<u16>;
#[doc = "Field `LP_AUTO_SR_SHORT_IDLE` writer - 11:0\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into the self-refresh short or self-refresh power-down short \\[with or without memory clock gating\\]
low power states."]
pub type LpAutoSrShortIdleW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LP_AUTO_SR_LONG_IDLE` reader - 23:16\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long or self-refresh power-down long \\[with or without memory clock gating\\]
low power states."]
pub type LpAutoSrLongIdleR = crate::FieldReader;
#[doc = "Field `LP_AUTO_SR_LONG_IDLE` writer - 23:16\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long or self-refresh power-down long \\[with or without memory clock gating\\]
low power states."]
pub type LpAutoSrLongIdleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AUTO_SR_LONG_MC_GATE_IDLE` reader - 31:24\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating low power states."]
pub type LpAutoSrLongMcGateIdleR = crate::FieldReader;
#[doc = "Field `LP_AUTO_SR_LONG_MC_GATE_IDLE` writer - 31:24\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating low power states."]
pub type LpAutoSrLongMcGateIdleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into the self-refresh short or self-refresh power-down short \\[with or without memory clock gating\\]
low power states."]
    #[inline(always)]
    pub fn lp_auto_sr_short_idle(&self) -> LpAutoSrShortIdleR {
        LpAutoSrShortIdleR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long or self-refresh power-down long \\[with or without memory clock gating\\]
low power states."]
    #[inline(always)]
    pub fn lp_auto_sr_long_idle(&self) -> LpAutoSrLongIdleR {
        LpAutoSrLongIdleR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating low power states."]
    #[inline(always)]
    pub fn lp_auto_sr_long_mc_gate_idle(&self) -> LpAutoSrLongMcGateIdleR {
        LpAutoSrLongMcGateIdleR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into the self-refresh short or self-refresh power-down short \\[with or without memory clock gating\\]
low power states."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_sr_short_idle(
        &mut self,
    ) -> LpAutoSrShortIdleW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec> {
        LpAutoSrShortIdleW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long or self-refresh power-down long \\[with or without memory clock gating\\]
low power states."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_sr_long_idle(
        &mut self,
    ) -> LpAutoSrLongIdleW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec> {
        LpAutoSrLongIdleW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the idle time \\[in long counts\\]
until the controller will automatically issue an entry into the self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating low power states."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_sr_long_mc_gate_idle(
        &mut self,
    ) -> LpAutoSrLongMcGateIdleW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec> {
        LpAutoSrLongMcGateIdleW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_169\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_169::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_169::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_169::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_169::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_169 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl169Spec {
    const RESET_VALUE: u32 = 0;
}
