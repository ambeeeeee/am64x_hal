#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_179` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_179` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec>;
#[doc = "Field `PI_TRFC_F2` reader - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 2."]
pub type PiTrfcF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TRFC_F2` writer - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 2."]
pub type PiTrfcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 2."]
    #[inline(always)]
    pub fn pi_trfc_f2(&self) -> PiTrfcF2R {
        PiTrfcF2R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
DRAM tRFC value in memory clocks for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trfc_f2(&mut self) -> PiTrfcF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec> {
        PiTrfcF2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_179\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_179::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_179::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_179::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_179::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_179 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi179Spec {
    const RESET_VALUE: u32 = 0;
}
