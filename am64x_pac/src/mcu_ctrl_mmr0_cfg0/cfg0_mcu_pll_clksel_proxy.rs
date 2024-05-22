#[doc = "Register `CFG0_MCU_PLL_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0McuPllClkselProxySpec>;
#[doc = "Register `CFG0_MCU_PLL_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0McuPllClkselProxySpec>;
#[doc = "Field `MCU_PLL_CLKSEL_CLKLOSS_SWTCH_EN_PROXY` reader - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
pub type McuPllClkselClklossSwtchEnProxyR = crate::BitReader;
#[doc = "Field `MCU_PLL_CLKSEL_CLKLOSS_SWTCH_EN_PROXY` writer - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
pub type McuPllClkselClklossSwtchEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_PLL_CLKSEL_BYP_WARM_RST_PROXY` reader - 23:23\\]
PLL bypass mode after warm reset."]
pub type McuPllClkselBypWarmRstProxyR = crate::BitReader;
#[doc = "Field `MCU_PLL_CLKSEL_BYP_WARM_RST_PROXY` writer - 23:23\\]
PLL bypass mode after warm reset."]
pub type McuPllClkselBypWarmRstProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_PLL_CLKSEL_BYPASS_SW_OVRD_PROXY` reader - 31:31\\]
PLL Bypass warm reset software override"]
pub type McuPllClkselBypassSwOvrdProxyR = crate::BitReader;
#[doc = "Field `MCU_PLL_CLKSEL_BYPASS_SW_OVRD_PROXY` writer - 31:31\\]
PLL Bypass warm reset software override"]
pub type McuPllClkselBypassSwOvrdProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
    #[inline(always)]
    pub fn mcu_pll_clksel_clkloss_swtch_en_proxy(&self) -> McuPllClkselClklossSwtchEnProxyR {
        McuPllClkselClklossSwtchEnProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    pub fn mcu_pll_clksel_byp_warm_rst_proxy(&self) -> McuPllClkselBypWarmRstProxyR {
        McuPllClkselBypWarmRstProxyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    pub fn mcu_pll_clksel_bypass_sw_ovrd_proxy(&self) -> McuPllClkselBypassSwOvrdProxyR {
        McuPllClkselBypassSwOvrdProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pll_clksel_clkloss_swtch_en_proxy(
        &mut self,
    ) -> McuPllClkselClklossSwtchEnProxyW<Cfg0McuPllClkselProxySpec> {
        McuPllClkselClklossSwtchEnProxyW::new(self, 8)
    }
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pll_clksel_byp_warm_rst_proxy(
        &mut self,
    ) -> McuPllClkselBypWarmRstProxyW<Cfg0McuPllClkselProxySpec> {
        McuPllClkselBypWarmRstProxyW::new(self, 23)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pll_clksel_bypass_sw_ovrd_proxy(
        &mut self,
    ) -> McuPllClkselBypassSwOvrdProxyW<Cfg0McuPllClkselProxySpec> {
        McuPllClkselBypassSwOvrdProxyW::new(self, 31)
    }
}
#[doc = "CFG0_MCU_PLL_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_pll_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_pll_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuPllClkselProxySpec;
impl crate::RegisterSpec for Cfg0McuPllClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_pll_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuPllClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_pll_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuPllClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_PLL_CLKSEL_PROXY to value 0x0080_0000"]
impl crate::Resettable for Cfg0McuPllClkselProxySpec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
