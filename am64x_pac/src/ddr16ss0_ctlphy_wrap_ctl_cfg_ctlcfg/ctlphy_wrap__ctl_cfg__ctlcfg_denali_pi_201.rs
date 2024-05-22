#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_201` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_201` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec>;
#[doc = "Field `PI_TCAENT_F1` reader - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 1."]
pub type PiTcaentF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TCAENT_F1` writer - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 1."]
pub type PiTcaentF1W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PI_TMRZ_F2` reader - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 2."]
pub type PiTmrzF2R = crate::FieldReader;
#[doc = "Field `PI_TMRZ_F2` writer - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 2."]
pub type PiTmrzF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 1."]
    #[inline(always)]
    pub fn pi_tcaent_f1(&self) -> PiTcaentF1R {
        PiTcaentF1R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 2."]
    #[inline(always)]
    pub fn pi_tmrz_f2(&self) -> PiTmrzF2R {
        PiTmrzF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcaent_f1(&mut self) -> PiTcaentF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec> {
        PiTcaentF1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrz_f2(&mut self) -> PiTmrzF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec> {
        PiTmrzF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_201\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_201::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_201::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_201::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_201::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_201 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi201Spec {
    const RESET_VALUE: u32 = 0;
}
