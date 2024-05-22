#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_223` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_223` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec>;
#[doc = "Field `PI_WDQLVL_VREF_DELTA_F0` reader - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 0."]
pub type PiWdqlvlVrefDeltaF0R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_DELTA_F0` writer - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 0."]
pub type PiWdqlvlVrefDeltaF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WDQLVL_EN_F0` reader - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiWdqlvlEnF0R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_EN_F0` writer - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiWdqlvlEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_NTP_TRAIN_EN_F0` reader - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiNtpTrainEnF0R = crate::FieldReader;
#[doc = "Field `PI_NTP_TRAIN_EN_F0` writer - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiNtpTrainEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WDQLVL_CL_F0` reader - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 0."]
pub type PiWdqlvlClF0R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_CL_F0` writer - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 0."]
pub type PiWdqlvlClF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 0."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_delta_f0(&self) -> PiWdqlvlVrefDeltaF0R {
        PiWdqlvlVrefDeltaF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    pub fn pi_wdqlvl_en_f0(&self) -> PiWdqlvlEnF0R {
        PiWdqlvlEnF0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    pub fn pi_ntp_train_en_f0(&self) -> PiNtpTrainEnF0R {
        PiNtpTrainEnF0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 0."]
    #[inline(always)]
    pub fn pi_wdqlvl_cl_f0(&self) -> PiWdqlvlClF0R {
        PiWdqlvlClF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_delta_f0(
        &mut self,
    ) -> PiWdqlvlVrefDeltaF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec> {
        PiWdqlvlVrefDeltaF0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_en_f0(&mut self) -> PiWdqlvlEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec> {
        PiWdqlvlEnF0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ntp_train_en_f0(
        &mut self,
    ) -> PiNtpTrainEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec> {
        PiNtpTrainEnF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_cl_f0(&mut self) -> PiWdqlvlClF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec> {
        PiWdqlvlClF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_223\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_223::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_223::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_223::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_223::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_223 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi223Spec {
    const RESET_VALUE: u32 = 0;
}
