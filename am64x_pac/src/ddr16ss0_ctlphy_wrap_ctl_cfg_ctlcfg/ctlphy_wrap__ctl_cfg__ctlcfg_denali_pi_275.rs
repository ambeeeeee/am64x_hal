#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_275` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_275` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec>;
#[doc = "Field `PI_TZQLAT_F0` reader - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 0 in cycles."]
pub type PiTzqlatF0R = crate::FieldReader;
#[doc = "Field `PI_TZQLAT_F0` writer - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 0 in cycles."]
pub type PiTzqlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 0 in cycles."]
    #[inline(always)]
    pub fn pi_tzqlat_f0(&self) -> PiTzqlatF0R {
        PiTzqlatF0R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tzqlat_f0(&mut self) -> PiTzqlatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec> {
        PiTzqlatF0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_275\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_275::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_275::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_275::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_275::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_275 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi275Spec {
    const RESET_VALUE: u32 = 0;
}
