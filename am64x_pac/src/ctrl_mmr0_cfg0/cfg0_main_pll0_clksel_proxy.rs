#[doc = "Register `CFG0_MAIN_PLL0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0MainPll0ClkselProxySpec>;
#[doc = "Register `CFG0_MAIN_PLL0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0MainPll0ClkselProxySpec>;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYP_WARM_RST_PROXY` reader - 23:23\\]
PLL bypass mode after warm reset."]
pub type MainPll0ClkselBypWarmRstProxyR = crate::BitReader;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYP_WARM_RST_PROXY` writer - 23:23\\]
PLL bypass mode after warm reset."]
pub type MainPll0ClkselBypWarmRstProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYPASS_SW_OVRD_PROXY` reader - 31:31\\]
PLL Bypass warm reset software override"]
pub type MainPll0ClkselBypassSwOvrdProxyR = crate::BitReader;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYPASS_SW_OVRD_PROXY` writer - 31:31\\]
PLL Bypass warm reset software override"]
pub type MainPll0ClkselBypassSwOvrdProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    pub fn main_pll0_clksel_byp_warm_rst_proxy(&self) -> MainPll0ClkselBypWarmRstProxyR {
        MainPll0ClkselBypWarmRstProxyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    pub fn main_pll0_clksel_bypass_sw_ovrd_proxy(&self) -> MainPll0ClkselBypassSwOvrdProxyR {
        MainPll0ClkselBypassSwOvrdProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    #[must_use]
    pub fn main_pll0_clksel_byp_warm_rst_proxy(
        &mut self,
    ) -> MainPll0ClkselBypWarmRstProxyW<Cfg0MainPll0ClkselProxySpec> {
        MainPll0ClkselBypWarmRstProxyW::new(self, 23)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    #[must_use]
    pub fn main_pll0_clksel_bypass_sw_ovrd_proxy(
        &mut self,
    ) -> MainPll0ClkselBypassSwOvrdProxyW<Cfg0MainPll0ClkselProxySpec> {
        MainPll0ClkselBypassSwOvrdProxyW::new(self, 31)
    }
}
#[doc = "CFG0_MAIN_PLL0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainPll0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0MainPll0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_pll0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0MainPll0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_pll0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0MainPll0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_PLL0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0MainPll0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
