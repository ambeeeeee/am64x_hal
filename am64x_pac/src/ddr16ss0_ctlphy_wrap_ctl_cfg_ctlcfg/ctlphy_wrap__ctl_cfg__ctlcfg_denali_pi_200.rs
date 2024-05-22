#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_200` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_200` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec>;
#[doc = "Field `PI_TCAENT_F0` reader - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 0."]
pub type PiTcaentF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TCAENT_F0` writer - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 0."]
pub type PiTcaentF0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PI_TMRZ_F1` reader - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 1."]
pub type PiTmrzF1R = crate::FieldReader;
#[doc = "Field `PI_TMRZ_F1` writer - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 1."]
pub type PiTmrzF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 0."]
    #[inline(always)]
    pub fn pi_tcaent_f0(&self) -> PiTcaentF0R {
        PiTcaentF0R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 1."]
    #[inline(always)]
    pub fn pi_tmrz_f1(&self) -> PiTmrzF1R {
        PiTmrzF1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Defines the DRAM tCAENT term, in memory clocks for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcaent_f0(&mut self) -> PiTcaentF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec> {
        PiTcaentF0W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrz_f1(&mut self) -> PiTmrzF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec> {
        PiTmrzF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_200\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_200::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_200::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_200::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_200::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_200 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi200Spec {
    const RESET_VALUE: u32 = 0;
}
