#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_238` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_238` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec>;
#[doc = "Field `PI_TRAS_MAX_F1` reader - 19:0\\]
DRAM tRAS_MAX value in cycles for frequency set 1."]
pub type PiTrasMaxF1R = crate::FieldReader<u32>;
#[doc = "Field `PI_TRAS_MAX_F1` writer - 19:0\\]
DRAM tRAS_MAX value in cycles for frequency set 1."]
pub type PiTrasMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM tRAS_MAX value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tras_max_f1(&self) -> PiTrasMaxF1R {
        PiTrasMaxF1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM tRAS_MAX value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tras_max_f1(&mut self) -> PiTrasMaxF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec> {
        PiTrasMaxF1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_238\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_238::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_238::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_238::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_238::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_238 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi238Spec {
    const RESET_VALUE: u32 = 0;
}
