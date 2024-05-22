#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_284` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_284` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec>;
#[doc = "Field `PI_VREF_VAL_DEV0_0` reader - 6:0\\]
Defines the range and value for VREF training for DRAM 0 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev0_0R = crate::FieldReader;
#[doc = "Field `PI_VREF_VAL_DEV0_0` writer - 6:0\\]
Defines the range and value for VREF training for DRAM 0 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev0_0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_VREF_VAL_DEV0_1` reader - 14:8\\]
Defines the range and value for VREF training for DRAM 0 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev0_1R = crate::FieldReader;
#[doc = "Field `PI_VREF_VAL_DEV0_1` writer - 14:8\\]
Defines the range and value for VREF training for DRAM 0 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev0_1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_VREF_VAL_DEV1_0` reader - 22:16\\]
Defines the range and value for VREF training for DRAM 1 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev1_0R = crate::FieldReader;
#[doc = "Field `PI_VREF_VAL_DEV1_0` writer - 22:16\\]
Defines the range and value for VREF training for DRAM 1 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev1_0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_VREF_VAL_DEV1_1` reader - 30:24\\]
Defines the range and value for VREF training for DRAM 1 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev1_1R = crate::FieldReader;
#[doc = "Field `PI_VREF_VAL_DEV1_1` writer - 30:24\\]
Defines the range and value for VREF training for DRAM 1 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
pub type PiVrefValDev1_1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Defines the range and value for VREF training for DRAM 0 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    pub fn pi_vref_val_dev0_0(&self) -> PiVrefValDev0_0R {
        PiVrefValDev0_0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Defines the range and value for VREF training for DRAM 0 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    pub fn pi_vref_val_dev0_1(&self) -> PiVrefValDev0_1R {
        PiVrefValDev0_1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Defines the range and value for VREF training for DRAM 1 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    pub fn pi_vref_val_dev1_0(&self) -> PiVrefValDev1_0R {
        PiVrefValDev1_0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Defines the range and value for VREF training for DRAM 1 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    pub fn pi_vref_val_dev1_1(&self) -> PiVrefValDev1_1R {
        PiVrefValDev1_1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Defines the range and value for VREF training for DRAM 0 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_val_dev0_0(
        &mut self,
    ) -> PiVrefValDev0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec> {
        PiVrefValDev0_0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Defines the range and value for VREF training for DRAM 0 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_val_dev0_1(
        &mut self,
    ) -> PiVrefValDev0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec> {
        PiVrefValDev0_1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Defines the range and value for VREF training for DRAM 1 for CS 0. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_val_dev1_0(
        &mut self,
    ) -> PiVrefValDev1_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec> {
        PiVrefValDev1_0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Defines the range and value for VREF training for DRAM 1 for CS 1. If the PI_VREF_PDA_EN parameter is not set, device 0 values are used for all devices."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_val_dev1_1(
        &mut self,
    ) -> PiVrefValDev1_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec> {
        PiVrefValDev1_1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_284\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_284::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_284::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_284::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_284::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_284 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi284Spec {
    const RESET_VALUE: u32 = 0;
}
