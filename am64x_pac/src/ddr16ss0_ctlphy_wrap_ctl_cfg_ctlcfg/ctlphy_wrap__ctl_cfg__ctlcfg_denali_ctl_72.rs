#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_72` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_72` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec>;
#[doc = "Field `TRFC_OPT_THRESHOLD` reader - 2:0\\]
Number of clocks before TRFC expires when the refresh task will deassert its request for optimal command to command turn-around timing."]
pub type TrfcOptThresholdR = crate::FieldReader;
#[doc = "Field `TRFC_OPT_THRESHOLD` writer - 2:0\\]
Number of clocks before TRFC expires when the refresh task will deassert its request for optimal command to command turn-around timing."]
pub type TrfcOptThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CS_COMPARISON_FOR_REFRESH_DEPTH` reader - 13:8\\]
Defines the number of entries of the command queue that the refresh logic will consider for sending a refresh command. A non-zero value limits the decode to a subset of the full command pipeline."]
pub type CsComparisonForRefreshDepthR = crate::FieldReader;
#[doc = "Field `CS_COMPARISON_FOR_REFRESH_DEPTH` writer - 13:8\\]
Defines the number of entries of the command queue that the refresh logic will consider for sending a refresh command. A non-zero value limits the decode to a subset of the full command pipeline."]
pub type CsComparisonForRefreshDepthW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRFC_F0` reader - 25:16\\]
DRAM TRFC value in cycles. FC=0"]
pub type TrfcF0R = crate::FieldReader<u16>;
#[doc = "Field `TRFC_F0` writer - 25:16\\]
DRAM TRFC value in cycles. FC=0"]
pub type TrfcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of clocks before TRFC expires when the refresh task will deassert its request for optimal command to command turn-around timing."]
    #[inline(always)]
    pub fn trfc_opt_threshold(&self) -> TrfcOptThresholdR {
        TrfcOptThresholdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the number of entries of the command queue that the refresh logic will consider for sending a refresh command. A non-zero value limits the decode to a subset of the full command pipeline."]
    #[inline(always)]
    pub fn cs_comparison_for_refresh_depth(&self) -> CsComparisonForRefreshDepthR {
        CsComparisonForRefreshDepthR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
DRAM TRFC value in cycles. FC=0"]
    #[inline(always)]
    pub fn trfc_f0(&self) -> TrfcF0R {
        TrfcF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of clocks before TRFC expires when the refresh task will deassert its request for optimal command to command turn-around timing."]
    #[inline(always)]
    #[must_use]
    pub fn trfc_opt_threshold(
        &mut self,
    ) -> TrfcOptThresholdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec> {
        TrfcOptThresholdW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the number of entries of the command queue that the refresh logic will consider for sending a refresh command. A non-zero value limits the decode to a subset of the full command pipeline."]
    #[inline(always)]
    #[must_use]
    pub fn cs_comparison_for_refresh_depth(
        &mut self,
    ) -> CsComparisonForRefreshDepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec> {
        CsComparisonForRefreshDepthW::new(self, 8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
DRAM TRFC value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trfc_f0(&mut self) -> TrfcF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec> {
        TrfcF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_72\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_72::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_72::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_72::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_72::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_72 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl72Spec {
    const RESET_VALUE: u32 = 0;
}
