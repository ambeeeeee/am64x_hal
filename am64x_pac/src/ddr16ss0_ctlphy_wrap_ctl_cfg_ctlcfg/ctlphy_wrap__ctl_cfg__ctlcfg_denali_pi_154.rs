#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_154` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_154` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec>;
#[doc = "Field `PI_PHYMSTR_TYPE` reader - 1:0\\]
Defines how the controller should set the state of DRAM before turning control of the DFI bus over to the PI."]
pub type PiPhymstrTypeR = crate::FieldReader;
#[doc = "Field `PI_PHYMSTR_TYPE` writer - 1:0\\]
Defines how the controller should set the state of DRAM before turning control of the DFI bus over to the PI."]
pub type PiPhymstrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_POWER_REDUC_EN` reader - 16:16\\]
PI Power reduction enable, 1 = enabled."]
pub type PiPowerReducEnR = crate::BitReader;
#[doc = "Field `PI_POWER_REDUC_EN` writer - 16:16\\]
PI Power reduction enable, 1 = enabled."]
pub type PiPowerReducEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines how the controller should set the state of DRAM before turning control of the DFI bus over to the PI."]
    #[inline(always)]
    pub fn pi_phymstr_type(&self) -> PiPhymstrTypeR {
        PiPhymstrTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
PI Power reduction enable, 1 = enabled."]
    #[inline(always)]
    pub fn pi_power_reduc_en(&self) -> PiPowerReducEnR {
        PiPowerReducEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines how the controller should set the state of DRAM before turning control of the DFI bus over to the PI."]
    #[inline(always)]
    #[must_use]
    pub fn pi_phymstr_type(&mut self) -> PiPhymstrTypeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec> {
        PiPhymstrTypeW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
PI Power reduction enable, 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pi_power_reduc_en(
        &mut self,
    ) -> PiPowerReducEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec> {
        PiPowerReducEnW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_154\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_154::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_154::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_154::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_154::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_154 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi154Spec {
    const RESET_VALUE: u32 = 0;
}
