#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_147` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_147` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec>;
#[doc = "Field `PI_MONITOR_CAP_SEL_2` reader - 0:0\\]
Selection of captures for pi_monitor_2."]
pub type PiMonitorCapSel2R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_2` writer - 0:0\\]
Selection of captures for pi_monitor_2."]
pub type PiMonitorCapSel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MONITOR_2` reader - 15:8\\]
Monitor register 2. READ-ONLY."]
pub type PiMonitor2R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_2` writer - 15:8\\]
Monitor register 2. READ-ONLY."]
pub type PiMonitor2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MONITOR_SRC_SEL_3` reader - 19:16\\]
Selection of sources for pi_monitor_3."]
pub type PiMonitorSrcSel3R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_3` writer - 19:16\\]
Selection of sources for pi_monitor_3."]
pub type PiMonitorSrcSel3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_MONITOR_CAP_SEL_3` reader - 24:24\\]
Selection of captures for pi_monitor_3."]
pub type PiMonitorCapSel3R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_3` writer - 24:24\\]
Selection of captures for pi_monitor_3."]
pub type PiMonitorCapSel3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selection of captures for pi_monitor_2."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_2(&self) -> PiMonitorCapSel2R {
        PiMonitorCapSel2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Monitor register 2. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_2(&self) -> PiMonitor2R {
        PiMonitor2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selection of sources for pi_monitor_3."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_3(&self) -> PiMonitorSrcSel3R {
        PiMonitorSrcSel3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Selection of captures for pi_monitor_3."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_3(&self) -> PiMonitorCapSel3R {
        PiMonitorCapSel3R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selection of captures for pi_monitor_2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_2(
        &mut self,
    ) -> PiMonitorCapSel2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec> {
        PiMonitorCapSel2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Monitor register 2. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_2(&mut self) -> PiMonitor2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec> {
        PiMonitor2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Selection of sources for pi_monitor_3."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_3(
        &mut self,
    ) -> PiMonitorSrcSel3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec> {
        PiMonitorSrcSel3W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Selection of captures for pi_monitor_3."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_3(
        &mut self,
    ) -> PiMonitorCapSel3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec> {
        PiMonitorCapSel3W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_147\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_147::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_147::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_147::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_147::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_147 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi147Spec {
    const RESET_VALUE: u32 = 0;
}
