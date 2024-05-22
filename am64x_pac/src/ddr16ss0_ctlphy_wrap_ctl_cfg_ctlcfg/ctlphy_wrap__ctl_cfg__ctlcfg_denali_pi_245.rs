#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_245` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_245` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec>;
#[doc = "Field `PI_TSR_F2` reader - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 2."]
pub type PiTsrF2R = crate::FieldReader;
#[doc = "Field `PI_TSR_F2` writer - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 2."]
pub type PiTsrF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRD_F2` reader - 15:8\\]
DRAM tMRD value in cycles for frequency set 2."]
pub type PiTmrdF2R = crate::FieldReader;
#[doc = "Field `PI_TMRD_F2` writer - 15:8\\]
DRAM tMRD value in cycles for frequency set 2."]
pub type PiTmrdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMRW_F2` reader - 23:16\\]
DRAM tMRW value in cycles for frequency set 2."]
pub type PiTmrwF2R = crate::FieldReader;
#[doc = "Field `PI_TMRW_F2` writer - 23:16\\]
DRAM tMRW value in cycles for frequency set 2."]
pub type PiTmrwF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TMOD_F2` reader - 31:24\\]
DRAM tMOD value in cycles for frequency set 2."]
pub type PiTmodF2R = crate::FieldReader;
#[doc = "Field `PI_TMOD_F2` writer - 31:24\\]
DRAM tMOD value in cycles for frequency set 2."]
pub type PiTmodF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 2."]
    #[inline(always)]
    pub fn pi_tsr_f2(&self) -> PiTsrF2R {
        PiTsrF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tmrd_f2(&self) -> PiTmrdF2R {
        PiTmrdF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tMRW value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tmrw_f2(&self) -> PiTmrwF2R {
        PiTmrwF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tMOD value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tmod_f2(&self) -> PiTmodF2R {
        PiTmodF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Min cycles from sref entry to sref exit for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tsr_f2(&mut self) -> PiTsrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec> {
        PiTsrF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tMRD value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrd_f2(&mut self) -> PiTmrdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec> {
        PiTmrdF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tMRW value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrw_f2(&mut self) -> PiTmrwF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec> {
        PiTmrwF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tMOD value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmod_f2(&mut self) -> PiTmodF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec> {
        PiTmodF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_245\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_245::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_245::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_245::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_245::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_245 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi245Spec {
    const RESET_VALUE: u32 = 0;
}
