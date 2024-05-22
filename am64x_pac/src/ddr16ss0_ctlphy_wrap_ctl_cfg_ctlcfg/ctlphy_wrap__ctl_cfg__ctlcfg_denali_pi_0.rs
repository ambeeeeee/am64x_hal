#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_0` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_0` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec>;
#[doc = "Field `PI_START` reader - 0:0\\]
Initiate command processing in the PI. Set to 1 to initiate."]
pub type PiStartR = crate::BitReader;
#[doc = "Field `PI_START` writer - 0:0\\]
Initiate command processing in the PI. Set to 1 to initiate."]
pub type PiStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DRAM_CLASS` reader - 11:8\\]
Defines the memory class for the PI."]
pub type PiDramClassR = crate::FieldReader;
#[doc = "Field `PI_DRAM_CLASS` writer - 11:8\\]
Defines the memory class for the PI."]
pub type PiDramClassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Initiate command processing in the PI. Set to 1 to initiate."]
    #[inline(always)]
    pub fn pi_start(&self) -> PiStartR {
        PiStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the memory class for the PI."]
    #[inline(always)]
    pub fn pi_dram_class(&self) -> PiDramClassR {
        PiDramClassR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Initiate command processing in the PI. Set to 1 to initiate."]
    #[inline(always)]
    #[must_use]
    pub fn pi_start(&mut self) -> PiStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec> {
        PiStartW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the memory class for the PI."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dram_class(&mut self) -> PiDramClassW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec> {
        PiDramClassW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_0::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_0::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_0 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi0Spec {
    const RESET_VALUE: u32 = 0;
}
