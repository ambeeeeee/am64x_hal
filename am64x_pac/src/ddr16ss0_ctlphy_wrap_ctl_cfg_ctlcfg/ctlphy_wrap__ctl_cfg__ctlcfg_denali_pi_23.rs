#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_23` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_23` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec>;
#[doc = "Field `PI_SRE_PERIOD_EN` reader - 0:0\\]
Enable the self refresh exit triggered periodic leveling."]
pub type PiSrePeriodEnR = crate::BitReader;
#[doc = "Field `PI_SRE_PERIOD_EN` writer - 0:0\\]
Enable the self refresh exit triggered periodic leveling."]
pub type PiSrePeriodEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_MPD_PERIOD_EN` reader - 8:8\\]
Enable the max power saving mode exit triggered periodic leveling."]
pub type PiMpdPeriodEnR = crate::BitReader;
#[doc = "Field `PI_MPD_PERIOD_EN` writer - 8:8\\]
Enable the max power saving mode exit triggered periodic leveling."]
pub type PiMpdPeriodEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DFI40_POLARITY` reader - 16:16\\]
Defines the polarity of the dfi_wrdata_cs_n/dfi_rddata_cs_n signals."]
pub type PiDfi40PolarityR = crate::BitReader;
#[doc = "Field `PI_DFI40_POLARITY` writer - 16:16\\]
Defines the polarity of the dfi_wrdata_cs_n/dfi_rddata_cs_n signals."]
pub type PiDfi40PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WRLVL_REQ` reader - 24:24\\]
User request to initiate write leveling. Set to 1 to trigger. WRITE-ONLY"]
pub type PiWrlvlReqR = crate::BitReader;
#[doc = "Field `PI_WRLVL_REQ` writer - 24:24\\]
User request to initiate write leveling. Set to 1 to trigger. WRITE-ONLY"]
pub type PiWrlvlReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable the self refresh exit triggered periodic leveling."]
    #[inline(always)]
    pub fn pi_sre_period_en(&self) -> PiSrePeriodEnR {
        PiSrePeriodEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable the max power saving mode exit triggered periodic leveling."]
    #[inline(always)]
    pub fn pi_mpd_period_en(&self) -> PiMpdPeriodEnR {
        PiMpdPeriodEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines the polarity of the dfi_wrdata_cs_n/dfi_rddata_cs_n signals."]
    #[inline(always)]
    pub fn pi_dfi40_polarity(&self) -> PiDfi40PolarityR {
        PiDfi40PolarityR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
User request to initiate write leveling. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_wrlvl_req(&self) -> PiWrlvlReqR {
        PiWrlvlReqR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable the self refresh exit triggered periodic leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sre_period_en(&mut self) -> PiSrePeriodEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec> {
        PiSrePeriodEnW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable the max power saving mode exit triggered periodic leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mpd_period_en(&mut self) -> PiMpdPeriodEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec> {
        PiMpdPeriodEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines the polarity of the dfi_wrdata_cs_n/dfi_rddata_cs_n signals."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dfi40_polarity(
        &mut self,
    ) -> PiDfi40PolarityW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec> {
        PiDfi40PolarityW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
User request to initiate write leveling. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_req(&mut self) -> PiWrlvlReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec> {
        PiWrlvlReqW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_23::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_23::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_23 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi23Spec {
    const RESET_VALUE: u32 = 0;
}
