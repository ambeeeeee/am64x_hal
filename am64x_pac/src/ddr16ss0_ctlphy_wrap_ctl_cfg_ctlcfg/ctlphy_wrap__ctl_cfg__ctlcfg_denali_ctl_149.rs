#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_149` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_149` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F2` reader - 23:0\\]
MRR temp check number of long counts until the timeout is asserted. FC=2"]
pub type MrrTempchkTimeoutF2R = crate::FieldReader<u32>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F2` writer - 23:0\\]
MRR temp check number of long counts until the timeout is asserted. FC=2"]
pub type MrrTempchkTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PPR_CONTROL` reader - 24:24\\]
Enables the post-package repair feature. Set to 1 to enable. This parameter may only be programmed before initialization begins."]
pub type PprControlR = crate::BitReader;
#[doc = "Field `PPR_CONTROL` writer - 24:24\\]
Enables the post-package repair feature. Set to 1 to enable. This parameter may only be programmed before initialization begins."]
pub type PprControlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
MRR temp check number of long counts until the timeout is asserted. FC=2"]
    #[inline(always)]
    pub fn mrr_tempchk_timeout_f2(&self) -> MrrTempchkTimeoutF2R {
        MrrTempchkTimeoutF2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the post-package repair feature. Set to 1 to enable. This parameter may only be programmed before initialization begins."]
    #[inline(always)]
    pub fn ppr_control(&self) -> PprControlR {
        PprControlR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
MRR temp check number of long counts until the timeout is asserted. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_timeout_f2(
        &mut self,
    ) -> MrrTempchkTimeoutF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec> {
        MrrTempchkTimeoutF2W::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the post-package repair feature. Set to 1 to enable. This parameter may only be programmed before initialization begins."]
    #[inline(always)]
    #[must_use]
    pub fn ppr_control(&mut self) -> PprControlW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec> {
        PprControlW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_149\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_149::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_149::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_149::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_149::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_149 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl149Spec {
    const RESET_VALUE: u32 = 0;
}
