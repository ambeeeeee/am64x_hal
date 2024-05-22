#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_290` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_290` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec>;
#[doc = "Field `AREF_NORM_THRESHOLD` reader - 4:0\\]
AREF number of pending refreshes until the normal priority request is asserted."]
pub type ArefNormThresholdR = crate::FieldReader;
#[doc = "Field `AREF_NORM_THRESHOLD` writer - 4:0\\]
AREF number of pending refreshes until the normal priority request is asserted."]
pub type ArefNormThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AREF_HIGH_THRESHOLD` reader - 12:8\\]
AREF number of pending refreshes until the high priority request is asserted."]
pub type ArefHighThresholdR = crate::FieldReader;
#[doc = "Field `AREF_HIGH_THRESHOLD` writer - 12:8\\]
AREF number of pending refreshes until the high priority request is asserted."]
pub type ArefHighThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AREF_MAX_DEFICIT` reader - 20:16\\]
AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
pub type ArefMaxDeficitR = crate::FieldReader;
#[doc = "Field `AREF_MAX_DEFICIT` writer - 20:16\\]
AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
pub type ArefMaxDeficitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AREF_MAX_CREDIT` reader - 28:24\\]
AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
pub type ArefMaxCreditR = crate::FieldReader;
#[doc = "Field `AREF_MAX_CREDIT` writer - 28:24\\]
AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
pub type ArefMaxCreditW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
AREF number of pending refreshes until the normal priority request is asserted."]
    #[inline(always)]
    pub fn aref_norm_threshold(&self) -> ArefNormThresholdR {
        ArefNormThresholdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
AREF number of pending refreshes until the high priority request is asserted."]
    #[inline(always)]
    pub fn aref_high_threshold(&self) -> ArefHighThresholdR {
        ArefHighThresholdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
    #[inline(always)]
    pub fn aref_max_deficit(&self) -> ArefMaxDeficitR {
        ArefMaxDeficitR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
    #[inline(always)]
    pub fn aref_max_credit(&self) -> ArefMaxCreditR {
        ArefMaxCreditR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
AREF number of pending refreshes until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn aref_norm_threshold(
        &mut self,
    ) -> ArefNormThresholdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec> {
        ArefNormThresholdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
AREF number of pending refreshes until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn aref_high_threshold(
        &mut self,
    ) -> ArefHighThresholdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec> {
        ArefHighThresholdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
AREF number of pending refreshes until the maximum number of refreshes has been exceeded."]
    #[inline(always)]
    #[must_use]
    pub fn aref_max_deficit(
        &mut self,
    ) -> ArefMaxDeficitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec> {
        ArefMaxDeficitW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
    #[inline(always)]
    #[must_use]
    pub fn aref_max_credit(&mut self) -> ArefMaxCreditW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec> {
        ArefMaxCreditW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_290\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_290::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_290::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_290::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_290::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_290 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl290Spec {
    const RESET_VALUE: u32 = 0;
}
