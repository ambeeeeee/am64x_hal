#[doc = "Register `CFG0_SPI3_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Spi3ClkselProxySpec>;
#[doc = "Register `CFG0_SPI3_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Spi3ClkselProxySpec>;
#[doc = "Field `SPI3_CLKSEL_MSTR_LB_CLKSEL_PROXY` reader - 16:16\\]
Controller mode receive capture clock loopback selection"]
pub type Spi3ClkselMstrLbClkselProxyR = crate::BitReader;
#[doc = "Field `SPI3_CLKSEL_MSTR_LB_CLKSEL_PROXY` writer - 16:16\\]
Controller mode receive capture clock loopback selection"]
pub type Spi3ClkselMstrLbClkselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - 16:16\\]
Controller mode receive capture clock loopback selection"]
    #[inline(always)]
    pub fn spi3_clksel_mstr_lb_clksel_proxy(&self) -> Spi3ClkselMstrLbClkselProxyR {
        Spi3ClkselMstrLbClkselProxyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - 16:16\\]
Controller mode receive capture clock loopback selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi3_clksel_mstr_lb_clksel_proxy(
        &mut self,
    ) -> Spi3ClkselMstrLbClkselProxyW<Cfg0Spi3ClkselProxySpec> {
        Spi3ClkselMstrLbClkselProxyW::new(self, 16)
    }
}
#[doc = "CFG0_SPI3_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_spi3_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_spi3_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spi3ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Spi3ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_spi3_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Spi3ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_spi3_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Spi3ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SPI3_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Spi3ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
