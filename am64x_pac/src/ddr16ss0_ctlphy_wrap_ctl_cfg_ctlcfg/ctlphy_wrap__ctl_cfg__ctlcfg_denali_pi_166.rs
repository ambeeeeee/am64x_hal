#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_166` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_166` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec>;
#[doc = "Field `PI_TSDO_F0` reader - 7:0\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 0, in PI clocks"]
pub type PiTsdoF0R = crate::FieldReader;
#[doc = "Field `PI_TSDO_F0` writer - 7:0\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 0, in PI clocks"]
pub type PiTsdoF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TSDO_F1` reader - 15:8\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 1, in PI clocks"]
pub type PiTsdoF1R = crate::FieldReader;
#[doc = "Field `PI_TSDO_F1` writer - 15:8\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 1, in PI clocks"]
pub type PiTsdoF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TSDO_F2` reader - 23:16\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 2, in PI clocks"]
pub type PiTsdoF2R = crate::FieldReader;
#[doc = "Field `PI_TSDO_F2` writer - 23:16\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 2, in PI clocks"]
pub type PiTsdoF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 0, in PI clocks"]
    #[inline(always)]
    pub fn pi_tsdo_f0(&self) -> PiTsdoF0R {
        PiTsdoF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 1, in PI clocks"]
    #[inline(always)]
    pub fn pi_tsdo_f1(&self) -> PiTsdoF1R {
        PiTsdoF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 2, in PI clocks"]
    #[inline(always)]
    pub fn pi_tsdo_f2(&self) -> PiTsdoF2R {
        PiTsdoF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 0, in PI clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tsdo_f0(&mut self) -> PiTsdoF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec> {
        PiTsdoF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 1, in PI clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tsdo_f1(&mut self) -> PiTsdoF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec> {
        PiTsdoF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The delay from the read preamble training MRS command to the data strobe drive out for frequency set 2, in PI clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tsdo_f2(&mut self) -> PiTsdoF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec> {
        PiTsdoF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_166\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_166::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_166::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_166::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_166::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_166 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi166Spec {
    const RESET_VALUE: u32 = 0;
}
