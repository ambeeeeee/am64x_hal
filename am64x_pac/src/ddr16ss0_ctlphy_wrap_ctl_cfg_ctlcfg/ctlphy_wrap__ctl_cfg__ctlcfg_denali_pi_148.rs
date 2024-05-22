#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_148` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_148` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec>;
#[doc = "Field `PI_MONITOR_3` reader - 7:0\\]
Monitor register 3. READ-ONLY."]
pub type PiMonitor3R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_3` writer - 7:0\\]
Monitor register 3. READ-ONLY."]
pub type PiMonitor3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_MONITOR_SRC_SEL_4` reader - 11:8\\]
Selection of sources for pi_monitor_4."]
pub type PiMonitorSrcSel4R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_SRC_SEL_4` writer - 11:8\\]
Selection of sources for pi_monitor_4."]
pub type PiMonitorSrcSel4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_MONITOR_CAP_SEL_4` reader - 16:16\\]
Selection of captures for pi_monitor_4."]
pub type PiMonitorCapSel4R = crate::BitReader;
#[doc = "Field `PI_MONITOR_CAP_SEL_4` writer - 16:16\\]
Selection of captures for pi_monitor_4."]
pub type PiMonitorCapSel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MONITOR_4` reader - 31:24\\]
Monitor register 4. READ-ONLY."]
pub type PiMonitor4R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_4` writer - 31:24\\]
Monitor register 4. READ-ONLY."]
pub type PiMonitor4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Monitor register 3. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_3(&self) -> PiMonitor3R {
        PiMonitor3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Selection of sources for pi_monitor_4."]
    #[inline(always)]
    pub fn pi_monitor_src_sel_4(&self) -> PiMonitorSrcSel4R {
        PiMonitorSrcSel4R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Selection of captures for pi_monitor_4."]
    #[inline(always)]
    pub fn pi_monitor_cap_sel_4(&self) -> PiMonitorCapSel4R {
        PiMonitorCapSel4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Monitor register 4. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_4(&self) -> PiMonitor4R {
        PiMonitor4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Monitor register 3. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_3(&mut self) -> PiMonitor3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec> {
        PiMonitor3W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Selection of sources for pi_monitor_4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_src_sel_4(
        &mut self,
    ) -> PiMonitorSrcSel4W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec> {
        PiMonitorSrcSel4W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Selection of captures for pi_monitor_4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_cap_sel_4(
        &mut self,
    ) -> PiMonitorCapSel4W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec> {
        PiMonitorCapSel4W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Monitor register 4. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_4(&mut self) -> PiMonitor4W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec> {
        PiMonitor4W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_148::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_148::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_148::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_148::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_148 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi148Spec {
    const RESET_VALUE: u32 = 0;
}
