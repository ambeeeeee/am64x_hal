#[doc = "Register `CFG0_EMMC1_CLKSEL` reader"]
pub type R = crate::R<Cfg0Emmc1ClkselSpec>;
#[doc = "Register `CFG0_EMMC1_CLKSEL` writer"]
pub type W = crate::W<Cfg0Emmc1ClkselSpec>;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_REFCLK_SEL` reader - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc1ClkselEmmcsd1RefclkSelR = crate::BitReader;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_REFCLK_SEL` writer - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc1ClkselEmmcsd1RefclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_IO_CLKLB_SEL` reader - 16:16\\]
eMMC IO Clock Selection:"]
pub type Emmc1ClkselEmmcsd1IoClklbSelR = crate::BitReader;
#[doc = "Field `EMMC1_CLKSEL_EMMCSD1_IO_CLKLB_SEL` writer - 16:16\\]
eMMC IO Clock Selection:"]
pub type Emmc1ClkselEmmcsd1IoClklbSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    pub fn emmc1_clksel_emmcsd1_refclk_sel(&self) -> Emmc1ClkselEmmcsd1RefclkSelR {
        Emmc1ClkselEmmcsd1RefclkSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
eMMC IO Clock Selection:"]
    #[inline(always)]
    pub fn emmc1_clksel_emmcsd1_io_clklb_sel(&self) -> Emmc1ClkselEmmcsd1IoClklbSelR {
        Emmc1ClkselEmmcsd1IoClklbSelR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    #[must_use]
    pub fn emmc1_clksel_emmcsd1_refclk_sel(
        &mut self,
    ) -> Emmc1ClkselEmmcsd1RefclkSelW<Cfg0Emmc1ClkselSpec> {
        Emmc1ClkselEmmcsd1RefclkSelW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
eMMC IO Clock Selection:"]
    #[inline(always)]
    #[must_use]
    pub fn emmc1_clksel_emmcsd1_io_clklb_sel(
        &mut self,
    ) -> Emmc1ClkselEmmcsd1IoClklbSelW<Cfg0Emmc1ClkselSpec> {
        Emmc1ClkselEmmcsd1IoClklbSelW::new(self, 16)
    }
}
#[doc = "CFG0_EMMC1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc1_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc1_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Emmc1ClkselSpec;
impl crate::RegisterSpec for Cfg0Emmc1ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_emmc1_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Emmc1ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_emmc1_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Emmc1ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EMMC1_CLKSEL to value 0x01"]
impl crate::Resettable for Cfg0Emmc1ClkselSpec {
    const RESET_VALUE: u32 = 0x01;
}
