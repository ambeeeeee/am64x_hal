#[doc = "Register `CFG0_OSPI0_CLKSEL` reader"]
pub type R = crate::R<Cfg0Ospi0ClkselSpec>;
#[doc = "Register `CFG0_OSPI0_CLKSEL` writer"]
pub type W = crate::W<Cfg0Ospi0ClkselSpec>;
#[doc = "Field `OSPI0_CLKSEL_CLK_SEL` reader - 0:0\\]
OSPI0 reference clock selection"]
pub type Ospi0ClkselClkSelR = crate::BitReader;
#[doc = "Field `OSPI0_CLKSEL_CLK_SEL` writer - 0:0\\]
OSPI0 reference clock selection"]
pub type Ospi0ClkselClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPI0_CLKSEL_LOOPCLK_SEL` reader - 4:4\\]
OBSPI0 Loopback clock source"]
pub type Ospi0ClkselLoopclkSelR = crate::BitReader;
#[doc = "Field `OSPI0_CLKSEL_LOOPCLK_SEL` writer - 4:4\\]
OBSPI0 Loopback clock source"]
pub type Ospi0ClkselLoopclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
OSPI0 reference clock selection"]
    #[inline(always)]
    pub fn ospi0_clksel_clk_sel(&self) -> Ospi0ClkselClkSelR {
        Ospi0ClkselClkSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
OBSPI0 Loopback clock source"]
    #[inline(always)]
    pub fn ospi0_clksel_loopclk_sel(&self) -> Ospi0ClkselLoopclkSelR {
        Ospi0ClkselLoopclkSelR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
OSPI0 reference clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ospi0_clksel_clk_sel(&mut self) -> Ospi0ClkselClkSelW<Cfg0Ospi0ClkselSpec> {
        Ospi0ClkselClkSelW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
OBSPI0 Loopback clock source"]
    #[inline(always)]
    #[must_use]
    pub fn ospi0_clksel_loopclk_sel(&mut self) -> Ospi0ClkselLoopclkSelW<Cfg0Ospi0ClkselSpec> {
        Ospi0ClkselLoopclkSelW::new(self, 4)
    }
}
#[doc = "CFG0_OSPI0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ospi0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ospi0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Ospi0ClkselSpec;
impl crate::RegisterSpec for Cfg0Ospi0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_ospi0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Ospi0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_ospi0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Ospi0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_OSPI0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Ospi0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
