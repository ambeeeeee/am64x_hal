#[doc = "Register `CFG0_USB0_PHY_CTRL` reader"]
pub type R = crate::R<Cfg0Usb0PhyCtrlSpec>;
#[doc = "Register `CFG0_USB0_PHY_CTRL` writer"]
pub type W = crate::W<Cfg0Usb0PhyCtrlSpec>;
#[doc = "Field `USB0_PHY_CTRL_PLL_REF_SEL` reader - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
pub type Usb0PhyCtrlPllRefSelR = crate::FieldReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_REF_SEL` writer - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
pub type Usb0PhyCtrlPllRefSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_SEL` reader - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
pub type Usb0PhyCtrlPllClkoutSelR = crate::FieldReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_SEL` writer - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
pub type Usb0PhyCtrlPllClkoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_ON` reader - 11:11\\]
Controls USB0 PLL clock output"]
pub type Usb0PhyCtrlPllClkoutOnR = crate::BitReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_ON` writer - 11:11\\]
Controls USB0 PLL clock output"]
pub type Usb0PhyCtrlPllClkoutOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0_PHY_CTRL_PLL_STANDALONE` reader - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
pub type Usb0PhyCtrlPllStandaloneR = crate::BitReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_STANDALONE` writer - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
pub type Usb0PhyCtrlPllStandaloneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0_PHY_CTRL_LOOPBACK_MODE` reader - 17:16\\]
Activates USB0 PHY loopback operation"]
pub type Usb0PhyCtrlLoopbackModeR = crate::FieldReader;
#[doc = "Field `USB0_PHY_CTRL_LOOPBACK_MODE` writer - 17:16\\]
Activates USB0 PHY loopback operation"]
pub type Usb0PhyCtrlLoopbackModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB0_PHY_CTRL_CORE_VOLTAGE` reader - 31:31\\]
Selects the USB PHY Core Voltage"]
pub type Usb0PhyCtrlCoreVoltageR = crate::BitReader;
#[doc = "Field `USB0_PHY_CTRL_CORE_VOLTAGE` writer - 31:31\\]
Selects the USB PHY Core Voltage"]
pub type Usb0PhyCtrlCoreVoltageW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_ref_sel(&self) -> Usb0PhyCtrlPllRefSelR {
        Usb0PhyCtrlPllRefSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_clkout_sel(&self) -> Usb0PhyCtrlPllClkoutSelR {
        Usb0PhyCtrlPllClkoutSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Controls USB0 PLL clock output"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_clkout_on(&self) -> Usb0PhyCtrlPllClkoutOnR {
        Usb0PhyCtrlPllClkoutOnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_standalone(&self) -> Usb0PhyCtrlPllStandaloneR {
        Usb0PhyCtrlPllStandaloneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Activates USB0 PHY loopback operation"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_loopback_mode(&self) -> Usb0PhyCtrlLoopbackModeR {
        Usb0PhyCtrlLoopbackModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Selects the USB PHY Core Voltage"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_core_voltage(&self) -> Usb0PhyCtrlCoreVoltageR {
        Usb0PhyCtrlCoreVoltageR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_ref_sel(&mut self) -> Usb0PhyCtrlPllRefSelW<Cfg0Usb0PhyCtrlSpec> {
        Usb0PhyCtrlPllRefSelW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_clkout_sel(
        &mut self,
    ) -> Usb0PhyCtrlPllClkoutSelW<Cfg0Usb0PhyCtrlSpec> {
        Usb0PhyCtrlPllClkoutSelW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Controls USB0 PLL clock output"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_clkout_on(&mut self) -> Usb0PhyCtrlPllClkoutOnW<Cfg0Usb0PhyCtrlSpec> {
        Usb0PhyCtrlPllClkoutOnW::new(self, 11)
    }
    #[doc = "Bit 15 - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_standalone(
        &mut self,
    ) -> Usb0PhyCtrlPllStandaloneW<Cfg0Usb0PhyCtrlSpec> {
        Usb0PhyCtrlPllStandaloneW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Activates USB0 PHY loopback operation"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_loopback_mode(&mut self) -> Usb0PhyCtrlLoopbackModeW<Cfg0Usb0PhyCtrlSpec> {
        Usb0PhyCtrlLoopbackModeW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Selects the USB PHY Core Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_core_voltage(&mut self) -> Usb0PhyCtrlCoreVoltageW<Cfg0Usb0PhyCtrlSpec> {
        Usb0PhyCtrlCoreVoltageW::new(self, 31)
    }
}
#[doc = "CFG0_USB0_PHY_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_phy_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_phy_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usb0PhyCtrlSpec;
impl crate::RegisterSpec for Cfg0Usb0PhyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usb0_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Usb0PhyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usb0_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Usb0PhyCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USB0_PHY_CTRL to value 0x8000_0004"]
impl crate::Resettable for Cfg0Usb0PhyCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0004;
}
