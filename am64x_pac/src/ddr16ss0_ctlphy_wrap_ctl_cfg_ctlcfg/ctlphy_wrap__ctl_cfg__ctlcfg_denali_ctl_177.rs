#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_177` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_177` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec>;
#[doc = "Field `DFS_PHY_REG_WRITE_MASK` reader - 3:0\\]
Register mask which will be written during a frequency change."]
pub type DfsPhyRegWriteMaskR = crate::FieldReader;
#[doc = "Field `DFS_PHY_REG_WRITE_MASK` writer - 3:0\\]
Register mask which will be written during a frequency change."]
pub type DfsPhyRegWriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DFS_PHY_REG_WRITE_WAIT` reader - 23:8\\]
Defines the number of DFI PHY clocks that the controller will wait after issuing the register write to the PHY during a frequency change."]
pub type DfsPhyRegWriteWaitR = crate::FieldReader<u16>;
#[doc = "Field `DFS_PHY_REG_WRITE_WAIT` writer - 23:8\\]
Defines the number of DFI PHY clocks that the controller will wait after issuing the register write to the PHY during a frequency change."]
pub type DfsPhyRegWriteWaitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CURRENT_REG_COPY` reader - 25:24\\]
Indicates the current copy of timing parameters that is in use by the controller."]
pub type CurrentRegCopyR = crate::FieldReader;
#[doc = "Field `CURRENT_REG_COPY` writer - 25:24\\]
Indicates the current copy of timing parameters that is in use by the controller."]
pub type CurrentRegCopyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Register mask which will be written during a frequency change."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_mask(&self) -> DfsPhyRegWriteMaskR {
        DfsPhyRegWriteMaskR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Defines the number of DFI PHY clocks that the controller will wait after issuing the register write to the PHY during a frequency change."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_wait(&self) -> DfsPhyRegWriteWaitR {
        DfsPhyRegWriteWaitR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Indicates the current copy of timing parameters that is in use by the controller."]
    #[inline(always)]
    pub fn current_reg_copy(&self) -> CurrentRegCopyR {
        CurrentRegCopyR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Register mask which will be written during a frequency change."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_mask(
        &mut self,
    ) -> DfsPhyRegWriteMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec> {
        DfsPhyRegWriteMaskW::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Defines the number of DFI PHY clocks that the controller will wait after issuing the register write to the PHY during a frequency change."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_wait(
        &mut self,
    ) -> DfsPhyRegWriteWaitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec> {
        DfsPhyRegWriteWaitW::new(self, 8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Indicates the current copy of timing parameters that is in use by the controller."]
    #[inline(always)]
    #[must_use]
    pub fn current_reg_copy(
        &mut self,
    ) -> CurrentRegCopyW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec> {
        CurrentRegCopyW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_177\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_177::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_177::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_177::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_177::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_177 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl177Spec {
    const RESET_VALUE: u32 = 0;
}