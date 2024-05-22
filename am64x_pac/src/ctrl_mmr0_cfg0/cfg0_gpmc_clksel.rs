#[doc = "Register `CFG0_GPMC_CLKSEL` reader"]
pub type R = crate::R<Cfg0GpmcClkselSpec>;
#[doc = "Register `CFG0_GPMC_CLKSEL` writer"]
pub type W = crate::W<Cfg0GpmcClkselSpec>;
#[doc = "Field `GPMC_CLKSEL_CLK_SEL` reader - 0:0\\]
Selects the GPMC clock source"]
pub type GpmcClkselClkSelR = crate::BitReader;
#[doc = "Field `GPMC_CLKSEL_CLK_SEL` writer - 0:0\\]
Selects the GPMC clock source"]
pub type GpmcClkselClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the GPMC clock source"]
    #[inline(always)]
    pub fn gpmc_clksel_clk_sel(&self) -> GpmcClkselClkSelR {
        GpmcClkselClkSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the GPMC clock source"]
    #[inline(always)]
    #[must_use]
    pub fn gpmc_clksel_clk_sel(&mut self) -> GpmcClkselClkSelW<Cfg0GpmcClkselSpec> {
        GpmcClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_GPMC_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gpmc_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gpmc_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GpmcClkselSpec;
impl crate::RegisterSpec for Cfg0GpmcClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gpmc_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0GpmcClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gpmc_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0GpmcClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GPMC_CLKSEL to value 0"]
impl crate::Resettable for Cfg0GpmcClkselSpec {
    const RESET_VALUE: u32 = 0;
}
