#[doc = "Register `PR1_CFG__SLV__REGS_gpecfg0_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsGpecfg0RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_gpecfg0_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsGpecfg0RegSpec>;
#[doc = "Field `PRU0_GPI_SB_P` reader - "]
pub type Pru0GpiSbPR = crate::BitReader;
#[doc = "Field `PRU0_GPI_SB_P` writer - "]
pub type Pru0GpiSbPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPI_SHIFT_EN` reader - "]
pub type Pru0GpiShiftEnR = crate::BitReader;
#[doc = "Field `PRU0_GPI_SHIFT_EN` writer - "]
pub type Pru0GpiShiftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPO_SHIFT_SWAP` reader - "]
pub type Pru0GpoShiftSwapR = crate::BitReader;
#[doc = "Field `PRU0_GPO_SHIFT_SWAP` writer - "]
pub type Pru0GpoShiftSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPO_SHIFT_CLK_FREE` reader - "]
pub type Pru0GpoShiftClkFreeR = crate::BitReader;
#[doc = "Field `PRU0_GPO_SHIFT_CLK_FREE` writer - "]
pub type Pru0GpoShiftClkFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPO_SHIFT_GP_EN` reader - "]
pub type Pru0GpoShiftGpEnR = crate::BitReader;
#[doc = "Field `PRU0_GPO_SHIFT_GP_EN` writer - "]
pub type Pru0GpoShiftGpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPO_SHIFT_CNT` reader - "]
pub type Pru0GpoShiftCntR = crate::FieldReader;
#[doc = "Field `PRU0_GPO_SHIFT_CNT` writer - "]
pub type Pru0GpoShiftCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU0_GPO_SHIFT_CLK_HIGH` reader - "]
pub type Pru0GpoShiftClkHighR = crate::BitReader;
#[doc = "Field `PRU0_GPO_SHIFT_CLK_HIGH` writer - "]
pub type Pru0GpoShiftClkHighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_GPO_SHIFT_CLK_DONE` reader - "]
pub type Pru0GpoShiftClkDoneR = crate::BitReader;
#[doc = "Field `PRU0_GPO_SHIFT_CLK_DONE` writer - "]
pub type Pru0GpoShiftClkDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pru0_gpi_sb_p(&self) -> Pru0GpiSbPR {
        Pru0GpiSbPR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pru0_gpi_shift_en(&self) -> Pru0GpiShiftEnR {
        Pru0GpiShiftEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pru0_gpo_shift_swap(&self) -> Pru0GpoShiftSwapR {
        Pru0GpoShiftSwapR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pru0_gpo_shift_clk_free(&self) -> Pru0GpoShiftClkFreeR {
        Pru0GpoShiftClkFreeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pru0_gpo_shift_gp_en(&self) -> Pru0GpoShiftGpEnR {
        Pru0GpoShiftGpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pru0_gpo_shift_cnt(&self) -> Pru0GpoShiftCntR {
        Pru0GpoShiftCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_gpo_shift_clk_high(&self) -> Pru0GpoShiftClkHighR {
        Pru0GpoShiftClkHighR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pru0_gpo_shift_clk_done(&self) -> Pru0GpoShiftClkDoneR {
        Pru0GpoShiftClkDoneR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpi_sb_p(&mut self) -> Pru0GpiSbPW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpiSbPW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpi_shift_en(&mut self) -> Pru0GpiShiftEnW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpiShiftEnW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_shift_swap(&mut self) -> Pru0GpoShiftSwapW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpoShiftSwapW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_shift_clk_free(
        &mut self,
    ) -> Pru0GpoShiftClkFreeW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpoShiftClkFreeW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_shift_gp_en(&mut self) -> Pru0GpoShiftGpEnW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpoShiftGpEnW::new(self, 6)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_shift_cnt(&mut self) -> Pru0GpoShiftCntW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpoShiftCntW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_shift_clk_high(
        &mut self,
    ) -> Pru0GpoShiftClkHighW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpoShiftClkHighW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_gpo_shift_clk_done(
        &mut self,
    ) -> Pru0GpoShiftClkDoneW<Pr1Cfg_Slv_RegsGpecfg0RegSpec> {
        Pru0GpoShiftClkDoneW::new(self, 17)
    }
}
#[doc = "PR1_CFG__SLV__REGS_gpecfg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_gpecfg0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_gpecfg0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsGpecfg0RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsGpecfg0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_gpecfg0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsGpecfg0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_gpecfg0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsGpecfg0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_gpecfg0_reg to value 0x23"]
impl crate::Resettable for Pr1Cfg_Slv_RegsGpecfg0RegSpec {
    const RESET_VALUE: u32 = 0x23;
}
