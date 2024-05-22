#[doc = "Register `CFG0_EMMC0_CLKSEL` reader"]
pub type R = crate::R<Cfg0Emmc0ClkselSpec>;
#[doc = "Register `CFG0_EMMC0_CLKSEL` writer"]
pub type W = crate::W<Cfg0Emmc0ClkselSpec>;
#[doc = "Field `EMMC0_CLKSEL_EMMCSD0_REFCLK_SEL` reader - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc0ClkselEmmcsd0RefclkSelR = crate::BitReader;
#[doc = "Field `EMMC0_CLKSEL_EMMCSD0_REFCLK_SEL` writer - 0:0\\]
eMMC XIN_CLK selection"]
pub type Emmc0ClkselEmmcsd0RefclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    pub fn emmc0_clksel_emmcsd0_refclk_sel(&self) -> Emmc0ClkselEmmcsd0RefclkSelR {
        Emmc0ClkselEmmcsd0RefclkSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
eMMC XIN_CLK selection"]
    #[inline(always)]
    #[must_use]
    pub fn emmc0_clksel_emmcsd0_refclk_sel(
        &mut self,
    ) -> Emmc0ClkselEmmcsd0RefclkSelW<Cfg0Emmc0ClkselSpec> {
        Emmc0ClkselEmmcsd0RefclkSelW::new(self, 0)
    }
}
#[doc = "CFG0_EMMC0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_emmc0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_emmc0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Emmc0ClkselSpec;
impl crate::RegisterSpec for Cfg0Emmc0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_emmc0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Emmc0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_emmc0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Emmc0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EMMC0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Emmc0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
