#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_229` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_229` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec>;
#[doc = "Field `PI_WDQLVL_VREF_DELTA_F2` reader - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 2."]
pub type PiWdqlvlVrefDeltaF2R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_DELTA_F2` writer - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 2."]
pub type PiWdqlvlVrefDeltaF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WDQLVL_EN_F2` reader - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiWdqlvlEnF2R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_EN_F2` writer - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiWdqlvlEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_NTP_TRAIN_EN_F2` reader - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiNtpTrainEnF2R = crate::FieldReader;
#[doc = "Field `PI_NTP_TRAIN_EN_F2` writer - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiNtpTrainEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WDQLVL_CL_F2` reader - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 2."]
pub type PiWdqlvlClF2R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_CL_F2` writer - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 2."]
pub type PiWdqlvlClF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 2."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_delta_f2(&self) -> PiWdqlvlVrefDeltaF2R {
        PiWdqlvlVrefDeltaF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    pub fn pi_wdqlvl_en_f2(&self) -> PiWdqlvlEnF2R {
        PiWdqlvlEnF2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    pub fn pi_ntp_train_en_f2(&self) -> PiNtpTrainEnF2R {
        PiNtpTrainEnF2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 2."]
    #[inline(always)]
    pub fn pi_wdqlvl_cl_f2(&self) -> PiWdqlvlClF2R {
        PiWdqlvlClF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
The delta from the current Write DQ vref adjustment for non-initial wdq training for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_delta_f2(
        &mut self,
    ) -> PiWdqlvlVrefDeltaF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec> {
        PiWdqlvlVrefDeltaF2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Indicates if Write DQ leveling is enabled for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_en_f2(&mut self) -> PiWdqlvlEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec> {
        PiWdqlvlEnF2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Indicates whether the no topology WDQ training is enabled. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ntp_train_en_f2(
        &mut self,
    ) -> PiNtpTrainEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec> {
        PiNtpTrainEnF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
CL when the Read DBI disabled while doing WDQ training for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_cl_f2(&mut self) -> PiWdqlvlClF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec> {
        PiWdqlvlClF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_229\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_229::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_229::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_229::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_229::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_229 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi229Spec {
    const RESET_VALUE: u32 = 0;
}
