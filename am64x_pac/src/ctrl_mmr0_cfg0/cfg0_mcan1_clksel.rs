#[doc = "Register `CFG0_MCAN1_CLKSEL` reader"]
pub type R = crate::R<Cfg0Mcan1ClkselSpec>;
#[doc = "Register `CFG0_MCAN1_CLKSEL` writer"]
pub type W = crate::W<Cfg0Mcan1ClkselSpec>;
#[doc = "Field `MCAN1_CLKSEL_CLK_SEL` reader - 1:0\\]
MAIN MCAN_CLK selection"]
pub type Mcan1ClkselClkSelR = crate::FieldReader;
#[doc = "Field `MCAN1_CLKSEL_CLK_SEL` writer - 1:0\\]
MAIN MCAN_CLK selection"]
pub type Mcan1ClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
MAIN MCAN_CLK selection"]
    #[inline(always)]
    pub fn mcan1_clksel_clk_sel(&self) -> Mcan1ClkselClkSelR {
        Mcan1ClkselClkSelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
MAIN MCAN_CLK selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcan1_clksel_clk_sel(&mut self) -> Mcan1ClkselClkSelW<Cfg0Mcan1ClkselSpec> {
        Mcan1ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_MCAN1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcan1_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcan1_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Mcan1ClkselSpec;
impl crate::RegisterSpec for Cfg0Mcan1ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcan1_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Mcan1ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcan1_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Mcan1ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCAN1_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Mcan1ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
