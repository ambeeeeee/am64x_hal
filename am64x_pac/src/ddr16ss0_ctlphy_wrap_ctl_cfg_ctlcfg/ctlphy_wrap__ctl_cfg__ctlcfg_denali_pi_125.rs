#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_125` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_125` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec>;
#[doc = "Field `PI_BIST_STAGE_0` reader - 29:0\\]
Sets the programmable algorithm of each stage X when pi_bist_mmode = 'h4."]
pub type PiBistStage0R = crate::FieldReader<u32>;
#[doc = "Field `PI_BIST_STAGE_0` writer - 29:0\\]
Sets the programmable algorithm of each stage X when pi_bist_mmode = 'h4."]
pub type PiBistStage0W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 29:0\\]
Sets the programmable algorithm of each stage X when pi_bist_mmode = 'h4."]
    #[inline(always)]
    pub fn pi_bist_stage_0(&self) -> PiBistStage0R {
        PiBistStage0R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 29:0\\]
Sets the programmable algorithm of each stage X when pi_bist_mmode = 'h4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_stage_0(&mut self) -> PiBistStage0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec> {
        PiBistStage0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_125::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_125::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_125 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi125Spec {
    const RESET_VALUE: u32 = 0;
}
