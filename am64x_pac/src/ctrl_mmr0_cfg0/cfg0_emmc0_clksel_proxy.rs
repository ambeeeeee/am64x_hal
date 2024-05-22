#[doc = "Register `CFG0_EMMC0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Emmc0ClkselProxySpec>;
#[doc = "Register `CFG0_EMMC0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Emmc0ClkselProxySpec>;
#[doc = "Field `EMMC0_CLKSEL_EMMCSD0_REFCLK_SEL_PROXY` reader - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc0ClkselEmmcsd0RefclkSelProxyR = crate::BitReader;
#[doc = "Field `EMMC0_CLKSEL_EMMCSD0_REFCLK_SEL_PROXY` writer - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc0ClkselEmmcsd0RefclkSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    pub fn emmc0_clksel_emmcsd0_refclk_sel_proxy(&self) -> Emmc0ClkselEmmcsd0RefclkSelProxyR {
        Emmc0ClkselEmmcsd0RefclkSelProxyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    #[must_use]
    pub fn emmc0_clksel_emmcsd0_refclk_sel_proxy(
        &mut self,
    ) -> Emmc0ClkselEmmcsd0RefclkSelProxyW<Cfg0Emmc0ClkselProxySpec> {
        Emmc0ClkselEmmcsd0RefclkSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_EMMC0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Emmc0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Emmc0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_emmc0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Emmc0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_emmc0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Emmc0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EMMC0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Emmc0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
