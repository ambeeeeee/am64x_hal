#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_109` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_109` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F2` reader - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands. FC=2"]
pub type DfsPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F2` writer - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands. FC=2"]
pub type DfsPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_STATUS_LOG` reader - 18:16\\]
Indicates what kind of ZQ command was terminated without execution that caused the ZQ status interrupt to assert. Bit \\[0\\]
correlates to a ZQ cal init, reset, short or long command. Bit \\[1\\]
correlates to a ZQ cal start command. Bit \\[2\\]
correlates to a ZQ cal latch command. Value of 1 indicates that that type of command was received, but terminated without execution. READ-ONLY"]
pub type ZqStatusLogR = crate::FieldReader;
#[doc = "Field `ZQ_STATUS_LOG` writer - 18:16\\]
Indicates what kind of ZQ command was terminated without execution that caused the ZQ status interrupt to assert. Bit \\[0\\]
correlates to a ZQ cal init, reset, short or long command. Bit \\[1\\]
correlates to a ZQ cal start command. Bit \\[2\\]
correlates to a ZQ cal latch command. Value of 1 indicates that that type of command was received, but terminated without execution. READ-ONLY"]
pub type ZqStatusLogW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands. FC=2"]
    #[inline(always)]
    pub fn dfs_promote_threshold_f2(&self) -> DfsPromoteThresholdF2R {
        DfsPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Indicates what kind of ZQ command was terminated without execution that caused the ZQ status interrupt to assert. Bit \\[0\\]
correlates to a ZQ cal init, reset, short or long command. Bit \\[1\\]
correlates to a ZQ cal start command. Bit \\[2\\]
correlates to a ZQ cal latch command. Value of 1 indicates that that type of command was received, but terminated without execution. READ-ONLY"]
    #[inline(always)]
    pub fn zq_status_log(&self) -> ZqStatusLogR {
        ZqStatusLogR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn dfs_promote_threshold_f2(
        &mut self,
    ) -> DfsPromoteThresholdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec> {
        DfsPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Indicates what kind of ZQ command was terminated without execution that caused the ZQ status interrupt to assert. Bit \\[0\\]
correlates to a ZQ cal init, reset, short or long command. Bit \\[1\\]
correlates to a ZQ cal start command. Bit \\[2\\]
correlates to a ZQ cal latch command. Value of 1 indicates that that type of command was received, but terminated without execution. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn zq_status_log(&mut self) -> ZqStatusLogW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec> {
        ZqStatusLogW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_109\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_109::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_109::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_109::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_109::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_109 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl109Spec {
    const RESET_VALUE: u32 = 0;
}
