#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_207` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_207` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT_F0` reader - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStartPointF0R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT_F0` writer - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStartPointF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT_F0` reader - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStopPointF0R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT_F0` writer - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStopPointF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT_F1` reader - 22:16\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStartPointF1R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_START_POINT_F1` writer - 22:16\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStartPointF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT_F1` reader - 30:24\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStopPointF1R = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STOP_POINT_F1` writer - 30:24\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
pub type PiCalvlVrefInitialStopPointF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_start_point_f0(&self) -> PiCalvlVrefInitialStartPointF0R {
        PiCalvlVrefInitialStartPointF0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_stop_point_f0(&self) -> PiCalvlVrefInitialStopPointF0R {
        PiCalvlVrefInitialStopPointF0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_start_point_f1(&self) -> PiCalvlVrefInitialStartPointF1R {
        PiCalvlVrefInitialStartPointF1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_stop_point_f1(&self) -> PiCalvlVrefInitialStopPointF1R {
        PiCalvlVrefInitialStopPointF1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_start_point_f0(
        &mut self,
    ) -> PiCalvlVrefInitialStartPointF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec> {
        PiCalvlVrefInitialStartPointF0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 0 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_stop_point_f0(
        &mut self,
    ) -> PiCalvlVrefInitialStopPointF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec> {
        PiCalvlVrefInitialStopPointF0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
The start point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_start_point_f1(
        &mut self,
    ) -> PiCalvlVrefInitialStartPointF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec> {
        PiCalvlVrefInitialStartPointF1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
The end point of initial training for the Vref\\[ca\\]
training for frequency set 1 { vrefca_range, vref_ca_setting\\[5:0\\]}."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_stop_point_f1(
        &mut self,
    ) -> PiCalvlVrefInitialStopPointF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec> {
        PiCalvlVrefInitialStopPointF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_207\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_207::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_207::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_207::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_207::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_207 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi207Spec {
    const RESET_VALUE: u32 = 0;
}
