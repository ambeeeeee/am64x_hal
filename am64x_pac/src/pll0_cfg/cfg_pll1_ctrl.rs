#[doc = "Register `CFG_pll1_CTRL` reader"]
pub type R = crate::R<CfgPll1CtrlSpec>;
#[doc = "Register `CFG_pll1_CTRL` writer"]
pub type W = crate::W<CfgPll1CtrlSpec>;
#[doc = "Field `DAC_EN` reader - 0:0\\]
Enable fractional noise canceling DAC This bit has no function (DAC is always disabled) in integer divide mode (when dsm_en=0) 1'b0 - Fractional NC DAC is disabled (for Test modes only) 1'b1 - Fractional NC DAC is enabled (ignored in integer divide mode)"]
pub type DacEnR = crate::BitReader;
#[doc = "Field `DAC_EN` writer - 0:0\\]
Enable fractional noise canceling DAC This bit has no function (DAC is always disabled) in integer divide mode (when dsm_en=0) 1'b0 - Fractional NC DAC is disabled (for Test modes only) 1'b1 - Fractional NC DAC is enabled (ignored in integer divide mode)"]
pub type DacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSM_EN` reader - 1:1\\]
Delta-Sigma modulator enable 1'b0 - Delta-Sigma modulator is disabled (use integer divide mode) 1'b1 - Delta-Sigma modulator is enabled (use fractional divide mode)"]
pub type DsmEnR = crate::BitReader;
#[doc = "Field `DSM_EN` writer - 1:1\\]
Delta-Sigma modulator enable 1'b0 - Delta-Sigma modulator is disabled (use integer divide mode) 1'b1 - Delta-Sigma modulator is enabled (use fractional divide mode)"]
pub type DsmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_POSTDIV_EN` reader - 4:4\\]
Post divide CLK enable 1'b0 - Post divide powered down. FOUTPOSTDIV and all auxiliary PLL clocks (4-ohase and synchronous divided clocks) are held low 1'b1 - Post divide enabled. FOUTPOSTDIV, 4-phase and synchronous clocks are enabled."]
pub type ClkPostdivEnR = crate::BitReader;
#[doc = "Field `CLK_POSTDIV_EN` writer - 4:4\\]
Post divide CLK enable 1'b0 - Post divide powered down. FOUTPOSTDIV and all auxiliary PLL clocks (4-ohase and synchronous divided clocks) are held low 1'b1 - Post divide enabled. FOUTPOSTDIV, 4-phase and synchronous clocks are enabled."]
pub type ClkPostdivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_4PH_EN` reader - 5:5\\]
Enable 4-phase clock generator. This bit is ignored if clk_postdiv_en=0 1'b0 - 4-phase dividers disabled. FOUT1PHx and FOUTn clocks are held low. 1'b1 - 4-phase dividers enabled. FOUT1PH0/90/180/270 and FOUT2, FOUT3, FOUT4 clocks are enabled."]
pub type Clk4phEnR = crate::BitReader;
#[doc = "Field `CLK_4PH_EN` writer - 5:5\\]
Enable 4-phase clock generator. This bit is ignored if clk_postdiv_en=0 1'b0 - 4-phase dividers disabled. FOUT1PHx and FOUTn clocks are held low. 1'b1 - 4-phase dividers enabled. FOUT1PH0/90/180/270 and FOUT2, FOUT3, FOUT4 clocks are enabled."]
pub type Clk4phEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTL_BYP_EN` reader - 8:8\\]
PLL internal bypass enable. This is an asynchronous mux which can produce glitches on the outputclocks during switching. 1'b0 - Output clocks are derived from the VCO clock 1'b1 - Output clocks are derived from the FREF reference clock"]
pub type IntlBypEnR = crate::BitReader;
#[doc = "Field `INTL_BYP_EN` writer - 8:8\\]
PLL internal bypass enable. This is an asynchronous mux which can produce glitches on the outputclocks during switching. 1'b0 - Output clocks are derived from the VCO clock 1'b1 - Output clocks are derived from the FREF reference clock"]
pub type IntlBypEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL_EN` reader - 15:15\\]
PLL enable 1'b0 - PLL is disabled 1'b1 - PLL is enabled"]
pub type PllEnR = crate::BitReader;
#[doc = "Field `PLL_EN` writer - 15:15\\]
PLL enable 1'b0 - PLL is disabled 1'b1 - PLL is enabled"]
pub type PllEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYP_ON_LOCKLOSS` reader - 16:16\\]
Bypass on loss of PLL lock. This bit controls the PLL bypass mux to automatically switch the clock source to the reference clock when the PLL losses lock. 1'b0 - Do not automatically switch to ref clock source on PLL lock loss 1'b1 - Switch to ref clock source when PLL losses lock"]
pub type BypOnLocklossR = crate::BitReader;
#[doc = "Field `BYP_ON_LOCKLOSS` writer - 16:16\\]
Bypass on loss of PLL lock. This bit controls the PLL bypass mux to automatically switch the clock source to the reference clock when the PLL losses lock. 1'b0 - Do not automatically switch to ref clock source on PLL lock loss 1'b1 - Switch to ref clock source when PLL losses lock"]
pub type BypOnLocklossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_EN` reader - 31:31\\]
Bypass enable. This controls the glitch-free bypass mux. The bypass_en bit should be set prior tomaking any changes to the PLL settings. 1'b0 - Synchronously select PLL and associated HSDIV clock outputs 1'b1 - Synchronously select the reference clock for all PLL and HSDIV clock outputs"]
pub type BypassEnR = crate::BitReader;
#[doc = "Field `BYPASS_EN` writer - 31:31\\]
Bypass enable. This controls the glitch-free bypass mux. The bypass_en bit should be set prior tomaking any changes to the PLL settings. 1'b0 - Synchronously select PLL and associated HSDIV clock outputs 1'b1 - Synchronously select the reference clock for all PLL and HSDIV clock outputs"]
pub type BypassEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable fractional noise canceling DAC This bit has no function (DAC is always disabled) in integer divide mode (when dsm_en=0) 1'b0 - Fractional NC DAC is disabled (for Test modes only) 1'b1 - Fractional NC DAC is enabled (ignored in integer divide mode)"]
    #[inline(always)]
    pub fn dac_en(&self) -> DacEnR {
        DacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Delta-Sigma modulator enable 1'b0 - Delta-Sigma modulator is disabled (use integer divide mode) 1'b1 - Delta-Sigma modulator is enabled (use fractional divide mode)"]
    #[inline(always)]
    pub fn dsm_en(&self) -> DsmEnR {
        DsmEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Post divide CLK enable 1'b0 - Post divide powered down. FOUTPOSTDIV and all auxiliary PLL clocks (4-ohase and synchronous divided clocks) are held low 1'b1 - Post divide enabled. FOUTPOSTDIV, 4-phase and synchronous clocks are enabled."]
    #[inline(always)]
    pub fn clk_postdiv_en(&self) -> ClkPostdivEnR {
        ClkPostdivEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable 4-phase clock generator. This bit is ignored if clk_postdiv_en=0 1'b0 - 4-phase dividers disabled. FOUT1PHx and FOUTn clocks are held low. 1'b1 - 4-phase dividers enabled. FOUT1PH0/90/180/270 and FOUT2, FOUT3, FOUT4 clocks are enabled."]
    #[inline(always)]
    pub fn clk_4ph_en(&self) -> Clk4phEnR {
        Clk4phEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL internal bypass enable. This is an asynchronous mux which can produce glitches on the outputclocks during switching. 1'b0 - Output clocks are derived from the VCO clock 1'b1 - Output clocks are derived from the FREF reference clock"]
    #[inline(always)]
    pub fn intl_byp_en(&self) -> IntlBypEnR {
        IntlBypEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
PLL enable 1'b0 - PLL is disabled 1'b1 - PLL is enabled"]
    #[inline(always)]
    pub fn pll_en(&self) -> PllEnR {
        PllEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Bypass on loss of PLL lock. This bit controls the PLL bypass mux to automatically switch the clock source to the reference clock when the PLL losses lock. 1'b0 - Do not automatically switch to ref clock source on PLL lock loss 1'b1 - Switch to ref clock source when PLL losses lock"]
    #[inline(always)]
    pub fn byp_on_lockloss(&self) -> BypOnLocklossR {
        BypOnLocklossR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Bypass enable. This controls the glitch-free bypass mux. The bypass_en bit should be set prior tomaking any changes to the PLL settings. 1'b0 - Synchronously select PLL and associated HSDIV clock outputs 1'b1 - Synchronously select the reference clock for all PLL and HSDIV clock outputs"]
    #[inline(always)]
    pub fn bypass_en(&self) -> BypassEnR {
        BypassEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable fractional noise canceling DAC This bit has no function (DAC is always disabled) in integer divide mode (when dsm_en=0) 1'b0 - Fractional NC DAC is disabled (for Test modes only) 1'b1 - Fractional NC DAC is enabled (ignored in integer divide mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_en(&mut self) -> DacEnW<CfgPll1CtrlSpec> {
        DacEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Delta-Sigma modulator enable 1'b0 - Delta-Sigma modulator is disabled (use integer divide mode) 1'b1 - Delta-Sigma modulator is enabled (use fractional divide mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dsm_en(&mut self) -> DsmEnW<CfgPll1CtrlSpec> {
        DsmEnW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Post divide CLK enable 1'b0 - Post divide powered down. FOUTPOSTDIV and all auxiliary PLL clocks (4-ohase and synchronous divided clocks) are held low 1'b1 - Post divide enabled. FOUTPOSTDIV, 4-phase and synchronous clocks are enabled."]
    #[inline(always)]
    #[must_use]
    pub fn clk_postdiv_en(&mut self) -> ClkPostdivEnW<CfgPll1CtrlSpec> {
        ClkPostdivEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable 4-phase clock generator. This bit is ignored if clk_postdiv_en=0 1'b0 - 4-phase dividers disabled. FOUT1PHx and FOUTn clocks are held low. 1'b1 - 4-phase dividers enabled. FOUT1PH0/90/180/270 and FOUT2, FOUT3, FOUT4 clocks are enabled."]
    #[inline(always)]
    #[must_use]
    pub fn clk_4ph_en(&mut self) -> Clk4phEnW<CfgPll1CtrlSpec> {
        Clk4phEnW::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
PLL internal bypass enable. This is an asynchronous mux which can produce glitches on the outputclocks during switching. 1'b0 - Output clocks are derived from the VCO clock 1'b1 - Output clocks are derived from the FREF reference clock"]
    #[inline(always)]
    #[must_use]
    pub fn intl_byp_en(&mut self) -> IntlBypEnW<CfgPll1CtrlSpec> {
        IntlBypEnW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
PLL enable 1'b0 - PLL is disabled 1'b1 - PLL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn pll_en(&mut self) -> PllEnW<CfgPll1CtrlSpec> {
        PllEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Bypass on loss of PLL lock. This bit controls the PLL bypass mux to automatically switch the clock source to the reference clock when the PLL losses lock. 1'b0 - Do not automatically switch to ref clock source on PLL lock loss 1'b1 - Switch to ref clock source when PLL losses lock"]
    #[inline(always)]
    #[must_use]
    pub fn byp_on_lockloss(&mut self) -> BypOnLocklossW<CfgPll1CtrlSpec> {
        BypOnLocklossW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Bypass enable. This controls the glitch-free bypass mux. The bypass_en bit should be set prior tomaking any changes to the PLL settings. 1'b0 - Synchronously select PLL and associated HSDIV clock outputs 1'b1 - Synchronously select the reference clock for all PLL and HSDIV clock outputs"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_en(&mut self) -> BypassEnW<CfgPll1CtrlSpec> {
        BypassEnW::new(self, 31)
    }
}
#[doc = "CFG_pll1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll1CtrlSpec;
impl crate::RegisterSpec for CfgPll1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll1_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgPll1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll1_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgPll1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll1_CTRL to value 0x8001_0011"]
impl crate::Resettable for CfgPll1CtrlSpec {
    const RESET_VALUE: u32 = 0x8001_0011;
}
