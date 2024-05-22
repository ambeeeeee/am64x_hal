#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_67` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_67` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec>;
#[doc = "Field `PI_WDQLVL_PERIODIC` reader - 0:0\\]
Enables periodic write DQ training."]
pub type PiWdqlvlPeriodicR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_PERIODIC` writer - 0:0\\]
Enables periodic write DQ training."]
pub type PiWdqlvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_REQ` reader - 8:8\\]
SW write to initiate Write DQ training request. WRITE-ONLY"]
pub type PiWdqlvlReqR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_REQ` writer - 8:8\\]
SW write to initiate Write DQ training request. WRITE-ONLY"]
pub type PiWdqlvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_CS_SW` reader - 17:16\\]
Write DQ training target chip select."]
pub type PiWdqlvlCsSwR = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_CS_SW` writer - 17:16\\]
Write DQ training target chip select."]
pub type PiWdqlvlCsSwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WDQLVL_CS` reader - 24:24\\]
Write DQ training target chip select."]
pub type PiWdqlvlCsR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_CS` writer - 24:24\\]
Write DQ training target chip select."]
pub type PiWdqlvlCsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables periodic write DQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_periodic(&self) -> PiWdqlvlPeriodicR {
        PiWdqlvlPeriodicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SW write to initiate Write DQ training request. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_wdqlvl_req(&self) -> PiWdqlvlReqR {
        PiWdqlvlReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Write DQ training target chip select."]
    #[inline(always)]
    pub fn pi_wdqlvl_cs_sw(&self) -> PiWdqlvlCsSwR {
        PiWdqlvlCsSwR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Write DQ training target chip select."]
    #[inline(always)]
    pub fn pi_wdqlvl_cs(&self) -> PiWdqlvlCsR {
        PiWdqlvlCsR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables periodic write DQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_periodic(
        &mut self,
    ) -> PiWdqlvlPeriodicW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec> {
        PiWdqlvlPeriodicW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SW write to initiate Write DQ training request. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_req(&mut self) -> PiWdqlvlReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec> {
        PiWdqlvlReqW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Write DQ training target chip select."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_cs_sw(&mut self) -> PiWdqlvlCsSwW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec> {
        PiWdqlvlCsSwW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Write DQ training target chip select."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_cs(&mut self) -> PiWdqlvlCsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec> {
        PiWdqlvlCsW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_67\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_67::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_67::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_67 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi67Spec {
    const RESET_VALUE: u32 = 0;
}
