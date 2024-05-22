#[doc = "Register `CFG0_MCU_SPI0_CLKSEL` reader"]
pub type R = crate::R<Cfg0McuSpi0ClkselSpec>;
#[doc = "Register `CFG0_MCU_SPI0_CLKSEL` writer"]
pub type W = crate::W<Cfg0McuSpi0ClkselSpec>;
#[doc = "Field `MCU_SPI0_CLKSEL_MSTR_LB_CLKSEL` reader - 16:16\\]
Controller mode receive capture clock loopback selection"]
pub type McuSpi0ClkselMstrLbClkselR = crate::BitReader;
#[doc = "Field `MCU_SPI0_CLKSEL_MSTR_LB_CLKSEL` writer - 16:16\\]
Controller mode receive capture clock loopback selection"]
pub type McuSpi0ClkselMstrLbClkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - 16:16\\]
Controller mode receive capture clock loopback selection"]
    #[inline(always)]
    pub fn mcu_spi0_clksel_mstr_lb_clksel(&self) -> McuSpi0ClkselMstrLbClkselR {
        McuSpi0ClkselMstrLbClkselR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - 16:16\\]
Controller mode receive capture clock loopback selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_spi0_clksel_mstr_lb_clksel(
        &mut self,
    ) -> McuSpi0ClkselMstrLbClkselW<Cfg0McuSpi0ClkselSpec> {
        McuSpi0ClkselMstrLbClkselW::new(self, 16)
    }
}
#[doc = "CFG0_MCU_SPI0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_spi0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_spi0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuSpi0ClkselSpec;
impl crate::RegisterSpec for Cfg0McuSpi0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_spi0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0McuSpi0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_spi0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0McuSpi0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_SPI0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0McuSpi0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
