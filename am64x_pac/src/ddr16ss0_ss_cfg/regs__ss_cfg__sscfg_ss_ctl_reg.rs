#[doc = "Register `REGS__SS_CFG__SSCFG_SS_CTL_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgSsCtlRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_SS_CTL_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgSsCtlRegSpec>;
#[doc = "Field `PHY_PLL_BYPASS` reader - 0:0\\]
Cadence PHY De-Skew PLL bypass. Write 1 to bypass PLL."]
pub type PhyPllBypassR = crate::BitReader;
#[doc = "Field `PHY_PLL_BYPASS` writer - 0:0\\]
Cadence PHY De-Skew PLL bypass. Write 1 to bypass PLL."]
pub type PhyPllBypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Cadence PHY De-Skew PLL bypass. Write 1 to bypass PLL."]
    #[inline(always)]
    pub fn phy_pll_bypass(&self) -> PhyPllBypassR {
        PhyPllBypassR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Cadence PHY De-Skew PLL bypass. Write 1 to bypass PLL."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_bypass(&mut self) -> PhyPllBypassW<Regs_SsCfg_SscfgSsCtlRegSpec> {
        PhyPllBypassW::new(self, 0)
    }
}
#[doc = "The Subsystem Control Register contains fields for control functions required for submodules in the subsystem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ss_ctl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ss_ctl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgSsCtlRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgSsCtlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ss_ctl_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgSsCtlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ss_ctl_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgSsCtlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_SS_CTL_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgSsCtlRegSpec {
    const RESET_VALUE: u32 = 0;
}
