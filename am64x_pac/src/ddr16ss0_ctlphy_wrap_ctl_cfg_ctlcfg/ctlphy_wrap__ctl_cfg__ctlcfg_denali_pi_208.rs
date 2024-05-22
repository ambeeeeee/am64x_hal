#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_208` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_208` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT_F2` reader - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStartPointF2R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT_F2` writer - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStartPointF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT_F2` reader - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStopPointF2R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT_F2` writer - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStopPointF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_CALVL_VREF_DELTA_F0` reader - 19:16\\]
The delta fro the current CA vref for non-initial CA training for frequency set 0."]
pub type PiCalvlVrefDeltaF0R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_DELTA_F0` writer - 19:16\\]
The delta fro the current CA vref for non-initial CA training for frequency set 0."]
pub type PiCalvlVrefDeltaF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_CALVL_VREF_DELTA_F1` reader - 27:24\\]
The delta fro the current CA vref for non-initial CA training for frequency set 1."]
pub type PiCalvlVrefDeltaF1R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_DELTA_F1` writer - 27:24\\]
The delta fro the current CA vref for non-initial CA training for frequency set 1."]
pub type PiCalvlVrefDeltaF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_start_point_f2(&self) -> PiCalvlVrefInitialStartPointF2R {
        PiCalvlVrefInitialStartPointF2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_stop_point_f2(&self) -> PiCalvlVrefInitialStopPointF2R {
        PiCalvlVrefInitialStopPointF2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The delta fro the current CA vref for non-initial CA training for frequency set 0."]
    #[inline(always)]
    pub fn pi_calvl_vref_delta_f0(&self) -> PiCalvlVrefDeltaF0R {
        PiCalvlVrefDeltaF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
The delta fro the current CA vref for non-initial CA training for frequency set 1."]
    #[inline(always)]
    pub fn pi_calvl_vref_delta_f1(&self) -> PiCalvlVrefDeltaF1R {
        PiCalvlVrefDeltaF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_start_point_f2(
        &mut self,
    ) -> PiCalvlVrefInitialStartPointF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec> {
        PiCalvlVrefInitialStartPointF2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 2 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_stop_point_f2(
        &mut self,
    ) -> PiCalvlVrefInitialStopPointF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec> {
        PiCalvlVrefInitialStopPointF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The delta fro the current CA vref for non-initial CA training for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_delta_f0(
        &mut self,
    ) -> PiCalvlVrefDeltaF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec> {
        PiCalvlVrefDeltaF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
The delta fro the current CA vref for non-initial CA training for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_delta_f1(
        &mut self,
    ) -> PiCalvlVrefDeltaF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec> {
        PiCalvlVrefDeltaF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_208\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_208::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_208::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_208::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_208::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_208 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi208Spec {
    const RESET_VALUE: u32 = 0;
}
