#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_66` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_66` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec>;
#[doc = "Field `PI_WDQLVL_ROTATE` reader - 0:0\\]
Enables write DQ training rotate for interval training."]
pub type PiWdqlvlRotateR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_ROTATE` writer - 0:0\\]
Enables write DQ training rotate for interval training."]
pub type PiWdqlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_CS_MAP` reader - 9:8\\]
Map of CS's included in write DQ training sequence."]
pub type PiWdqlvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_CS_MAP` writer - 9:8\\]
Map of CS's included in write DQ training sequence."]
pub type PiWdqlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STEPSIZE` reader - 20:16\\]
Write DQ training vref step size for initial training."]
pub type PiWdqlvlVrefInitialStepsizeR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STEPSIZE` writer - 20:16\\]
Write DQ training vref step size for initial training."]
pub type PiWdqlvlVrefInitialStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_WDQLVL_VREF_NORMAL_STEPSIZE` reader - 28:24\\]
Write DQ training vref step size for post_initial training."]
pub type PiWdqlvlVrefNormalStepsizeR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_NORMAL_STEPSIZE` writer - 28:24\\]
Write DQ training vref step size for post_initial training."]
pub type PiWdqlvlVrefNormalStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables write DQ training rotate for interval training."]
    #[inline(always)]
    pub fn pi_wdqlvl_rotate(&self) -> PiWdqlvlRotateR {
        PiWdqlvlRotateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Map of CS's included in write DQ training sequence."]
    #[inline(always)]
    pub fn pi_wdqlvl_cs_map(&self) -> PiWdqlvlCsMapR {
        PiWdqlvlCsMapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Write DQ training vref step size for initial training."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_stepsize(&self) -> PiWdqlvlVrefInitialStepsizeR {
        PiWdqlvlVrefInitialStepsizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Write DQ training vref step size for post_initial training."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_normal_stepsize(&self) -> PiWdqlvlVrefNormalStepsizeR {
        PiWdqlvlVrefNormalStepsizeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables write DQ training rotate for interval training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_rotate(&mut self) -> PiWdqlvlRotateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec> {
        PiWdqlvlRotateW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Map of CS's included in write DQ training sequence."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_cs_map(&mut self) -> PiWdqlvlCsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec> {
        PiWdqlvlCsMapW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Write DQ training vref step size for initial training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_stepsize(
        &mut self,
    ) -> PiWdqlvlVrefInitialStepsizeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec> {
        PiWdqlvlVrefInitialStepsizeW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Write DQ training vref step size for post_initial training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_normal_stepsize(
        &mut self,
    ) -> PiWdqlvlVrefNormalStepsizeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec> {
        PiWdqlvlVrefNormalStepsizeW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_66\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_66::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_66::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_66::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_66::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_66 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi66Spec {
    const RESET_VALUE: u32 = 0;
}
