#[doc = "Register `CFG0_OSPI0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Ospi0ClkselProxySpec>;
#[doc = "Register `CFG0_OSPI0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Ospi0ClkselProxySpec>;
#[doc = "Field `OSPI0_CLKSEL_CLK_SEL_PROXY` reader - 0:0\\]
OSPI0 reference clock selection"]
pub type Ospi0ClkselClkSelProxyR = crate::BitReader;
#[doc = "Field `OSPI0_CLKSEL_CLK_SEL_PROXY` writer - 0:0\\]
OSPI0 reference clock selection"]
pub type Ospi0ClkselClkSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSPI0_CLKSEL_LOOPCLK_SEL_PROXY` reader - 4:4\\]
OBSPI0 Loopback clock source"]
pub type Ospi0ClkselLoopclkSelProxyR = crate::BitReader;
#[doc = "Field `OSPI0_CLKSEL_LOOPCLK_SEL_PROXY` writer - 4:4\\]
OBSPI0 Loopback clock source"]
pub type Ospi0ClkselLoopclkSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
OSPI0 reference clock selection"]
    #[inline(always)]
    pub fn ospi0_clksel_clk_sel_proxy(&self) -> Ospi0ClkselClkSelProxyR {
        Ospi0ClkselClkSelProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
OBSPI0 Loopback clock source"]
    #[inline(always)]
    pub fn ospi0_clksel_loopclk_sel_proxy(&self) -> Ospi0ClkselLoopclkSelProxyR {
        Ospi0ClkselLoopclkSelProxyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
OSPI0 reference clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn ospi0_clksel_clk_sel_proxy(
        &mut self,
    ) -> Ospi0ClkselClkSelProxyW<Cfg0Ospi0ClkselProxySpec> {
        Ospi0ClkselClkSelProxyW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
OBSPI0 Loopback clock source"]
    #[inline(always)]
    #[must_use]
    pub fn ospi0_clksel_loopclk_sel_proxy(
        &mut self,
    ) -> Ospi0ClkselLoopclkSelProxyW<Cfg0Ospi0ClkselProxySpec> {
        Ospi0ClkselLoopclkSelProxyW::new(self, 4)
    }
}
#[doc = "CFG0_OSPI0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ospi0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ospi0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Ospi0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Ospi0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_ospi0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Ospi0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_ospi0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Ospi0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_OSPI0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Ospi0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
