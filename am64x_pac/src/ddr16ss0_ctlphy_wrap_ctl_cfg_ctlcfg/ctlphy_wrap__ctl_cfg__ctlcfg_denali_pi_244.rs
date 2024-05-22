#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_244` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_244` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec>;
#[doc = "Field `PI_TRAS_MIN_F2` reader - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 2."]
pub type PiTrasMinF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TRAS_MIN_F2` writer - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 2."]
pub type PiTrasMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PI_TDQSCK_MAX_F2` reader - 19:16\\]
Additional delay needed for tDQSCK for frequency set 2."]
pub type PiTdqsckMaxF2R = crate::FieldReader;
#[doc = "Field `PI_TDQSCK_MAX_F2` writer - 19:16\\]
Additional delay needed for tDQSCK for frequency set 2."]
pub type PiTdqsckMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TCCDMW_F2` reader - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 2."]
pub type PiTccdmwF2R = crate::FieldReader;
#[doc = "Field `PI_TCCDMW_F2` writer - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 2."]
pub type PiTccdmwF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tras_min_f2(&self) -> PiTrasMinF2R {
        PiTrasMinF2R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Additional delay needed for tDQSCK for frequency set 2."]
    #[inline(always)]
    pub fn pi_tdqsck_max_f2(&self) -> PiTdqsckMaxF2R {
        PiTdqsckMaxF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tccdmw_f2(&self) -> PiTccdmwF2R {
        PiTccdmwF2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
DRAM tRAS_MIN value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tras_min_f2(&mut self) -> PiTrasMinF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec> {
        PiTrasMinF2W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Additional delay needed for tDQSCK for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdqsck_max_f2(&mut self) -> PiTdqsckMaxF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec> {
        PiTdqsckMaxF2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
LPDDR4 DRAM tCCDMW in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccdmw_f2(&mut self) -> PiTccdmwF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec> {
        PiTccdmwF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_244\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_244::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_244::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_244::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_244::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_244 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi244Spec {
    const RESET_VALUE: u32 = 0;
}
