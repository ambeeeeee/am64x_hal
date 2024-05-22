#[doc = "Register `CFG0_SPI1_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Spi1ClkselProxySpec>;
#[doc = "Register `CFG0_SPI1_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Spi1ClkselProxySpec>;
#[doc = "Field `SPI1_CLKSEL_MSTR_LB_CLKSEL_PROXY` reader - 16:16\\]
Controller mode receive capture clock loopback selection"]
pub type Spi1ClkselMstrLbClkselProxyR = crate::BitReader;
#[doc = "Field `SPI1_CLKSEL_MSTR_LB_CLKSEL_PROXY` writer - 16:16\\]
Controller mode receive capture clock loopback selection"]
pub type Spi1ClkselMstrLbClkselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - 16:16\\]
Controller mode receive capture clock loopback selection"]
    #[inline(always)]
    pub fn spi1_clksel_mstr_lb_clksel_proxy(&self) -> Spi1ClkselMstrLbClkselProxyR {
        Spi1ClkselMstrLbClkselProxyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - 16:16\\]
Controller mode receive capture clock loopback selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_clksel_mstr_lb_clksel_proxy(
        &mut self,
    ) -> Spi1ClkselMstrLbClkselProxyW<Cfg0Spi1ClkselProxySpec> {
        Spi1ClkselMstrLbClkselProxyW::new(self, 16)
    }
}
#[doc = "CFG0_SPI1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi1_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi1_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spi1ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Spi1ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_spi1_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Spi1ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_spi1_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Spi1ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SPI1_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Spi1ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
