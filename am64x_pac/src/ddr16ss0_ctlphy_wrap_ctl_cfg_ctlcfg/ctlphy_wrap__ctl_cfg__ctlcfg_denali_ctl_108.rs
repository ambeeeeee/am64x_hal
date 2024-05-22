#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_108` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_108` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F0` reader - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands. FC=0"]
pub type DfsPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F0` writer - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands. FC=0"]
pub type DfsPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F1` reader - 31:16\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands. FC=1"]
pub type DfsPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F1` writer - 31:16\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands. FC=1"]
pub type DfsPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands. FC=0"]
    #[inline(always)]
    pub fn dfs_promote_threshold_f0(&self) -> DfsPromoteThresholdF0R {
        DfsPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands. FC=1"]
    #[inline(always)]
    pub fn dfs_promote_threshold_f1(&self) -> DfsPromoteThresholdF1R {
        DfsPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn dfs_promote_threshold_f0(
        &mut self,
    ) -> DfsPromoteThresholdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec> {
        DfsPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn dfs_promote_threshold_f1(
        &mut self,
    ) -> DfsPromoteThresholdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec> {
        DfsPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_108\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_108::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_108::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_108::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_108::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_108 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl108Spec {
    const RESET_VALUE: u32 = 0;
}
