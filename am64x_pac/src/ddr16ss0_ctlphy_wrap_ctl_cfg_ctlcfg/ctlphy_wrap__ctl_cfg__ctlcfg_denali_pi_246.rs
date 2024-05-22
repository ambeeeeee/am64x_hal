#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_246` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_246` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec>;
#[doc = "Field `PI_TMOD_PAR_F2` reader - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 2."]
pub type PiTmodParF2R = crate::FieldReader;
#[doc = "Field `PI_TMOD_PAR_F2` writer - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 2."]
pub type PiTmodParF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRD_PAR_F2` reader - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 2."]
pub type PiTmrdParF2R = crate::FieldReader;
#[doc = "Field `PI_TMRD_PAR_F2` writer - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 2."]
pub type PiTmrdParF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tmod_par_f2(&self) -> PiTmodParF2R {
        PiTmodParF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tmrd_par_f2(&self) -> PiTmrdParF2R {
        PiTmrdParF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_par_f2(&mut self) -> PiTmodParF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec> {
        PiTmodParF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_par_f2(&mut self) -> PiTmrdParF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec> {
        PiTmrdParF2W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_246\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_246::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_246::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_246::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_246::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_246 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi246Spec {
    const RESET_VALUE: u32 = 0;
}
