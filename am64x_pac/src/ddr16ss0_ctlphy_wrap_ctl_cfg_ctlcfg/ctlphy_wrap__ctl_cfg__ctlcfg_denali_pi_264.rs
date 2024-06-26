#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_264` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_264` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec>;
#[doc = "Field `PI_TINIT_F1` reader - 23:0\\]
DRAM tINIT value for frequency set 1 in cycles."]
pub type PiTinitF1R = crate::FieldReader<u32>;
#[doc = "Field `PI_TINIT_F1` writer - 23:0\\]
DRAM tINIT value for frequency set 1 in cycles."]
pub type PiTinitF1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM tINIT value for frequency set 1 in cycles."]
    #[inline(always)]
    pub fn pi_tinit_f1(&self) -> PiTinitF1R {
        PiTinitF1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM tINIT value for frequency set 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tinit_f1(&mut self) -> PiTinitF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec> {
        PiTinitF1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_264\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_264::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_264::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_264::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_264::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_264 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi264Spec {
    const RESET_VALUE: u32 = 0;
}
