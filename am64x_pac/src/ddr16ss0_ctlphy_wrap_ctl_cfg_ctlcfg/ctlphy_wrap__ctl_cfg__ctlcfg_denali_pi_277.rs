#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_277` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_277` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec>;
#[doc = "Field `PI_TZQLAT_F1` reader - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 1 in cycles."]
pub type PiTzqlatF1R = crate::FieldReader;
#[doc = "Field `PI_TZQLAT_F1` writer - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 1 in cycles."]
pub type PiTzqlatF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 1 in cycles."]
    #[inline(always)]
    pub fn pi_tzqlat_f1(&self) -> PiTzqlatF1R {
        PiTzqlatF1R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Holds the DRAM ZQLAT value for frequency set 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tzqlat_f1(&mut self) -> PiTzqlatF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec> {
        PiTzqlatF1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_277\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_277::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_277::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_277::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_277::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_277 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi277Spec {
    const RESET_VALUE: u32 = 0;
}
