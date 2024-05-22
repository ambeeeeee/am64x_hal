#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_145` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_145` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec>;
#[doc = "Field `PI_MONITOR_SRC_SEL_0` reader - 11:8\\]
Selection of sources for pi_monitor_0."]
pub type PiMonitorSrcSel0R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_0` writer - 11:8\\]
Selection of sources for pi_monitor_0."]
pub type PiMonitorSrcSel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_MONITOR_CAP_SEL_0` reader - 16:16\\]
Selection of captures for pi_monitor_0."]
pub type PiMonitorCapSel0R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_0` writer - 16:16\\]
Selection of captures for pi_monitor_0."]
pub type PiMonitorCapSel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MONITOR_0` reader - 31:24\\]
Monitor register 0. READ-ONLY."]
pub type PiMonitor0R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_0` writer - 31:24\\]
Monitor register 0. READ-ONLY."]
pub type PiMonitor0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:11 - 11:8\\]
Selection of sources for pi_monitor_0."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_0(&self) -> PiMonitorSrcSel0R {
        PiMonitorSrcSel0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Selection of captures for pi_monitor_0."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_0(&self) -> PiMonitorCapSel0R {
        PiMonitorCapSel0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Monitor register 0. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_0(&self) -> PiMonitor0R {
        PiMonitor0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - 11:8\\]
Selection of sources for pi_monitor_0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_0(
        &mut self,
    ) -> PiMonitorSrcSel0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec> {
        PiMonitorSrcSel0W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Selection of captures for pi_monitor_0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_0(
        &mut self,
    ) -> PiMonitorCapSel0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec> {
        PiMonitorCapSel0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Monitor register 0. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_0(&mut self) -> PiMonitor0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec> {
        PiMonitor0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_145\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_145::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_145::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_145::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_145::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_145 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi145Spec {
    const RESET_VALUE: u32 = 0;
}
