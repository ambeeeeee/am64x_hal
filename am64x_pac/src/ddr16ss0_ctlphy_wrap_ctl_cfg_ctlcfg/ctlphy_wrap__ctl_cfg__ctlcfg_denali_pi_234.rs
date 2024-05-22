#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_234` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_234` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec>;
#[doc = "Field `PI_TRAS_MIN_F0` reader - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 0."]
pub type PiTrasMinF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TRAS_MIN_F0` writer - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 0."]
pub type PiTrasMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PI_TDQSCK_MAX_F0` reader - 19:16\\]
Additional delay needed for tDQSCK for frequency set 0."]
pub type PiTdqsckMaxF0R = crate::FieldReader;
#[doc = "Field `PI_TDQSCK_MAX_F0` writer - 19:16\\]
Additional delay needed for tDQSCK for frequency set 0."]
pub type PiTdqsckMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TCCDMW_F0` reader - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 0."]
pub type PiTccdmwF0R = crate::FieldReader;
#[doc = "Field `PI_TCCDMW_F0` writer - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 0."]
pub type PiTccdmwF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tras_min_f0(&self) -> PiTrasMinF0R {
        PiTrasMinF0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Additional delay needed for tDQSCK for frequency set 0."]
    #[inline(always)]
    pub fn pi_tdqsck_max_f0(&self) -> PiTdqsckMaxF0R {
        PiTdqsckMaxF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tccdmw_f0(&self) -> PiTccdmwF0R {
        PiTccdmwF0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tras_min_f0(&mut self) -> PiTrasMinF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec> {
        PiTrasMinF0W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Additional delay needed for tDQSCK for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdqsck_max_f0(&mut self) -> PiTdqsckMaxF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec> {
        PiTdqsckMaxF0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccdmw_f0(&mut self) -> PiTccdmwF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec> {
        PiTccdmwF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_234\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_234::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_234::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_234::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_234::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_234 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi234Spec {
    const RESET_VALUE: u32 = 0;
}
