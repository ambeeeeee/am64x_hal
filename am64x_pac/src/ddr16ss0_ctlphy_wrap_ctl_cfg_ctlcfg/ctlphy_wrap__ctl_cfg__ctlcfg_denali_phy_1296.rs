#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1296` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1296` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec>;
#[doc = "Field `PHY_ADRCTL_SNAP_OBS_REGS` reader - 0:0\\]
Initiates a snapshot of the internal observation registers for the address/control block. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyAdrctlSnapObsRegsR = crate::BitReader;
#[doc = "Field `PHY_ADRCTL_SNAP_OBS_REGS` writer - 0:0\\]
Initiates a snapshot of the internal observation registers for the address/control block. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyAdrctlSnapObsRegsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DFI_PHYUPD_TYPE` reader - 9:8\\]
Defines the value of the dfi_phyupd_type output signal to MC."]
pub type PhyDfiPhyupdTypeR = crate::FieldReader;
#[doc = "Field `PHY_DFI_PHYUPD_TYPE` writer - 9:8\\]
Defines the value of the dfi_phyupd_type output signal to MC."]
pub type PhyDfiPhyupdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADRCTL_LPDDR` reader - 16:16\\]
Adds a cycle of delay for the address/control slices to match the address slice."]
pub type PhyAdrctlLpddrR = crate::BitReader;
#[doc = "Field `PHY_ADRCTL_LPDDR` writer - 16:16\\]
Adds a cycle of delay for the address/control slices to match the address slice."]
pub type PhyAdrctlLpddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_ACTIVE` reader - 24:24\\]
Indicates an LPDDR4 device is connected to the PHY."]
pub type PhyLp4ActiveR = crate::BitReader;
#[doc = "Field `PHY_LP4_ACTIVE` writer - 24:24\\]
Indicates an LPDDR4 device is connected to the PHY."]
pub type PhyLp4ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Initiates a snapshot of the internal observation registers for the address/control block. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn phy_adrctl_snap_obs_regs(&self) -> PhyAdrctlSnapObsRegsR {
        PhyAdrctlSnapObsRegsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the value of the dfi_phyupd_type output signal to MC."]
    #[inline(always)]
    pub fn phy_dfi_phyupd_type(&self) -> PhyDfiPhyupdTypeR {
        PhyDfiPhyupdTypeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Adds a cycle of delay for the address/control slices to match the address slice."]
    #[inline(always)]
    pub fn phy_adrctl_lpddr(&self) -> PhyAdrctlLpddrR {
        PhyAdrctlLpddrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates an LPDDR4 device is connected to the PHY."]
    #[inline(always)]
    pub fn phy_lp4_active(&self) -> PhyLp4ActiveR {
        PhyLp4ActiveR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Initiates a snapshot of the internal observation registers for the address/control block. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_snap_obs_regs(
        &mut self,
    ) -> PhyAdrctlSnapObsRegsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec> {
        PhyAdrctlSnapObsRegsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the value of the dfi_phyupd_type output signal to MC."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dfi_phyupd_type(
        &mut self,
    ) -> PhyDfiPhyupdTypeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec> {
        PhyDfiPhyupdTypeW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Adds a cycle of delay for the address/control slices to match the address slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_lpddr(
        &mut self,
    ) -> PhyAdrctlLpddrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec> {
        PhyAdrctlLpddrW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates an LPDDR4 device is connected to the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_active(&mut self) -> PhyLp4ActiveW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec> {
        PhyLp4ActiveW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1296\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1296::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1296::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1296::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1296::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1296 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1296Spec {
    const RESET_VALUE: u32 = 0;
}
