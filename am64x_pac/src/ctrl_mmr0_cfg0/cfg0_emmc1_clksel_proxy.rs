#[doc = "Register `CFG0_EMMC1_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Emmc1ClkselProxySpec>;
#[doc = "Register `CFG0_EMMC1_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Emmc1ClkselProxySpec>;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_REFCLK_SEL_PROXY` reader - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc1ClkselEmmcsd1RefclkSelProxyR = crate::BitReader;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_REFCLK_SEL_PROXY` writer - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc1ClkselEmmcsd1RefclkSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_IO_CLKLB_SEL_PROXY` reader - 16:16\\]
eMMC IO Clock Selection:"]
pub type Emmc1ClkselEmmcsd1IoClklbSelProxyR = crate::BitReader;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_IO_CLKLB_SEL_PROXY` writer - 16:16\\]
eMMC IO Clock Selection:"]
pub type Emmc1ClkselEmmcsd1IoClklbSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    pub fn emmc1_clksel_emmcsd1_refclk_sel_proxy(&self) -> Emmc1ClkselEmmcsd1RefclkSelProxyR {
        Emmc1ClkselEmmcsd1RefclkSelProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
eMMC IO Clock Selection:"]
    #[inline(always)]
    pub fn emmc1_clksel_emmcsd1_io_clklb_sel_proxy(&self) -> Emmc1ClkselEmmcsd1IoClklbSelProxyR {
        Emmc1ClkselEmmcsd1IoClklbSelProxyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    #[must_use]
    pub fn emmc1_clksel_emmcsd1_refclk_sel_proxy(
        &mut self,
    ) -> Emmc1ClkselEmmcsd1RefclkSelProxyW<Cfg0Emmc1ClkselProxySpec> {
        Emmc1ClkselEmmcsd1RefclkSelProxyW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
eMMC IO Clock Selection:"]
    #[inline(always)]
    #[must_use]
    pub fn emmc1_clksel_emmcsd1_io_clklb_sel_proxy(
        &mut self,
    ) -> Emmc1ClkselEmmcsd1IoClklbSelProxyW<Cfg0Emmc1ClkselProxySpec> {
        Emmc1ClkselEmmcsd1IoClklbSelProxyW::new(self, 16)
    }
}
#[doc = "CFG0_EMMC1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc1_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc1_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Emmc1ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Emmc1ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_emmc1_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Emmc1ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_emmc1_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Emmc1ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EMMC1_CLKSEL_PROXY to value 0x01"]
impl crate::Resettable for Cfg0Emmc1ClkselProxySpec {
    const RESET_VALUE: u32 = 0x01;
}
