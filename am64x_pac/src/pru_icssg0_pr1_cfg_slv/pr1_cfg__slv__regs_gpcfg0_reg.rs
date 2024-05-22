#[doc = "Register `PR1_CFG__SLV__REGS_gpcfg0_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsGpcfg0RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_gpcfg0_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsGpcfg0RegSpec>;
#[doc = "Field `PRU0_GPI_MODE` reader - "]
pub type Pru0GpiModeR = crate::FieldReader;
#[doc = "Field `PRU0_GPI_MODE` writer - "]
pub type Pru0GpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_GPI_CLK_MODE` reader - "]
pub type Pru0GpiClkModeR = crate::BitReader;
#[doc = "Field `PRU0_GPI_CLK_MODE` writer - "]
pub type Pru0GpiClkModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPI_DIV0` reader - "]
pub type Pru0GpiDiv0R = crate::FieldReader;
#[doc = "Field `PRU0_GPI_DIV0` writer - "]
pub type Pru0GpiDiv0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_GPI_DIV1` reader - "]
pub type Pru0GpiDiv1R = crate::FieldReader;
#[doc = "Field `PRU0_GPI_DIV1` writer - "]
pub type Pru0GpiDiv1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_GPI_SB` reader - "]
pub type Pru0GpiSbR = crate::BitReader;
#[doc = "Field `PRU0_GPI_SB` writer - "]
pub type Pru0GpiSbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPO_MODE` reader - "]
pub type Pru0GpoModeR = crate::BitReader;
#[doc = "Field `PRU0_GPO_MODE` writer - "]
pub type Pru0GpoModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPO_DIV0` reader - "]
pub type Pru0GpoDiv0R = crate::FieldReader;
#[doc = "Field `PRU0_GPO_DIV0` writer - "]
pub type Pru0GpoDiv0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_GPO_DIV1` reader - "]
pub type Pru0GpoDiv1R = crate::FieldReader;
#[doc = "Field `PRU0_GPO_DIV1` writer - "]
pub type Pru0GpoDiv1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_GPO_SH1_SEL` reader - "]
pub type Pru0GpoSh1SelR = crate::BitReader;
#[doc = "Field `PRU0_GPO_SH1_SEL` writer - "]
pub type Pru0GpoSh1SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1_PRU0_GP_MUX_SEL` reader - "]
pub type Pr1Pru0GpMuxSelR = crate::FieldReader;
#[doc = "Field `PR1_PRU0_GP_MUX_SEL` writer - "]
pub type Pr1Pru0GpMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru0_gpi_mode(&self) -> Pru0GpiModeR {
        Pru0GpiModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru0_gpi_clk_mode(&self) -> Pru0GpiClkModeR {
        Pru0GpiClkModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn pru0_gpi_div0(&self) -> Pru0GpiDiv0R {
        Pru0GpiDiv0R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn pru0_gpi_div1(&self) -> Pru0GpiDiv1R {
        Pru0GpiDiv1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pru0_gpi_sb(&self) -> Pru0GpiSbR {
        Pru0GpiSbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pru0_gpo_mode(&self) -> Pru0GpoModeR {
        Pru0GpoModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn pru0_gpo_div0(&self) -> Pru0GpoDiv0R {
        Pru0GpoDiv0R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn pru0_gpo_div1(&self) -> Pru0GpoDiv1R {
        Pru0GpoDiv1R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pru0_gpo_sh1_sel(&self) -> Pru0GpoSh1SelR {
        Pru0GpoSh1SelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn pr1_pru0_gp_mux_sel(&self) -> Pr1Pru0GpMuxSelR {
        Pr1Pru0GpMuxSelR::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpi_mode(&mut self) -> Pru0GpiModeW<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpiModeW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpi_clk_mode(&mut self) -> Pru0GpiClkModeW<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpiClkModeW::new(self, 2)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpi_div0(&mut self) -> Pru0GpiDiv0W<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpiDiv0W::new(self, 3)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpi_div1(&mut self) -> Pru0GpiDiv1W<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpiDiv1W::new(self, 8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpi_sb(&mut self) -> Pru0GpiSbW<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpiSbW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_mode(&mut self) -> Pru0GpoModeW<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpoModeW::new(self, 14)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_div0(&mut self) -> Pru0GpoDiv0W<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpoDiv0W::new(self, 15)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_div1(&mut self) -> Pru0GpoDiv1W<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpoDiv1W::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_sh1_sel(&mut self) -> Pru0GpoSh1SelW<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pru0GpoSh1SelW::new(self, 25)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    #[must_use]
    pub fn pr1_pru0_gp_mux_sel(&mut self) -> Pr1Pru0GpMuxSelW<Pr1Cfg_Slv_RegsGpcfg0RegSpec> {
        Pr1Pru0GpMuxSelW::new(self, 26)
    }
}
#[doc = "PR1_CFG__SLV__REGS_gpcfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_gpcfg0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_gpcfg0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsGpcfg0RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsGpcfg0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_gpcfg0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsGpcfg0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_gpcfg0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsGpcfg0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_gpcfg0_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsGpcfg0RegSpec {
    const RESET_VALUE: u32 = 0;
}
