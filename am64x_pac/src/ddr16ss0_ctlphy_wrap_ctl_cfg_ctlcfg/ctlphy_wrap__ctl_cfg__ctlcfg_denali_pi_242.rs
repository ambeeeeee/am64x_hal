#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_242` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_242` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec>;
#[doc = "Field `PI_TRCD_F2` reader - 7:0\\]
DRAM tRCD value in cycles for frequency set 2."]
pub type PiTrcdF2R = crate::FieldReader;
#[doc = "Field `PI_TRCD_F2` writer - 7:0\\]
DRAM tRCD value in cycles for frequency set 2."]
pub type PiTrcdF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TCCD_L_F2` reader - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 2."]
pub type PiTccdLF2R = crate::FieldReader;
#[doc = "Field `PI_TCCD_L_F2` writer - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 2."]
pub type PiTccdLF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TWTR_F2` reader - 21:16\\]
DRAM tWTR value in cycles for frequency set 2."]
pub type PiTwtrF2R = crate::FieldReader;
#[doc = "Field `PI_TWTR_F2` writer - 21:16\\]
DRAM tWTR value in cycles for frequency set 2."]
pub type PiTwtrF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TWR_F2` reader - 31:24\\]
DRAM tWR value in cycles for frequency set 2."]
pub type PiTwrF2R = crate::FieldReader;
#[doc = "Field `PI_TWR_F2` writer - 31:24\\]
DRAM tWR value in cycles for frequency set 2."]
pub type PiTwrF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tRCD value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_trcd_f2(&self) -> PiTrcdF2R {
        PiTrcdF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_tccd_l_f2(&self) -> PiTccdLF2R {
        PiTccdLF2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM tWTR value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_twtr_f2(&self) -> PiTwtrF2R {
        PiTwtrF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tWR value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_twr_f2(&self) -> PiTwrF2R {
        PiTwrF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tRCD value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trcd_f2(&mut self) -> PiTrcdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec> {
        PiTrcdF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccd_l_f2(&mut self) -> PiTccdLF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec> {
        PiTccdLF2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM tWTR value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twtr_f2(&mut self) -> PiTwtrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec> {
        PiTwtrF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tWR value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_f2(&mut self) -> PiTwrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec> {
        PiTwrF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_242\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_242::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_242::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_242::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_242::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_242 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi242Spec {
    const RESET_VALUE: u32 = 0;
}
