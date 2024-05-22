#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_136` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_136` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec>;
#[doc = "Field `PI_TRST_PWRON` reader - 31:0\\]
Duration of memory reset during power-on initialization."]
pub type PiTrstPwronR = crate::FieldReader<u32>;
#[doc = "Field `PI_TRST_PWRON` writer - 31:0\\]
Duration of memory reset during power-on initialization."]
pub type PiTrstPwronW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Duration of memory reset during power-on initialization."]
    #[inline(always)]
    pub fn pi_trst_pwron(&self) -> PiTrstPwronR {
        PiTrstPwronR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Duration of memory reset during power-on initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trst_pwron(&mut self) -> PiTrstPwronW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec> {
        PiTrstPwronW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_136::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_136::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_136::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_136::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_136 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi136Spec {
    const RESET_VALUE: u32 = 0;
}
