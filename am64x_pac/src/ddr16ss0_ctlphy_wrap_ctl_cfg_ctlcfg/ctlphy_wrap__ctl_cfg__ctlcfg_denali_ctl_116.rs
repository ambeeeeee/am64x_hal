#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_116` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_116` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F2` reader - 15:0\\]
DFI control update number of long counts until the normal priority request is asserted. FC=2"]
pub type UpdCtrlupdNormThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_NORM_THRESHOLD_F2` writer - 15:0\\]
DFI control update number of long counts until the normal priority request is asserted. FC=2"]
pub type UpdCtrlupdNormThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F2` reader - 31:16\\]
DFI control update number of long counts until the high priority request is asserted. FC=2"]
pub type UpdCtrlupdHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `UPD_CTRLUPD_HIGH_THRESHOLD_F2` writer - 31:16\\]
DFI control update number of long counts until the high priority request is asserted. FC=2"]
pub type UpdCtrlupdHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DFI control update number of long counts until the normal priority request is asserted. FC=2"]
    #[inline(always)]
    pub fn upd_ctrlupd_norm_threshold_f2(&self) -> UpdCtrlupdNormThresholdF2R {
        UpdCtrlupdNormThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DFI control update number of long counts until the high priority request is asserted. FC=2"]
    #[inline(always)]
    pub fn upd_ctrlupd_high_threshold_f2(&self) -> UpdCtrlupdHighThresholdF2R {
        UpdCtrlupdHighThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DFI control update number of long counts until the normal priority request is asserted. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_norm_threshold_f2(
        &mut self,
    ) -> UpdCtrlupdNormThresholdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec> {
        UpdCtrlupdNormThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DFI control update number of long counts until the high priority request is asserted. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn upd_ctrlupd_high_threshold_f2(
        &mut self,
    ) -> UpdCtrlupdHighThresholdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec> {
        UpdCtrlupdHighThresholdF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_116\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_116::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_116::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_116::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_116::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_116 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl116Spec {
    const RESET_VALUE: u32 = 0;
}
