#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_149` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_149` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec>;
#[doc = "Field `PI_MONITOR_SRC_SEL_5` reader - 3:0\\]
Selection of sources for pi_monitor_5."]
pub type PiMonitorSrcSel5R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_5` writer - 3:0\\]
Selection of sources for pi_monitor_5."]
pub type PiMonitorSrcSel5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_MONITOR_CAP_SEL_5` reader - 8:8\\]
Selection of captures for pi_monitor_5."]
pub type PiMonitorCapSel5R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_5` writer - 8:8\\]
Selection of captures for pi_monitor_5."]
pub type PiMonitorCapSel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MONITOR_5` reader - 23:16\\]
Monitor register 5. READ-ONLY."]
pub type PiMonitor5R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_5` writer - 23:16\\]
Monitor register 5. READ-ONLY."]
pub type PiMonitor5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MONITOR_SRC_SEL_6` reader - 27:24\\]
Selection of sources for pi_monitor_6."]
pub type PiMonitorSrcSel6R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_6` writer - 27:24\\]
Selection of sources for pi_monitor_6."]
pub type PiMonitorSrcSel6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selection of sources for pi_monitor_5."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_5(&self) -> PiMonitorSrcSel5R {
        PiMonitorSrcSel5R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Selection of captures for pi_monitor_5."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_5(&self) -> PiMonitorCapSel5R {
        PiMonitorCapSel5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Monitor register 5. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_5(&self) -> PiMonitor5R {
        PiMonitor5R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Selection of sources for pi_monitor_6."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_6(&self) -> PiMonitorSrcSel6R {
        PiMonitorSrcSel6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selection of sources for pi_monitor_5."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_5(
        &mut self,
    ) -> PiMonitorSrcSel5W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec> {
        PiMonitorSrcSel5W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Selection of captures for pi_monitor_5."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_5(
        &mut self,
    ) -> PiMonitorCapSel5W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec> {
        PiMonitorCapSel5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Monitor register 5. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_5(&mut self) -> PiMonitor5W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec> {
        PiMonitor5W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Selection of sources for pi_monitor_6."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_6(
        &mut self,
    ) -> PiMonitorSrcSel6W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec> {
        PiMonitorSrcSel6W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_149\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_149::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_149::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_149::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_149::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_149 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi149Spec {
    const RESET_VALUE: u32 = 0;
}
