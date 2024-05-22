#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_235` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_235` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec>;
#[doc = "Field `PI_TSR_F0` reader - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 0."]
pub type PiTsrF0R = crate::FieldReader;
#[doc = "Field `PI_TSR_F0` writer - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 0."]
pub type PiTsrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRD_F0` reader - 15:8\\]
DRAM tMRD value in cycles for frequency set 0."]
pub type PiTmrdF0R = crate::FieldReader;
#[doc = "Field `PI_TMRD_F0` writer - 15:8\\]
DRAM tMRD value in cycles for frequency set 0."]
pub type PiTmrdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRW_F0` reader - 23:16\\]
DRAM tMRW value in cycles for frequency set 0."]
pub type PiTmrwF0R = crate::FieldReader;
#[doc = "Field `PI_TMRW_F0` writer - 23:16\\]
DRAM tMRW value in cycles for frequency set 0."]
pub type PiTmrwF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMOD_F0` reader - 31:24\\]
DRAM tMOD value in cycles for frequency set 0."]
pub type PiTmodF0R = crate::FieldReader;
#[doc = "Field `PI_TMOD_F0` writer - 31:24\\]
DRAM tMOD value in cycles for frequency set 0."]
pub type PiTmodF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 0."]
    #[inline(always)]
    pub fn pi_tsr_f0(&self) -> PiTsrF0R {
        PiTsrF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tmrd_f0(&self) -> PiTmrdF0R {
        PiTmrdF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tMRW value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tmrw_f0(&self) -> PiTmrwF0R {
        PiTmrwF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tMOD value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tmod_f0(&self) -> PiTmodF0R {
        PiTmodF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tsr_f0(&mut self) -> PiTsrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec> {
        PiTsrF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_f0(&mut self) -> PiTmrdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec> {
        PiTmrdF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tMRW value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrw_f0(&mut self) -> PiTmrwF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec> {
        PiTmrwF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tMOD value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_f0(&mut self) -> PiTmodF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec> {
        PiTmodF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_235\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_235::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_235::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_235::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_235::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_235 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi235Spec {
    const RESET_VALUE: u32 = 0;
}
