#[doc = "Register `CFG0_MCU_PLL_CLKSEL` reader"]
pub type R = crate::R<Cfg0McuPllClkselSpec>;
#[doc = "Register `CFG0_MCU_PLL_CLKSEL` writer"]
pub type W = crate::W<Cfg0McuPllClkselSpec>;
#[doc = "Field `MCU_PLL_CLKSEL_CLKLOSS_SWTCH_EN` reader - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
pub type McuPllClkselClklossSwtchEnR = crate::BitReader;
#[doc = "Field `MCU_PLL_CLKSEL_CLKLOSS_SWTCH_EN` writer - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
pub type McuPllClkselClklossSwtchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_PLL_CLKSEL_BYP_WARM_RST` reader - 23:23\\]
PLL bypass mode after warm reset."]
pub type McuPllClkselBypWarmRstR = crate::BitReader;
#[doc = "Field `MCU_PLL_CLKSEL_BYP_WARM_RST` writer - 23:23\\]
PLL bypass mode after warm reset."]
pub type McuPllClkselBypWarmRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_PLL_CLKSEL_BYPASS_SW_OVRD` reader - 31:31\\]
PLL Bypass warm reset software override"]
pub type McuPllClkselBypassSwOvrdR = crate::BitReader;
#[doc = "Field `MCU_PLL_CLKSEL_BYPASS_SW_OVRD` writer - 31:31\\]
PLL Bypass warm reset software override"]
pub type McuPllClkselBypassSwOvrdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
    #[inline(always)]
    pub fn mcu_pll_clksel_clkloss_swtch_en(&self) -> McuPllClkselClklossSwtchEnR {
        McuPllClkselClklossSwtchEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    pub fn mcu_pll_clksel_byp_warm_rst(&self) -> McuPllClkselBypWarmRstR {
        McuPllClkselBypWarmRstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    pub fn mcu_pll_clksel_bypass_sw_ovrd(&self) -> McuPllClkselBypassSwOvrdR {
        McuPllClkselBypassSwOvrdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
When set, actives automatic switching of MCU PLL\\[2:0\\]
clock source to CLK_12M_RC if HFOSC0 clock loss is detected"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pll_clksel_clkloss_swtch_en(
        &mut self,
    ) -> McuPllClkselClklossSwtchEnW<Cfg0McuPllClkselSpec> {
        McuPllClkselClklossSwtchEnW::new(self, 8)
    }
    #[doc = "Bit 23 - 23:23\\]
PLL bypass mode after warm reset."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pll_clksel_byp_warm_rst(&mut self) -> McuPllClkselBypWarmRstW<Cfg0McuPllClkselSpec> {
        McuPllClkselBypWarmRstW::new(self, 23)
    }
    #[doc = "Bit 31 - 31:31\\]
PLL Bypass warm reset software override"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_pll_clksel_bypass_sw_ovrd(
        &mut self,
    ) -> McuPllClkselBypassSwOvrdW<Cfg0McuPllClkselSpec> {
        McuPllClkselBypassSwOvrdW::new(self, 31)
    }
}
#[doc = "CFG0_MCU_PLL_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_pll_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_pll_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuPllClkselSpec;
impl crate::RegisterSpec for Cfg0McuPllClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_pll_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0McuPllClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_pll_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0McuPllClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_PLL_CLKSEL to value 0x0080_0000"]
impl crate::Resettable for Cfg0McuPllClkselSpec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
