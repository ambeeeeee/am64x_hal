#[doc = "Register `CFG0_WWD0_CLKSEL` reader"]
pub type R = crate::R<Cfg0Wwd0ClkselSpec>;
#[doc = "Register `CFG0_WWD0_CLKSEL` writer"]
pub type W = crate::W<Cfg0Wwd0ClkselSpec>;
#[doc = "Field `WWD0_CLKSEL_CLK_SEL` reader - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd0ClkselClkSelR = crate::FieldReader;
#[doc = "Field `WWD0_CLKSEL_CLK_SEL` writer - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
pub type Wwd0ClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WWD0_CLKSEL_WRTLOCK` reader - 31:31\\]
When set, locks WWD0_CLKSEL from further writes until the next module reset."]
pub type Wwd0ClkselWrtlockR = crate::BitReader;
#[doc = "Field `WWD0_CLKSEL_WRTLOCK` writer - 31:31\\]
When set, locks WWD0_CLKSEL from further writes until the next module reset."]
pub type Wwd0ClkselWrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    pub fn wwd0_clksel_clk_sel(&self) -> Wwd0ClkselClkSelR {
        Wwd0ClkselClkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD0_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    pub fn wwd0_clksel_wrtlock(&self) -> Wwd0ClkselWrtlockR {
        Wwd0ClkselWrtlockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Windowed watchdog timer functional clock input select mux control"]
    #[inline(always)]
    #[must_use]
    pub fn wwd0_clksel_clk_sel(&mut self) -> Wwd0ClkselClkSelW<Cfg0Wwd0ClkselSpec> {
        Wwd0ClkselClkSelW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set, locks WWD0_CLKSEL from further writes until the next module reset."]
    #[inline(always)]
    #[must_use]
    pub fn wwd0_clksel_wrtlock(&mut self) -> Wwd0ClkselWrtlockW<Cfg0Wwd0ClkselSpec> {
        Wwd0ClkselWrtlockW::new(self, 31)
    }
}
#[doc = "CFG0_WWD0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_wwd0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_wwd0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Wwd0ClkselSpec;
impl crate::RegisterSpec for Cfg0Wwd0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_wwd0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Wwd0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_wwd0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Wwd0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_WWD0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Wwd0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
