#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_175` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_175` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec>;
#[doc = "Field `PI_TRFC_F0` reader - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 0."]
pub type PiTrfcF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TRFC_F0` writer - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 0."]
pub type PiTrfcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 0."]
    #[inline(always)]
    pub fn pi_trfc_f0(&self) -> PiTrfcF0R {
        PiTrfcF0R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trfc_f0(&mut self) -> PiTrfcF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec> {
        PiTrfcF0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_175\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_175::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_175::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_175::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_175::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_175 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi175Spec {
    const RESET_VALUE: u32 = 0;
}
