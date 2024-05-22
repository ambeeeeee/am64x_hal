#[doc = "Register `CFG0_MAIN_PLL0_CLKSEL` reader"]
pub type R = crate::R<Cfg0MainPll0ClkselSpec>;
#[doc = "Register `CFG0_MAIN_PLL0_CLKSEL` writer"]
pub type W = crate::W<Cfg0MainPll0ClkselSpec>;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYP_WARM_RST` reader - 23:23\\]
PLL bypass mode after warm reset."]
pub type MainPll0ClkselBypWarmRstR = crate::BitReader;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYP_WARM_RST` writer - 23:23\\]
PLL bypass mode after warm reset."]
pub type MainPll0ClkselBypWarmRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYPASS_SW_OVRD` reader - 31:31\\]
PLL Bypass warm reset software override"]
pub type MainPll0ClkselBypassSwOvrdR = crate::BitReader;
#[doc = "Field `MAIN_PLL0_CLKSEL_BYPASS_SW_OVRD` writer - 31:31\\]
PLL Bypass warm reset software override"]
pub type MainPll0ClkselBypassSwOvrdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    pub fn main_pll0_clksel_byp_warm_rst(&self) -> MainPll0ClkselBypWarmRstR {
        MainPll0ClkselBypWarmRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    pub fn main_pll0_clksel_bypass_sw_ovrd(&self) -> MainPll0ClkselBypassSwOvrdR {
        MainPll0ClkselBypassSwOvrdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    #[must_use]
    pub fn main_pll0_clksel_byp_warm_rst(
        &mut self,
    ) -> MainPll0ClkselBypWarmRstW<Cfg0MainPll0ClkselSpec> {
        MainPll0ClkselBypWarmRstW::new(self, 23)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    #[must_use]
    pub fn main_pll0_clksel_bypass_sw_ovrd(
        &mut self,
    ) -> MainPll0ClkselBypassSwOvrdW<Cfg0MainPll0ClkselSpec> {
        MainPll0ClkselBypassSwOvrdW::new(self, 31)
    }
}
#[doc = "CFG0_MAIN_PLL0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainPll0ClkselSpec;
impl crate::RegisterSpec for Cfg0MainPll0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_pll0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0MainPll0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_pll0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0MainPll0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_PLL0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0MainPll0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
