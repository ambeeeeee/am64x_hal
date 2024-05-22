#[doc = "Register `CFG0_WWD1_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Wwd1ClkselProxySpec>;
#[doc = "Register `CFG0_WWD1_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Wwd1ClkselProxySpec>;
#[doc = "Field `WWD1_CLKSEL_CLK_SEL_PROXY` reader - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd1ClkselClkSelProxyR = crate::FieldReader;
#[doc = "Field `WWD1_CLKSEL_CLK_SEL_PROXY` writer - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd1ClkselClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WWD1_CLKSEL_WRTLOCK_PROXY` reader - 31:31\\]
When set, locks WWD1_CLKSEL from further writes until the next module reset."]
pub type Wwd1ClkselWrtlockProxyR = crate::BitReader;
#[doc = "Field `WWD1_CLKSEL_WRTLOCK_PROXY` writer - 31:31\\]
When set, locks WWD1_CLKSEL from further writes until the next module reset."]
pub type Wwd1ClkselWrtlockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    pub fn wwd1_clksel_clk_sel_proxy(&self) -> Wwd1ClkselClkSelProxyR {
        Wwd1ClkselClkSelProxyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD1_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    pub fn wwd1_clksel_wrtlock_proxy(&self) -> Wwd1ClkselWrtlockProxyR {
        Wwd1ClkselWrtlockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    #[must_use]
    pub fn wwd1_clksel_clk_sel_proxy(&mut self) -> Wwd1ClkselClkSelProxyW<Cfg0Wwd1ClkselProxySpec> {
        Wwd1ClkselClkSelProxyW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD1_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    #[must_use]
    pub fn wwd1_clksel_wrtlock_proxy(
        &mut self,
    ) -> Wwd1ClkselWrtlockProxyW<Cfg0Wwd1ClkselProxySpec> {
        Wwd1ClkselWrtlockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_WWD1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd1_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd1_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Wwd1ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Wwd1ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_wwd1_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Wwd1ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_wwd1_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Wwd1ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_WWD1_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Wwd1ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
