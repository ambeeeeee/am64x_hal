#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1293` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1293` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec>;
#[doc = "Field `PHY_CSLVL_ENABLE` reader - 0:0\\]
CS training enable. Set to 1 to enable CS training during CA training."]
pub type PhyCslvlEnableR = crate::BitReader;
#[doc = "Field `PHY_CSLVL_ENABLE` writer - 0:0\\]
CS training enable. Set to 1 to enable CS training during CA training."]
pub type PhyCslvlEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CSLVL_PERIODIC_START_OFFSET` reader - 16:8\\]
Defines the relative offset from previous LE and TE to start periodic CSLVL with."]
pub type PhyCslvlPeriodicStartOffsetR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_PERIODIC_START_OFFSET` writer - 16:8\\]
Defines the relative offset from previous LE and TE to start periodic CSLVL with."]
pub type PhyCslvlPeriodicStartOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PHY_LP4_BOOT_DISABLE` reader - 24:24\\]
Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
pub type PhyLp4BootDisableR = crate::BitReader;
#[doc = "Field `PHY_LP4_BOOT_DISABLE` writer - 24:24\\]
Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
pub type PhyLp4BootDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CS training enable. Set to 1 to enable CS training during CA training."]
    #[inline(always)]
    pub fn phy_cslvl_enable(&self) -> PhyCslvlEnableR {
        PhyCslvlEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Defines the relative offset from previous LE and TE to start periodic CSLVL with."]
    #[inline(always)]
    pub fn phy_cslvl_periodic_start_offset(&self) -> PhyCslvlPeriodicStartOffsetR {
        PhyCslvlPeriodicStartOffsetR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
    #[inline(always)]
    pub fn phy_lp4_boot_disable(&self) -> PhyLp4BootDisableR {
        PhyLp4BootDisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CS training enable. Set to 1 to enable CS training during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_enable(
        &mut self,
    ) -> PhyCslvlEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec> {
        PhyCslvlEnableW::new(self, 0)
    }
    #[doc = "Bits 8:16 - 16:8\\]
Defines the relative offset from previous LE and TE to start periodic CSLVL with."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_periodic_start_offset(
        &mut self,
    ) -> PhyCslvlPeriodicStartOffsetW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec> {
        PhyCslvlPeriodicStartOffsetW::new(self, 8)
    }
    #[doc = "Bit 24 - 24:24\\]
Controls the handling of the DFI frequency. When set to 1, DFI frequency 0 is considered the first operational frequency. When cleared to 0, DFI frequency 0 is the boot frequency and other DFI frequency values are operational frequencies. Must be cleared to 0 for LPDDR3 devices operating in an LPDDR4 capable configuration."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_disable(
        &mut self,
    ) -> PhyLp4BootDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec> {
        PhyLp4BootDisableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1293\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1293::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1293::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1293::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1293::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1293 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1293Spec {
    const RESET_VALUE: u32 = 0;
}
