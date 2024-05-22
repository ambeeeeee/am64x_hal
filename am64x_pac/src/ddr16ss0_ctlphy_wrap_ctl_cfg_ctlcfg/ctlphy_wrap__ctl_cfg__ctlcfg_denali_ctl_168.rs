#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_168` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_168` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec>;
#[doc = "Field `LP_AUTO_MEM_GATE_EN` reader - 2:0\\]
Enable memory clock gating when entering a low power state via the auto low power counters. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, and bit \\[2\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
pub type LpAutoMemGateEnR = crate::FieldReader;
#[doc = "Field `LP_AUTO_MEM_GATE_EN` writer - 2:0\\]
Enable memory clock gating when entering a low power state via the auto low power counters. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, and bit \\[2\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
pub type LpAutoMemGateEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AUTO_PD_IDLE` reader - 19:8\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into one of the power-down low power states."]
pub type LpAutoPdIdleR = crate::FieldReader<u16>;
#[doc = "Field `LP_AUTO_PD_IDLE` writer - 19:8\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into one of the power-down low power states."]
pub type LpAutoPdIdleW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable memory clock gating when entering a low power state via the auto low power counters. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, and bit \\[2\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn lp_auto_mem_gate_en(&self) -> LpAutoMemGateEnR {
        LpAutoMemGateEnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into one of the power-down low power states."]
    #[inline(always)]
    pub fn lp_auto_pd_idle(&self) -> LpAutoPdIdleR {
        LpAutoPdIdleR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable memory clock gating when entering a low power state via the auto low power counters. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, and bit \\[2\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_mem_gate_en(
        &mut self,
    ) -> LpAutoMemGateEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec> {
        LpAutoMemGateEnW::new(self, 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Defines the idle time \\[in controller clocks\\]
until the controller will automatically issue an entry into one of the power-down low power states."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_pd_idle(&mut self) -> LpAutoPdIdleW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec> {
        LpAutoPdIdleW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_168::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_168::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_168::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_168::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_168 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl168Spec {
    const RESET_VALUE: u32 = 0;
}
