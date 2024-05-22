#[doc = "Register `CFG0_USB0_PHY_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Usb0PhyCtrlProxySpec>;
#[doc = "Register `CFG0_USB0_PHY_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Usb0PhyCtrlProxySpec>;
#[doc = "Field `USB0_PHY_CTRL_PLL_REF_SEL_PROXY` reader - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
pub type Usb0PhyCtrlPllRefSelProxyR = crate::FieldReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_REF_SEL_PROXY` writer - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
pub type Usb0PhyCtrlPllRefSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_SEL_PROXY` reader - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
pub type Usb0PhyCtrlPllClkoutSelProxyR = crate::FieldReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_SEL_PROXY` writer - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
pub type Usb0PhyCtrlPllClkoutSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_ON_PROXY` reader - 11:11\\]
Controls USB0 PLL clock output"]
pub type Usb0PhyCtrlPllClkoutOnProxyR = crate::BitReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_CLKOUT_ON_PROXY` writer - 11:11\\]
Controls USB0 PLL clock output"]
pub type Usb0PhyCtrlPllClkoutOnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0_PHY_CTRL_PLL_STANDALONE_PROXY` reader - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
pub type Usb0PhyCtrlPllStandaloneProxyR = crate::BitReader;
#[doc = "Field `USB0_PHY_CTRL_PLL_STANDALONE_PROXY` writer - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
pub type Usb0PhyCtrlPllStandaloneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0_PHY_CTRL_LOOPBACK_MODE_PROXY` reader - 17:16\\]
Activates USB0 PHY loopback operation"]
pub type Usb0PhyCtrlLoopbackModeProxyR = crate::FieldReader;
#[doc = "Field `USB0_PHY_CTRL_LOOPBACK_MODE_PROXY` writer - 17:16\\]
Activates USB0 PHY loopback operation"]
pub type Usb0PhyCtrlLoopbackModeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB0_PHY_CTRL_CORE_VOLTAGE_PROXY` reader - 31:31\\]
Selects the USB PHY Core Voltage"]
pub type Usb0PhyCtrlCoreVoltageProxyR = crate::BitReader;
#[doc = "Field `USB0_PHY_CTRL_CORE_VOLTAGE_PROXY` writer - 31:31\\]
Selects the USB PHY Core Voltage"]
pub type Usb0PhyCtrlCoreVoltageProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_ref_sel_proxy(&self) -> Usb0PhyCtrlPllRefSelProxyR {
        Usb0PhyCtrlPllRefSelProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_clkout_sel_proxy(&self) -> Usb0PhyCtrlPllClkoutSelProxyR {
        Usb0PhyCtrlPllClkoutSelProxyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Controls USB0 PLL clock output"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_clkout_on_proxy(&self) -> Usb0PhyCtrlPllClkoutOnProxyR {
        Usb0PhyCtrlPllClkoutOnProxyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_pll_standalone_proxy(&self) -> Usb0PhyCtrlPllStandaloneProxyR {
        Usb0PhyCtrlPllStandaloneProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Activates USB0 PHY loopback operation"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_loopback_mode_proxy(&self) -> Usb0PhyCtrlLoopbackModeProxyR {
        Usb0PhyCtrlLoopbackModeProxyR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Selects the USB PHY Core Voltage"]
    #[inline(always)]
    pub fn usb0_phy_ctrl_core_voltage_proxy(&self) -> Usb0PhyCtrlCoreVoltageProxyR {
        Usb0PhyCtrlCoreVoltageProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates the frequency of the REF_CLOCK input used by the USB PLL. This value should match the frequency value of either the HFOSC0 or HFOSC1 oscillator as selected by the USB0_CLKSEL register"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_ref_sel_proxy(
        &mut self,
    ) -> Usb0PhyCtrlPllRefSelProxyW<Cfg0Usb0PhyCtrlProxySpec> {
        Usb0PhyCtrlPllRefSelProxyW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects the frequency of the USB0 PLL output clock"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_clkout_sel_proxy(
        &mut self,
    ) -> Usb0PhyCtrlPllClkoutSelProxyW<Cfg0Usb0PhyCtrlProxySpec> {
        Usb0PhyCtrlPllClkoutSelProxyW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Controls USB0 PLL clock output"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_clkout_on_proxy(
        &mut self,
    ) -> Usb0PhyCtrlPllClkoutOnProxyW<Cfg0Usb0PhyCtrlProxySpec> {
        Usb0PhyCtrlPllClkoutOnProxyW::new(self, 11)
    }
    #[doc = "Bit 15 - 15:15\\]
Activates USB0 PHY as a standalone PLL"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_pll_standalone_proxy(
        &mut self,
    ) -> Usb0PhyCtrlPllStandaloneProxyW<Cfg0Usb0PhyCtrlProxySpec> {
        Usb0PhyCtrlPllStandaloneProxyW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Activates USB0 PHY loopback operation"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_loopback_mode_proxy(
        &mut self,
    ) -> Usb0PhyCtrlLoopbackModeProxyW<Cfg0Usb0PhyCtrlProxySpec> {
        Usb0PhyCtrlLoopbackModeProxyW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Selects the USB PHY Core Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn usb0_phy_ctrl_core_voltage_proxy(
        &mut self,
    ) -> Usb0PhyCtrlCoreVoltageProxyW<Cfg0Usb0PhyCtrlProxySpec> {
        Usb0PhyCtrlCoreVoltageProxyW::new(self, 31)
    }
}
#[doc = "CFG0_USB0_PHY_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_phy_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_phy_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usb0PhyCtrlProxySpec;
impl crate::RegisterSpec for Cfg0Usb0PhyCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usb0_phy_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Usb0PhyCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usb0_phy_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Usb0PhyCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USB0_PHY_CTRL_PROXY to value 0x8000_0004"]
impl crate::Resettable for Cfg0Usb0PhyCtrlProxySpec {
    const RESET_VALUE: u32 = 0x8000_0004;
}
