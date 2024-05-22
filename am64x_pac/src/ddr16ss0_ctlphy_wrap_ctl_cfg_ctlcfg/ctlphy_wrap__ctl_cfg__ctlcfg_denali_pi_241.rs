#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_241` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_241` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec>;
#[doc = "Field `PI_TMOD_PAR_F1` reader - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 1."]
pub type PiTmodParF1R = crate::FieldReader;
#[doc = "Field `PI_TMOD_PAR_F1` writer - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 1."]
pub type PiTmodParF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRD_PAR_F1` reader - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 1."]
pub type PiTmrdParF1R = crate::FieldReader;
#[doc = "Field `PI_TMRD_PAR_F1` writer - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 1."]
pub type PiTmrdParF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRTP_F2` reader - 23:16\\]
DRAM tRTP value in cycles for frequency set 2."]
pub type PiTrtpF2R = crate::FieldReader;
#[doc = "Field `PI_TRTP_F2` writer - 23:16\\]
DRAM tRTP value in cycles for frequency set 2."]
pub type PiTrtpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRP_F2` reader - 31:24\\]
DRAM tRP value in cycles for frequency set 2."]
pub type PiTrpF2R = crate::FieldReader;
#[doc = "Field `PI_TRP_F2` writer - 31:24\\]
DRAM tRP value in cycles for frequency set 2."]
pub type PiTrpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tmod_par_f1(&self) -> PiTmodParF1R {
        PiTmodParF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tmrd_par_f1(&self) -> PiTmrdParF1R {
        PiTmrdParF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tRTP value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_trtp_f2(&self) -> PiTrtpF2R {
        PiTrtpF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tRP value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_trp_f2(&self) -> PiTrpF2R {
        PiTrpF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_par_f1(&mut self) -> PiTmodParF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec> {
        PiTmodParF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_par_f1(&mut self) -> PiTmrdParF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec> {
        PiTmrdParF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tRTP value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trtp_f2(&mut self) -> PiTrtpF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec> {
        PiTrtpF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tRP value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trp_f2(&mut self) -> PiTrpF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec> {
        PiTrpF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_241\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_241::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_241::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_241::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_241::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_241 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi241Spec {
    const RESET_VALUE: u32 = 0;
}
