#[doc = "Register `CFG0_OBSCLK0_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Obsclk0CtrlProxySpec>;
#[doc = "Register `CFG0_OBSCLK0_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Obsclk0CtrlProxySpec>;
#[doc = "Field `OBSCLK0_CTRL_CLK_SEL_PROXY` reader - 3:0\\]
OBSCLK0 clock source selection."]
pub type Obsclk0CtrlClkSelProxyR = crate::FieldReader;
#[doc = "Field `OBSCLK0_CTRL_CLK_SEL_PROXY` writer - 3:0\\]
OBSCLK0 clock source selection."]
pub type Obsclk0CtrlClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV_PROXY` reader - 15:8\\]
OBSCLK0 output divider"]
pub type Obsclk0CtrlClkDivProxyR = crate::FieldReader;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV_PROXY` writer - 15:8\\]
OBSCLK0 output divider"]
pub type Obsclk0CtrlClkDivProxyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV_LD_PROXY` reader - 16:16\\]
Load the output divider value"]
pub type Obsclk0CtrlClkDivLdProxyR = crate::BitReader;
#[doc = "Field `OBSCLK0_CTRL_CLK_DIV_LD_PROXY` writer - 16:16\\]
Load the output divider value"]
pub type Obsclk0CtrlClkDivLdProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
OBSCLK0 clock source selection."]
    #[inline(always)]
    pub fn obsclk0_ctrl_clk_sel_proxy(&self) -> Obsclk0CtrlClkSelProxyR {
        Obsclk0CtrlClkSelProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
OBSCLK0 output divider"]
    #[inline(always)]
    pub fn obsclk0_ctrl_clk_div_proxy(&self) -> Obsclk0CtrlClkDivProxyR {
        Obsclk0CtrlClkDivProxyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    pub fn obsclk0_ctrl_clk_div_ld_proxy(&self) -> Obsclk0CtrlClkDivLdProxyR {
        Obsclk0CtrlClkDivLdProxyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
OBSCLK0 clock source selection."]
    #[inline(always)]
    #[must_use]
    pub fn obsclk0_ctrl_clk_sel_proxy(
        &mut self,
    ) -> Obsclk0CtrlClkSelProxyW<Cfg0Obsclk0CtrlProxySpec> {
        Obsclk0CtrlClkSelProxyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
OBSCLK0 output divider"]
    #[inline(always)]
    #[must_use]
    pub fn obsclk0_ctrl_clk_div_proxy(
        &mut self,
    ) -> Obsclk0CtrlClkDivProxyW<Cfg0Obsclk0CtrlProxySpec> {
        Obsclk0CtrlClkDivProxyW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    #[must_use]
    pub fn obsclk0_ctrl_clk_div_ld_proxy(
        &mut self,
    ) -> Obsclk0CtrlClkDivLdProxyW<Cfg0Obsclk0CtrlProxySpec> {
        Obsclk0CtrlClkDivLdProxyW::new(self, 16)
    }
}
#[doc = "CFG0_OBSCLK0_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_obsclk0_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_obsclk0_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Obsclk0CtrlProxySpec;
impl crate::RegisterSpec for Cfg0Obsclk0CtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_obsclk0_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Obsclk0CtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_obsclk0_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Obsclk0CtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_OBSCLK0_CTRL_PROXY to value 0x07"]
impl crate::Resettable for Cfg0Obsclk0CtrlProxySpec {
    const RESET_VALUE: u32 = 0x07;
}
