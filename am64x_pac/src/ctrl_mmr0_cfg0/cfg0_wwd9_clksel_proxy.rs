#[doc = "Register `CFG0_WWD9_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Wwd9ClkselProxySpec>;
#[doc = "Register `CFG0_WWD9_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Wwd9ClkselProxySpec>;
#[doc = "Field `WWD9_CLKSEL_CLK_SEL_PROXY` reader - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd9ClkselClkSelProxyR = crate::FieldReader;
#[doc = "Field `WWD9_CLKSEL_CLK_SEL_PROXY` writer - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd9ClkselClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WWD9_CLKSEL_WRTLOCK_PROXY` reader - 31:31\\]
When set, locks WWD9_CLKSEL from further writes until the next module reset."]
pub type Wwd9ClkselWrtlockProxyR = crate::BitReader;
#[doc = "Field `WWD9_CLKSEL_WRTLOCK_PROXY` writer - 31:31\\]
When set, locks WWD9_CLKSEL from further writes until the next module reset."]
pub type Wwd9ClkselWrtlockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    pub fn wwd9_clksel_clk_sel_proxy(&self) -> Wwd9ClkselClkSelProxyR {
        Wwd9ClkselClkSelProxyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD9_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    pub fn wwd9_clksel_wrtlock_proxy(&self) -> Wwd9ClkselWrtlockProxyR {
        Wwd9ClkselWrtlockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    #[must_use]
    pub fn wwd9_clksel_clk_sel_proxy(&mut self) -> Wwd9ClkselClkSelProxyW<Cfg0Wwd9ClkselProxySpec> {
        Wwd9ClkselClkSelProxyW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD9_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    #[must_use]
    pub fn wwd9_clksel_wrtlock_proxy(
        &mut self,
    ) -> Wwd9ClkselWrtlockProxyW<Cfg0Wwd9ClkselProxySpec> {
        Wwd9ClkselWrtlockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_WWD9_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd9_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd9_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Wwd9ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Wwd9ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_wwd9_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Wwd9ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_wwd9_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Wwd9ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_WWD9_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Wwd9ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
