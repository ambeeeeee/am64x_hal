#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_236` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_236` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec>;
#[doc = "Field `PI_TMOD_PAR_F0` reader - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 0."]
pub type PiTmodParF0R = crate::FieldReader;
#[doc = "Field `PI_TMOD_PAR_F0` writer - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 0."]
pub type PiTmodParF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRD_PAR_F0` reader - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 0."]
pub type PiTmrdParF0R = crate::FieldReader;
#[doc = "Field `PI_TMRD_PAR_F0` writer - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 0."]
pub type PiTmrdParF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRTP_F1` reader - 23:16\\]
DRAM tRTP value in cycles for frequency set 1."]
pub type PiTrtpF1R = crate::FieldReader;
#[doc = "Field `PI_TRTP_F1` writer - 23:16\\]
DRAM tRTP value in cycles for frequency set 1."]
pub type PiTrtpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TRP_F1` reader - 31:24\\]
DRAM tRP value in cycles for frequency set 1."]
pub type PiTrpF1R = crate::FieldReader;
#[doc = "Field `PI_TRP_F1` writer - 31:24\\]
DRAM tRP value in cycles for frequency set 1."]
pub type PiTrpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tmod_par_f0(&self) -> PiTmodParF0R {
        PiTmodParF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tmrd_par_f0(&self) -> PiTmrdParF0R {
        PiTmrdParF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tRTP value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_trtp_f1(&self) -> PiTrtpF1R {
        PiTrtpF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tRP value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_trp_f1(&self) -> PiTrpF1R {
        PiTrpF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tMOD value when CA parity is enabled in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_par_f0(&mut self) -> PiTmodParF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec> {
        PiTmodParF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value when CA parity is enabled in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_par_f0(&mut self) -> PiTmrdParF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec> {
        PiTmrdParF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tRTP value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trtp_f1(&mut self) -> PiTrtpF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec> {
        PiTrtpF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tRP value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trp_f1(&mut self) -> PiTrpF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec> {
        PiTrpF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_236\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_236::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_236::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_236::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_236::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_236 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi236Spec {
    const RESET_VALUE: u32 = 0;
}
