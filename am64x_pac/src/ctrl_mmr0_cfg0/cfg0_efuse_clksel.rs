#[doc = "Register `CFG0_EFUSE_CLKSEL` reader"]
pub type R = crate::R<Cfg0EfuseClkselSpec>;
#[doc = "Register `CFG0_EFUSE_CLKSEL` writer"]
pub type W = crate::W<Cfg0EfuseClkselSpec>;
#[doc = "Field `EFUSE_CLKSEL_CLK_SEL` reader - 0:0\\]
Selects the clock source"]
pub type EfuseClkselClkSelR = crate::BitReader;
#[doc = "Field `EFUSE_CLKSEL_CLK_SEL` writer - 0:0\\]
Selects the clock source"]
pub type EfuseClkselClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source"]
    #[inline(always)]
    pub fn efuse_clksel_clk_sel(&self) -> EfuseClkselClkSelR {
        EfuseClkselClkSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_clksel_clk_sel(&mut self) -> EfuseClkselClkSelW<Cfg0EfuseClkselSpec> {
        EfuseClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_EFUSE_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_efuse_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_efuse_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EfuseClkselSpec;
impl crate::RegisterSpec for Cfg0EfuseClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_efuse_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0EfuseClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_efuse_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0EfuseClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EFUSE_CLKSEL to value 0"]
impl crate::Resettable for Cfg0EfuseClkselSpec {
    const RESET_VALUE: u32 = 0;
}
