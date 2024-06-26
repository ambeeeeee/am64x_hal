#[doc = "Register `CFG0_WWD8_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Wwd8ClkselProxySpec>;
#[doc = "Register `CFG0_WWD8_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Wwd8ClkselProxySpec>;
#[doc = "Field `WWD8_CLKSEL_CLK_SEL_PROXY` reader - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd8ClkselClkSelProxyR = crate::FieldReader;
#[doc = "Field `WWD8_CLKSEL_CLK_SEL_PROXY` writer - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd8ClkselClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WWD8_CLKSEL_WRTLOCK_PROXY` reader - 31:31\\]
When set, locks WWD8_CLKSEL from further writes until the next module reset."]
pub type Wwd8ClkselWrtlockProxyR = crate::BitReader;
#[doc = "Field `WWD8_CLKSEL_WRTLOCK_PROXY` writer - 31:31\\]
When set, locks WWD8_CLKSEL from further writes until the next module reset."]
pub type Wwd8ClkselWrtlockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    pub fn wwd8_clksel_clk_sel_proxy(&self) -> Wwd8ClkselClkSelProxyR {
        Wwd8ClkselClkSelProxyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD8_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    pub fn wwd8_clksel_wrtlock_proxy(&self) -> Wwd8ClkselWrtlockProxyR {
        Wwd8ClkselWrtlockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    #[must_use]
    pub fn wwd8_clksel_clk_sel_proxy(&mut self) -> Wwd8ClkselClkSelProxyW<Cfg0Wwd8ClkselProxySpec> {
        Wwd8ClkselClkSelProxyW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD8_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    #[must_use]
    pub fn wwd8_clksel_wrtlock_proxy(
        &mut self,
    ) -> Wwd8ClkselWrtlockProxyW<Cfg0Wwd8ClkselProxySpec> {
        Wwd8ClkselWrtlockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_WWD8_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd8_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd8_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Wwd8ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Wwd8ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_wwd8_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Wwd8ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_wwd8_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Wwd8ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_WWD8_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Wwd8ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
