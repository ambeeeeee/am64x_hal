#[doc = "Register `CFG0_MCAN0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Mcan0ClkselProxySpec>;
#[doc = "Register `CFG0_MCAN0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Mcan0ClkselProxySpec>;
#[doc = "Field `MCAN0_CLKSEL_CLK_SEL_PROXY` reader - 1:0\\]
MAIN MCAN_CLK selection"]
pub type Mcan0ClkselClkSelProxyR = crate::FieldReader;
#[doc = "Field `MCAN0_CLKSEL_CLK_SEL_PROXY` writer - 1:0\\]
MAIN MCAN_CLK selection"]
pub type Mcan0ClkselClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MAIN MCAN_CLK selection"]
    #[inline(always)]
    pub fn mcan0_clksel_clk_sel_proxy(&self) -> Mcan0ClkselClkSelProxyR {
        Mcan0ClkselClkSelProxyR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MAIN MCAN_CLK selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcan0_clksel_clk_sel_proxy(
        &mut self,
    ) -> Mcan0ClkselClkSelProxyW<Cfg0Mcan0ClkselProxySpec> {
        Mcan0ClkselClkSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_MCAN0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcan0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcan0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Mcan0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Mcan0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcan0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Mcan0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcan0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Mcan0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCAN0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Mcan0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
