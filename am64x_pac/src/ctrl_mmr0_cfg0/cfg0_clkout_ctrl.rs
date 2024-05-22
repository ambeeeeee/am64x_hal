#[doc = "Register `CFG0_CLKOUT_CTRL` reader"]
pub type R = crate::R<Cfg0ClkoutCtrlSpec>;
#[doc = "Register `CFG0_CLKOUT_CTRL` writer"]
pub type W = crate::W<Cfg0ClkoutCtrlSpec>;
#[doc = "Field `CLKOUT_CTRL_CLK_SEL` reader - 0:0\\]
Selects CLKOUT clock source"]
pub type ClkoutCtrlClkSelR = crate::BitReader;
#[doc = "Field `CLKOUT_CTRL_CLK_SEL` writer - 0:0\\]
Selects CLKOUT clock source"]
pub type ClkoutCtrlClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT_CTRL_CLK_EN` reader - 4:4\\]
When set, activates CLKOUT output"]
pub type ClkoutCtrlClkEnR = crate::BitReader;
#[doc = "Field `CLKOUT_CTRL_CLK_EN` writer - 4:4\\]
When set, activates CLKOUT output"]
pub type ClkoutCtrlClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects CLKOUT clock source"]
    #[inline(always)]
    pub fn clkout_ctrl_clk_sel(&self) -> ClkoutCtrlClkSelR {
        ClkoutCtrlClkSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set, activates CLKOUT output"]
    #[inline(always)]
    pub fn clkout_ctrl_clk_en(&self) -> ClkoutCtrlClkEnR {
        ClkoutCtrlClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects CLKOUT clock source"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_ctrl_clk_sel(&mut self) -> ClkoutCtrlClkSelW<Cfg0ClkoutCtrlSpec> {
        ClkoutCtrlClkSelW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set, activates CLKOUT output"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_ctrl_clk_en(&mut self) -> ClkoutCtrlClkEnW<Cfg0ClkoutCtrlSpec> {
        ClkoutCtrlClkEnW::new(self, 4)
    }
}
#[doc = "CFG0_CLKOUT_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_clkout_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_clkout_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClkoutCtrlSpec;
impl crate::RegisterSpec for Cfg0ClkoutCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_clkout_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0ClkoutCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_clkout_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0ClkoutCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLKOUT_CTRL to value 0"]
impl crate::Resettable for Cfg0ClkoutCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
