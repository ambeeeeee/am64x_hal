#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_237` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_237` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec>;
#[doc = "Field `PI_TRCD_F1` reader - 7:0\\]
DRAM tRCD value in cycles for frequency set 1."]
pub type PiTrcdF1R = crate::FieldReader;
#[doc = "Field `PI_TRCD_F1` writer - 7:0\\]
DRAM tRCD value in cycles for frequency set 1."]
pub type PiTrcdF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TCCD_L_F1` reader - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 1."]
pub type PiTccdLF1R = crate::FieldReader;
#[doc = "Field `PI_TCCD_L_F1` writer - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 1."]
pub type PiTccdLF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TWTR_F1` reader - 21:16\\]
DRAM tWTR value in cycles for frequency set 1."]
pub type PiTwtrF1R = crate::FieldReader;
#[doc = "Field `PI_TWTR_F1` writer - 21:16\\]
DRAM tWTR value in cycles for frequency set 1."]
pub type PiTwtrF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_TWR_F1` reader - 31:24\\]
DRAM tWR value in cycles for frequency set 1."]
pub type PiTwrF1R = crate::FieldReader;
#[doc = "Field `PI_TWR_F1` writer - 31:24\\]
DRAM tWR value in cycles for frequency set 1."]
pub type PiTwrF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tRCD value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_trcd_f1(&self) -> PiTrcdF1R {
        PiTrcdF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_tccd_l_f1(&self) -> PiTccdLF1R {
        PiTccdLF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM tWTR value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_twtr_f1(&self) -> PiTwtrF1R {
        PiTwtrF1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tWR value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_twr_f1(&self) -> PiTwrF1R {
        PiTwrF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tRCD value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trcd_f1(&mut self) -> PiTrcdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec> {
        PiTrcdF1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM CAS-to_CAS value within the same bank group in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tccd_l_f1(&mut self) -> PiTccdLF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec> {
        PiTccdLF1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM tWTR value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twtr_f1(&mut self) -> PiTwtrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec> {
        PiTwtrF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tWR value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_twr_f1(&mut self) -> PiTwrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec> {
        PiTwrF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_237\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_237::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_237::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_237::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_237::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_237 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi237Spec {
    const RESET_VALUE: u32 = 0;
}
