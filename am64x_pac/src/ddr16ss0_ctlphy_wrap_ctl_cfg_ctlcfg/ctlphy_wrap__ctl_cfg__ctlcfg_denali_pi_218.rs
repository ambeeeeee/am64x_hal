#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_218` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_218` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec>;
#[doc = "Field `PI_TFC_F0` reader - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 0."]
pub type PiTfcF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TFC_F0` writer - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 0."]
pub type PiTfcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TCKEHDQS_F1` reader - 21:16\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 1."]
pub type PiTckehdqsF1R = crate::FieldReader;
#[doc = "Field `PI_TCKEHDQS_F1` writer - 21:16\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 1."]
pub type PiTckehdqsF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 0."]
    #[inline(always)]
    pub fn pi_tfc_f0(&self) -> PiTfcF0R {
        PiTfcF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 1."]
    #[inline(always)]
    pub fn pi_tckehdqs_f1(&self) -> PiTckehdqsF1R {
        PiTckehdqsF1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tfc_f0(&mut self) -> PiTfcF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec> {
        PiTfcF0W::new(self, 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckehdqs_f1(&mut self) -> PiTckehdqsF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec> {
        PiTckehdqsF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_218\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_218::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_218::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_218::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_218::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_218 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi218Spec {
    const RESET_VALUE: u32 = 0;
}
