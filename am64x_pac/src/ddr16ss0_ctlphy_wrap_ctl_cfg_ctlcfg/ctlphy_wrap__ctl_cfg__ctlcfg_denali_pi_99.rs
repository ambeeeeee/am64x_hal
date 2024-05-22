#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_99` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_99` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec>;
#[doc = "Field `PI_BIST_START_ADDRESS_1` reader - 0:0\\]
Start BIST checking at this address."]
pub type PiBistStartAddress1R = crate::BitReader;
#[doc = "Field `PI_BIST_START_ADDRESS_1` writer - 0:0\\]
Start BIST checking at this address."]
pub type PiBistStartAddress1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MBIST_INIT_PATTERN` reader - 15:8\\]
PI mbist data check, random lfsr pattern mode init pattern seed."]
pub type PiMbistInitPatternR = crate::FieldReader;
#[doc = "Field `PI_MBIST_INIT_PATTERN` writer - 15:8\\]
PI mbist data check, random lfsr pattern mode init pattern seed."]
pub type PiMbistInitPatternW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start BIST checking at this address."]
    #[inline(always)]
    pub fn pi_bist_start_address_1(&self) -> PiBistStartAddress1R {
        PiBistStartAddress1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
PI mbist data check, random lfsr pattern mode init pattern seed."]
    #[inline(always)]
    pub fn pi_mbist_init_pattern(&self) -> PiMbistInitPatternR {
        PiMbistInitPatternR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start BIST checking at this address."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_start_address_1(
        &mut self,
    ) -> PiBistStartAddress1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec> {
        PiBistStartAddress1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
PI mbist data check, random lfsr pattern mode init pattern seed."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mbist_init_pattern(
        &mut self,
    ) -> PiMbistInitPatternW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec> {
        PiMbistInitPatternW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_99::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_99::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_99 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi99Spec {
    const RESET_VALUE: u32 = 0;
}
