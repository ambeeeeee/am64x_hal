#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_63` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_63` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec>;
#[doc = "Field `PI_CLKDISABLE_2_INIT_START` reader - 7:0\\]
Defines the delay from the asserting of dfi_dram_clk_disable to the asserting of dfi_init_start in DFI clock."]
pub type PiClkdisable2InitStartR = crate::FieldReader;
#[doc = "Field `PI_CLKDISABLE_2_INIT_START` writer - 7:0\\]
Defines the delay from the asserting of dfi_dram_clk_disable to the asserting of dfi_init_start in DFI clock."]
pub type PiClkdisable2InitStartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_INIT_STARTORCOMPLETE_2_CLKDISABLE` reader - 15:8\\]
Defines the delay from deasserting of dfi_init_start or asserting of dfi_init_complete to deasserting of dfi_dram_clk_disable in DFI clock."]
pub type PiInitStartorcomplete2ClkdisableR = crate::FieldReader;
#[doc = "Field `PI_INIT_STARTORCOMPLETE_2_CLKDISABLE` writer - 15:8\\]
Defines the delay from deasserting of dfi_init_start or asserting of dfi_init_complete to deasserting of dfi_dram_clk_disable in DFI clock."]
pub type PiInitStartorcomplete2ClkdisableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_DRAM_CLK_DISABLE_DEASSERT_SEL` reader - 16:16\\]
Indicate dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert. Set to 0: dfi_dram_clk_disable deassert following dfi_init_start deassert. Set to 1: dfi_dram_clk_disable deassert following dfi_init_complete assert."]
pub type PiDramClkDisableDeassertSelR = crate::BitReader;
#[doc = "Field `PI_DRAM_CLK_DISABLE_DEASSERT_SEL` writer - 16:16\\]
Indicate dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert. Set to 0: dfi_dram_clk_disable deassert following dfi_init_start deassert. Set to 1: dfi_dram_clk_disable deassert following dfi_init_complete assert."]
pub type PiDramClkDisableDeassertSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_MIN` reader - 31:24\\]
Minimum number of DFI clocks from dfi_init_complete to a command/training event."]
pub type PiTdfiInitCompleteMinR = crate::FieldReader;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_MIN` writer - 31:24\\]
Minimum number of DFI clocks from dfi_init_complete to a command/training event."]
pub type PiTdfiInitCompleteMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the delay from the asserting of dfi_dram_clk_disable to the asserting of dfi_init_start in DFI clock."]
    #[inline(always)]
    pub fn pi_clkdisable_2_init_start(&self) -> PiClkdisable2InitStartR {
        PiClkdisable2InitStartR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the delay from deasserting of dfi_init_start or asserting of dfi_init_complete to deasserting of dfi_dram_clk_disable in DFI clock."]
    #[inline(always)]
    pub fn pi_init_startorcomplete_2_clkdisable(&self) -> PiInitStartorcomplete2ClkdisableR {
        PiInitStartorcomplete2ClkdisableR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicate dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert. Set to 0: dfi_dram_clk_disable deassert following dfi_init_start deassert. Set to 1: dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    #[inline(always)]
    pub fn pi_dram_clk_disable_deassert_sel(&self) -> PiDramClkDisableDeassertSelR {
        PiDramClkDisableDeassertSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Minimum number of DFI clocks from dfi_init_complete to a command/training event."]
    #[inline(always)]
    pub fn pi_tdfi_init_complete_min(&self) -> PiTdfiInitCompleteMinR {
        PiTdfiInitCompleteMinR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the delay from the asserting of dfi_dram_clk_disable to the asserting of dfi_init_start in DFI clock."]
    #[inline(always)]
    #[must_use]
    pub fn pi_clkdisable_2_init_start(
        &mut self,
    ) -> PiClkdisable2InitStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec> {
        PiClkdisable2InitStartW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the delay from deasserting of dfi_init_start or asserting of dfi_init_complete to deasserting of dfi_dram_clk_disable in DFI clock."]
    #[inline(always)]
    #[must_use]
    pub fn pi_init_startorcomplete_2_clkdisable(
        &mut self,
    ) -> PiInitStartorcomplete2ClkdisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec> {
        PiInitStartorcomplete2ClkdisableW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicate dfi_dram_clk_disable deassert following dfi_init_start deassert or dfi_init_complete assert. Set to 0: dfi_dram_clk_disable deassert following dfi_init_start deassert. Set to 1: dfi_dram_clk_disable deassert following dfi_init_complete assert."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dram_clk_disable_deassert_sel(
        &mut self,
    ) -> PiDramClkDisableDeassertSelW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec> {
        PiDramClkDisableDeassertSelW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Minimum number of DFI clocks from dfi_init_complete to a command/training event."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_complete_min(
        &mut self,
    ) -> PiTdfiInitCompleteMinW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec> {
        PiTdfiInitCompleteMinW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_63::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_63::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_63 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi63Spec {
    const RESET_VALUE: u32 = 0;
}
