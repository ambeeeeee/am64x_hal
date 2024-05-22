#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_175` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_175` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec>;
#[doc = "Field `DFS_DLL_OFF` reader - 2:0\\]
Defines if the memory DLL must be off for the associated frequency set. Bit \\[0\\]
corresponds to frequency set 0, bit \\[1\\]
corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
pub type DfsDllOffR = crate::FieldReader;
#[doc = "Field `DFS_DLL_OFF` writer - 2:0\\]
Defines if the memory DLL must be off for the associated frequency set. Bit \\[0\\]
corresponds to frequency set 0, bit \\[1\\]
corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
pub type DfsDllOffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DFS_PHY_REG_WRITE_EN` reader - 8:8\\]
Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
pub type DfsPhyRegWriteEnR = crate::BitReader;
#[doc = "Field `DFS_PHY_REG_WRITE_EN` writer - 8:8\\]
Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
pub type DfsPhyRegWriteEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Defines if the memory DLL must be off for the associated frequency set. Bit \\[0\\]
corresponds to frequency set 0, bit \\[1\\]
corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
    #[inline(always)]
    pub fn dfs_dll_off(&self) -> DfsDllOffR {
        DfsDllOffR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_phy_reg_write_en(&self) -> DfsPhyRegWriteEnR {
        DfsPhyRegWriteEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Defines if the memory DLL must be off for the associated frequency set. Bit \\[0\\]
corresponds to frequency set 0, bit \\[1\\]
corresponds to frequency set 1, etc. Set each bit to 1 to require DLL off."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_dll_off(&mut self) -> DfsDllOffW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec> {
        DfsDllOffW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable a register write to the PHY during a frequency change. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_phy_reg_write_en(
        &mut self,
    ) -> DfsPhyRegWriteEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec> {
        DfsPhyRegWriteEnW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_175\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_175::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_175::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_175::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_175::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_175 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl175Spec {
    const RESET_VALUE: u32 = 0;
}
