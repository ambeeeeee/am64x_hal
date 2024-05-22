#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_240` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_240` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec>;
#[doc = "Field `PI_TSR_F1` reader - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 1."]
pub type PiTsrF1R = crate::FieldReader;
#[doc = "Field `PI_TSR_F1` writer - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 1."]
pub type PiTsrF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRD_F1` reader - 15:8\\]
DRAM tMRD value in cycles for frequency set 1."]
pub type PiTmrdF1R = crate::FieldReader;
#[doc = "Field `PI_TMRD_F1` writer - 15:8\\]
DRAM tMRD value in cycles for frequency set 1."]
pub type PiTmrdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRW_F1` reader - 23:16\\]
DRAM tMRW value in cycles for frequency set 1."]
pub type PiTmrwF1R = crate::FieldReader;
#[doc = "Field `PI_TMRW_F1` writer - 23:16\\]
DRAM tMRW value in cycles for frequency set 1."]
pub type PiTmrwF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMOD_F1` reader - 31:24\\]
DRAM tMOD value in cycles for frequency set 1."]
pub type PiTmodF1R = crate::FieldReader;
#[doc = "Field `PI_TMOD_F1` writer - 31:24\\]
DRAM tMOD value in cycles for frequency set 1."]
pub type PiTmodF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 1."]
    #[inline(always)]
    pub fn pi_tsr_f1(&self) -> PiTsrF1R {
        PiTsrF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tmrd_f1(&self) -> PiTmrdF1R {
        PiTmrdF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tMRW value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tmrw_f1(&self) -> PiTmrwF1R {
        PiTmrwF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tMOD value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tmod_f1(&self) -> PiTmodF1R {
        PiTmodF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tsr_f1(&mut self) -> PiTsrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec> {
        PiTsrF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_f1(&mut self) -> PiTmrdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec> {
        PiTmrdF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tMRW value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrw_f1(&mut self) -> PiTmrwF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec> {
        PiTmrwF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tMOD value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_f1(&mut self) -> PiTmodF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec> {
        PiTmodF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_240\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_240::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_240::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_240::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_240::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_240 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi240Spec {
    const RESET_VALUE: u32 = 0;
}
