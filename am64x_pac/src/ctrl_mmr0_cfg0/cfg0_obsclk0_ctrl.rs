#[doc = "Register `CFG0_OBSCLK0_CTRL` reader"]
pub type R = crate::R<Cfg0Obsclk0CtrlSpec>;
#[doc = "Register `CFG0_OBSCLK0_CTRL` writer"]
pub type W = crate::W<Cfg0Obsclk0CtrlSpec>;
#[doc = "Field `OBSCLK0_CTRL_CLK_SEL` reader - 3:0\\]
OBSCLK0 clock source selection."]
pub type Obsclk0CtrlClkSelR = crate::FieldReader;
#[doc = "Field `OBSCLK0_CTRL_CLK_SEL` writer - 3:0\\]
OBSCLK0 clock source selection."]
pub type Obsclk0CtrlClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV` reader - 15:8\\]
OBSCLK0 output divider"]
pub type Obsclk0CtrlClkDivR = crate::FieldReader;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV` writer - 15:8\\]
OBSCLK0 output divider"]
pub type Obsclk0CtrlClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV_LD` reader - 16:16\\]
Load the output divider value"]
pub type Obsclk0CtrlClkDivLdR = crate::BitReader;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV_LD` writer - 16:16\\]
Load the output divider value"]
pub type Obsclk0CtrlClkDivLdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
OBSCLK0 clock source selection."]
    #[inline(always)]
    pub fn obsclk0_ctrl_clk_sel(&self) -> Obsclk0CtrlClkSelR {
        Obsclk0CtrlClkSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
OBSCLK0 output divider"]
    #[inline(always)]
    pub fn obsclk0_ctrl_clk_div(&self) -> Obsclk0CtrlClkDivR {
        Obsclk0CtrlClkDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    pub fn obsclk0_ctrl_clk_div_ld(&self) -> Obsclk0CtrlClkDivLdR {
        Obsclk0CtrlClkDivLdR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
OBSCLK0 clock source selection."]
    #[inline(always)]
    #[must_use]
    pub fn obsclk0_ctrl_clk_sel(&mut self) -> Obsclk0CtrlClkSelW<Cfg0Obsclk0CtrlSpec> {
        Obsclk0CtrlClkSelW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
OBSCLK0 output divider"]
    #[inline(always)]
    #[must_use]
    pub fn obsclk0_ctrl_clk_div(&mut self) -> Obsclk0CtrlClkDivW<Cfg0Obsclk0CtrlSpec> {
        Obsclk0CtrlClkDivW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    #[must_use]
    pub fn obsclk0_ctrl_clk_div_ld(&mut self) -> Obsclk0CtrlClkDivLdW<Cfg0Obsclk0CtrlSpec> {
        Obsclk0CtrlClkDivLdW::new(self, 16)
    }
}
#[doc = "CFG0_OBSCLK0_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_obsclk0_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_obsclk0_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Obsclk0CtrlSpec;
impl crate::RegisterSpec for Cfg0Obsclk0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_obsclk0_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Obsclk0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_obsclk0_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Obsclk0CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_OBSCLK0_CTRL to value 0x07"]
impl crate::Resettable for Cfg0Obsclk0CtrlSpec {
    const RESET_VALUE: u32 = 0x07;
}
