#[doc = "Register `CFG0_MAIN_PLL12_CLKSEL` reader"]
pub type R = crate::R<Cfg0MainPll12ClkselSpec>;
#[doc = "Register `CFG0_MAIN_PLL12_CLKSEL` writer"]
pub type W = crate::W<Cfg0MainPll12ClkselSpec>;
#[doc = "Field `MAIN_PLL12_CLKSEL_BYP_WARM_RST` reader - 23:23\\]
PLL bypass mode after warm reset."]
pub type MainPll12ClkselBypWarmRstR = crate::BitReader;
#[doc = "Field `MAIN_PLL12_CLKSEL_BYP_WARM_RST` writer - 23:23\\]
PLL bypass mode after warm reset."]
pub type MainPll12ClkselBypWarmRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_PLL12_CLKSEL_BYPASS_SW_OVRD` reader - 31:31\\]
PLL Bypass warm reset software override"]
pub type MainPll12ClkselBypassSwOvrdR = crate::BitReader;
#[doc = "Field `MAIN_PLL12_CLKSEL_BYPASS_SW_OVRD` writer - 31:31\\]
PLL Bypass warm reset software override"]
pub type MainPll12ClkselBypassSwOvrdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    pub fn main_pll12_clksel_byp_warm_rst(&self) -> MainPll12ClkselBypWarmRstR {
        MainPll12ClkselBypWarmRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    pub fn main_pll12_clksel_bypass_sw_ovrd(&self) -> MainPll12ClkselBypassSwOvrdR {
        MainPll12ClkselBypassSwOvrdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    #[must_use]
    pub fn main_pll12_clksel_byp_warm_rst(
        &mut self,
    ) -> MainPll12ClkselBypWarmRstW<Cfg0MainPll12ClkselSpec> {
        MainPll12ClkselBypWarmRstW::new(self, 23)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    #[must_use]
    pub fn main_pll12_clksel_bypass_sw_ovrd(
        &mut self,
    ) -> MainPll12ClkselBypassSwOvrdW<Cfg0MainPll12ClkselSpec> {
        MainPll12ClkselBypassSwOvrdW::new(self, 31)
    }
}
#[doc = "CFG0_MAIN_PLL12_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_pll12_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_pll12_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainPll12ClkselSpec;
impl crate::RegisterSpec for Cfg0MainPll12ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_pll12_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0MainPll12ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_pll12_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0MainPll12ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_PLL12_CLKSEL to value 0x0080_0000"]
impl crate::Resettable for Cfg0MainPll12ClkselSpec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
