#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_165` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_165` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec>;
#[doc = "Field `PI_TVREF_F1` reader - 15:0\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 1."]
pub type PiTvrefF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_F1` writer - 15:0\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 1."]
pub type PiTvrefF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_TVREF_F2` reader - 31:16\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 2."]
pub type PiTvrefF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_F2` writer - 31:16\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 2."]
pub type PiTvrefF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 1."]
    #[inline(always)]
    pub fn pi_tvref_f1(&self) -> PiTvrefF1R {
        PiTvrefF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 2."]
    #[inline(always)]
    pub fn pi_tvref_f2(&self) -> PiTvrefF2R {
        PiTvrefF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_f1(&mut self) -> PiTvrefF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec> {
        PiTvrefF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Defines the number of cycles that the PI should wait before issuing the next command after a VREF training MRW command for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_f2(&mut self) -> PiTvrefF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec> {
        PiTvrefF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_165\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_165::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_165::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_165::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_165::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_165 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi165Spec {
    const RESET_VALUE: u32 = 0;
}