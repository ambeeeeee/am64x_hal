#[doc = "Register `CFG0_MCU_OBSCLK_CTRL` reader"]
pub type R = crate::R<Cfg0McuObsclkCtrlSpec>;
#[doc = "Register `CFG0_MCU_OBSCLK_CTRL` writer"]
pub type W = crate::W<Cfg0McuObsclkCtrlSpec>;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_SEL` reader - 2:0\\]
MCU_OBSCLK pin clock selection"]
pub type McuObsclkCtrlClkSelR = crate::FieldReader;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_SEL` writer - 2:0\\]
MCU_OBSCLK pin clock selection"]
pub type McuObsclkCtrlClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV` reader - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
pub type McuObsclkCtrlClkDivR = crate::FieldReader;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV` writer - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
pub type McuObsclkCtrlClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV_LD` reader - 16:16\\]
Load the output divider value"]
pub type McuObsclkCtrlClkDivLdR = crate::BitReader;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV_LD` writer - 16:16\\]
Load the output divider value"]
pub type McuObsclkCtrlClkDivLdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_OBSCLK_CTRL_OUT_MUX_SEL` reader - 24:24\\]
MCU_OBSCLK pin output mux selection."]
pub type McuObsclkCtrlOutMuxSelR = crate::BitReader;
#[doc = "Field `MCU_OBSCLK_CTRL_OUT_MUX_SEL` writer - 24:24\\]
MCU_OBSCLK pin output mux selection."]
pub type McuObsclkCtrlOutMuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
MCU_OBSCLK pin clock selection"]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_clk_sel(&self) -> McuObsclkCtrlClkSelR {
        McuObsclkCtrlClkSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_clk_div(&self) -> McuObsclkCtrlClkDivR {
        McuObsclkCtrlClkDivR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_clk_div_ld(&self) -> McuObsclkCtrlClkDivLdR {
        McuObsclkCtrlClkDivLdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
MCU_OBSCLK pin output mux selection."]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_out_mux_sel(&self) -> McuObsclkCtrlOutMuxSelR {
        McuObsclkCtrlOutMuxSelR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
MCU_OBSCLK pin clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_clk_sel(&mut self) -> McuObsclkCtrlClkSelW<Cfg0McuObsclkCtrlSpec> {
        McuObsclkCtrlClkSelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_clk_div(&mut self) -> McuObsclkCtrlClkDivW<Cfg0McuObsclkCtrlSpec> {
        McuObsclkCtrlClkDivW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_clk_div_ld(&mut self) -> McuObsclkCtrlClkDivLdW<Cfg0McuObsclkCtrlSpec> {
        McuObsclkCtrlClkDivLdW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
MCU_OBSCLK pin output mux selection."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_out_mux_sel(
        &mut self,
    ) -> McuObsclkCtrlOutMuxSelW<Cfg0McuObsclkCtrlSpec> {
        McuObsclkCtrlOutMuxSelW::new(self, 24)
    }
}
#[doc = "CFG0_MCU_OBSCLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_obsclk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_obsclk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuObsclkCtrlSpec;
impl crate::RegisterSpec for Cfg0McuObsclkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_obsclk_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0McuObsclkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_obsclk_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0McuObsclkCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_OBSCLK_CTRL to value 0"]
impl crate::Resettable for Cfg0McuObsclkCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
