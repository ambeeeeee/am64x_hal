#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_86` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_86` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec>;
#[doc = "Field `PBR_CONT_REQ_EN` reader - 0:0\\]
Enables the per-bank refresh continuous request feature. Set to 1 to enable."]
pub type PbrContReqEnR = crate::BitReader;
#[doc = "Field `PBR_CONT_REQ_EN` writer - 0:0\\]
Enables the per-bank refresh continuous request feature. Set to 1 to enable."]
pub type PbrContReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AREF_PBR_CONT_EN_THRESHOLD` reader - 12:8\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be asserted."]
pub type ArefPbrContEnThresholdR = crate::FieldReader;
#[doc = "Field `AREF_PBR_CONT_EN_THRESHOLD` writer - 12:8\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be asserted."]
pub type ArefPbrContEnThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AREF_PBR_CONT_DIS_THRESHOLD` reader - 20:16\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be deasserted."]
pub type ArefPbrContDisThresholdR = crate::FieldReader;
#[doc = "Field `AREF_PBR_CONT_DIS_THRESHOLD` writer - 20:16\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be deasserted."]
pub type ArefPbrContDisThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the per-bank refresh continuous request feature. Set to 1 to enable."]
    #[inline(always)]
    pub fn pbr_cont_req_en(&self) -> PbrContReqEnR {
        PbrContReqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be asserted."]
    #[inline(always)]
    pub fn aref_pbr_cont_en_threshold(&self) -> ArefPbrContEnThresholdR {
        ArefPbrContEnThresholdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be deasserted."]
    #[inline(always)]
    pub fn aref_pbr_cont_dis_threshold(&self) -> ArefPbrContDisThresholdR {
        ArefPbrContDisThresholdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the per-bank refresh continuous request feature. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pbr_cont_req_en(&mut self) -> PbrContReqEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec> {
        PbrContReqEnW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn aref_pbr_cont_en_threshold(
        &mut self,
    ) -> ArefPbrContEnThresholdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec> {
        ArefPbrContEnThresholdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Sets the auto-refresh request count threshold when the PBR continuous refresh request enable will be deasserted."]
    #[inline(always)]
    #[must_use]
    pub fn aref_pbr_cont_dis_threshold(
        &mut self,
    ) -> ArefPbrContDisThresholdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec> {
        ArefPbrContDisThresholdW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_86::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_86::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_86::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_86::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_86 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl86Spec {
    const RESET_VALUE: u32 = 0;
}
