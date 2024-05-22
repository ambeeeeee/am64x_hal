#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_36` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_36` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec>;
#[doc = "Field `DQS_OSC_BASE_VALUE_1_CS0` reader - 15:0\\]
Base value for device 1 on chip 0. READ-ONLY DEV=1"]
pub type DqsOscBaseValue1Cs0R = crate::FieldReader<u16>;
#[doc = "Field `DQS_OSC_BASE_VALUE_1_CS0` writer - 15:0\\]
Base value for device 1 on chip 0. READ-ONLY DEV=1"]
pub type DqsOscBaseValue1Cs0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DQS_OSC_BASE_VALUE_0_CS1` reader - 31:16\\]
Base value for device 0 on chip 1. READ-ONLY DEV=0"]
pub type DqsOscBaseValue0Cs1R = crate::FieldReader<u16>;
#[doc = "Field `DQS_OSC_BASE_VALUE_0_CS1` writer - 31:16\\]
Base value for device 0 on chip 1. READ-ONLY DEV=0"]
pub type DqsOscBaseValue0Cs1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Base value for device 1 on chip 0. READ-ONLY DEV=1"]
    #[inline(always)]
    pub fn dqs_osc_base_value_1_cs0(&self) -> DqsOscBaseValue1Cs0R {
        DqsOscBaseValue1Cs0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Base value for device 0 on chip 1. READ-ONLY DEV=0"]
    #[inline(always)]
    pub fn dqs_osc_base_value_0_cs1(&self) -> DqsOscBaseValue0Cs1R {
        DqsOscBaseValue0Cs1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Base value for device 1 on chip 0. READ-ONLY DEV=1"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_base_value_1_cs0(
        &mut self,
    ) -> DqsOscBaseValue1Cs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec> {
        DqsOscBaseValue1Cs0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Base value for device 0 on chip 1. READ-ONLY DEV=0"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_base_value_0_cs1(
        &mut self,
    ) -> DqsOscBaseValue0Cs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec> {
        DqsOscBaseValue0Cs1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_36::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_36::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_36 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl36Spec {
    const RESET_VALUE: u32 = 0;
}
