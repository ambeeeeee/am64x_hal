#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_174` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_174` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec>;
#[doc = "Field `LPC_SR_ZQ_EN` reader - 0:0\\]
Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrZqEnR = crate::BitReader;
#[doc = "Field `LPC_SR_ZQ_EN` writer - 0:0\\]
Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
pub type LpcSrZqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDN_SHIFT_DELAY` reader - 16:8\\]
This parameter should be programmed to zero. Manual adjustment of inhibit_pwrdn_shift in memcd_strategy_data_delay."]
pub type PwrdnShiftDelayR = crate::FieldReader<u16>;
#[doc = "Field `PWRDN_SHIFT_DELAY` writer - 16:8\\]
This parameter should be programmed to zero. Manual adjustment of inhibit_pwrdn_shift in memcd_strategy_data_delay."]
pub type PwrdnShiftDelayW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DFS_ENABLE` reader - 24:24\\]
Enable hardware dynamic frequency scaling. Set to 1 to enable."]
pub type DfsEnableR = crate::BitReader;
#[doc = "Field `DFS_ENABLE` writer - 24:24\\]
Enable hardware dynamic frequency scaling. Set to 1 to enable."]
pub type DfsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    pub fn lpc_sr_zq_en(&self) -> LpcSrZqEnR {
        LpcSrZqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
This parameter should be programmed to zero. Manual adjustment of inhibit_pwrdn_shift in memcd_strategy_data_delay."]
    #[inline(always)]
    pub fn pwrdn_shift_delay(&self) -> PwrdnShiftDelayR {
        PwrdnShiftDelayR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable hardware dynamic frequency scaling. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_enable(&self) -> DfsEnableR {
        DfsEnableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable LPC to execute a ZQ calibration on a self-refresh exit sequence. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lpc_sr_zq_en(&mut self) -> LpcSrZqEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec> {
        LpcSrZqEnW::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
This parameter should be programmed to zero. Manual adjustment of inhibit_pwrdn_shift in memcd_strategy_data_delay."]
    #[inline(always)]
    #[must_use]
    pub fn pwrdn_shift_delay(
        &mut self,
    ) -> PwrdnShiftDelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec> {
        PwrdnShiftDelayW::new(self, 8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable hardware dynamic frequency scaling. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_enable(&mut self) -> DfsEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec> {
        DfsEnableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_174\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_174::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_174::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_174::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_174::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_174 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl174Spec {
    const RESET_VALUE: u32 = 0;
}
