#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_265` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_265` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec>;
#[doc = "Field `PI_TINIT3_F1` reader - 23:0\\]
DRAM tINIT3 value for frequency set 1 in cycles."]
pub type PiTinit3F1R = crate::FieldReader<u32>;
#[doc = "Field `PI_TINIT3_F1` writer - 23:0\\]
DRAM tINIT3 value for frequency set 1 in cycles."]
pub type PiTinit3F1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM tINIT3 value for frequency set 1 in cycles."]
    #[inline(always)]
    pub fn pi_tinit3_f1(&self) -> PiTinit3F1R {
        PiTinit3F1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM tINIT3 value for frequency set 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tinit3_f1(&mut self) -> PiTinit3F1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec> {
        PiTinit3F1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_265\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_265::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_265::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_265::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_265::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_265 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi265Spec {
    const RESET_VALUE: u32 = 0;
}
