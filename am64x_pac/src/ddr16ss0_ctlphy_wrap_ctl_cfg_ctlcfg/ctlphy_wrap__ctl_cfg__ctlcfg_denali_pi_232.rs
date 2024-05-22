#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_232` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_232` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec>;
#[doc = "Field `PI_TCCD_L_F0` reader - 4:0\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 0."]
pub type PiTccdLF0R = crate::FieldReader;
#[doc = "Field `PI_TCCD_L_F0` writer - 4:0\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 0."]
pub type PiTccdLF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TWTR_F0` reader - 13:8\\]
DRAM tWTR value in cycles for frequency set 0."]
pub type PiTwtrF0R = crate::FieldReader;
#[doc = "Field `PI_TWTR_F0` writer - 13:8\\]
DRAM tWTR value in cycles for frequency set 0."]
pub type PiTwtrF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TWR_F0` reader - 23:16\\]
DRAM tWR value in cycles for frequency set 0."]
pub type PiTwrF0R = crate::FieldReader;
#[doc = "Field `PI_TWR_F0` writer - 23:16\\]
DRAM tWR value in cycles for frequency set 0."]
pub type PiTwrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_tccd_l_f0(&self) -> PiTccdLF0R {
        PiTccdLF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM tWTR value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_twtr_f0(&self) -> PiTwtrF0R {
        PiTwtrF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tWR value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_twr_f0(&self) -> PiTwrF0R {
        PiTwrF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccd_l_f0(&mut self) -> PiTccdLF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec> {
        PiTccdLF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM tWTR value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twtr_f0(&mut self) -> PiTwtrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec> {
        PiTwtrF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tWR value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_f0(&mut self) -> PiTwrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec> {
        PiTwrF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_232\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_232::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_232::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_232::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_232::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_232 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi232Spec {
    const RESET_VALUE: u32 = 0;
}
