#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_150` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_150` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec>;
#[doc = "Field `PI_MONITOR_CAP_SEL_6` reader - 0:0\\]
Selection of captures for pi_monitor_6."]
pub type PiMonitorCapSel6R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_6` writer - 0:0\\]
Selection of captures for pi_monitor_6."]
pub type PiMonitorCapSel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MONITOR_6` reader - 15:8\\]
Monitor register 6. READ-ONLY."]
pub type PiMonitor6R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_6` writer - 15:8\\]
Monitor register 6. READ-ONLY."]
pub type PiMonitor6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MONITOR_SRC_SEL_7` reader - 19:16\\]
Selection of sources for pi_monitor_7."]
pub type PiMonitorSrcSel7R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_7` writer - 19:16\\]
Selection of sources for pi_monitor_7."]
pub type PiMonitorSrcSel7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_MONITOR_CAP_SEL_7` reader - 24:24\\]
Selection of captures for pi_monitor_7."]
pub type PiMonitorCapSel7R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_7` writer - 24:24\\]
Selection of captures for pi_monitor_7."]
pub type PiMonitorCapSel7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selection of captures for pi_monitor_6."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_6(&self) -> PiMonitorCapSel6R {
        PiMonitorCapSel6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Monitor register 6. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_6(&self) -> PiMonitor6R {
        PiMonitor6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selection of sources for pi_monitor_7."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_7(&self) -> PiMonitorSrcSel7R {
        PiMonitorSrcSel7R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Selection of captures for pi_monitor_7."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_7(&self) -> PiMonitorCapSel7R {
        PiMonitorCapSel7R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selection of captures for pi_monitor_6."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_6(
        &mut self,
    ) -> PiMonitorCapSel6W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec> {
        PiMonitorCapSel6W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Monitor register 6. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_6(&mut self) -> PiMonitor6W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec> {
        PiMonitor6W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selection of sources for pi_monitor_7."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_7(
        &mut self,
    ) -> PiMonitorSrcSel7W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec> {
        PiMonitorSrcSel7W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Selection of captures for pi_monitor_7."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_7(
        &mut self,
    ) -> PiMonitorCapSel7W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec> {
        PiMonitorCapSel7W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_150\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_150::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_150::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_150::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_150::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_150 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi150Spec {
    const RESET_VALUE: u32 = 0;
}
