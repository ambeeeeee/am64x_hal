#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_146` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_146` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec>;
#[doc = "Field `PI_MONITOR_SRC_SEL_1` reader - 3:0\\]
Selection of sources for pi_monitor_1."]
pub type PiMonitorSrcSel1R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_1` writer - 3:0\\]
Selection of sources for pi_monitor_1."]
pub type PiMonitorSrcSel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_MONITOR_CAP_SEL_1` reader - 8:8\\]
Selection of captures for pi_monitor_1."]
pub type PiMonitorCapSel1R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_1` writer - 8:8\\]
Selection of captures for pi_monitor_1."]
pub type PiMonitorCapSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MONITOR_1` reader - 23:16\\]
Monitor register 1. READ-ONLY."]
pub type PiMonitor1R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_1` writer - 23:16\\]
Monitor register 1. READ-ONLY."]
pub type PiMonitor1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MONITOR_SRC_SEL_2` reader - 27:24\\]
Selection of sources for pi_monitor_2."]
pub type PiMonitorSrcSel2R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_2` writer - 27:24\\]
Selection of sources for pi_monitor_2."]
pub type PiMonitorSrcSel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selection of sources for pi_monitor_1."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_1(&self) -> PiMonitorSrcSel1R {
        PiMonitorSrcSel1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Selection of captures for pi_monitor_1."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_1(&self) -> PiMonitorCapSel1R {
        PiMonitorCapSel1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Monitor register 1. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_1(&self) -> PiMonitor1R {
        PiMonitor1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Selection of sources for pi_monitor_2."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_2(&self) -> PiMonitorSrcSel2R {
        PiMonitorSrcSel2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selection of sources for pi_monitor_1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_1(
        &mut self,
    ) -> PiMonitorSrcSel1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec> {
        PiMonitorSrcSel1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Selection of captures for pi_monitor_1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_1(
        &mut self,
    ) -> PiMonitorCapSel1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec> {
        PiMonitorCapSel1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Monitor register 1. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_1(&mut self) -> PiMonitor1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec> {
        PiMonitor1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Selection of sources for pi_monitor_2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_2(
        &mut self,
    ) -> PiMonitorSrcSel2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec> {
        PiMonitorSrcSel2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_146\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_146::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_146::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_146::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_146::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_146 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi146Spec {
    const RESET_VALUE: u32 = 0;
}
