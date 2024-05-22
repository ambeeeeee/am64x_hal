#[doc = "Register `PR1_CFG__SLV__REGS_gpecfg1_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsGpecfg1RegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_gpecfg1_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsGpecfg1RegSpec>;
#[doc = "Field `PRU1_GPI_SB_P` reader - "]
pub type Pru1GpiSbPR = crate::BitReader;
#[doc = "Field `PRU1_GPI_SB_P` writer - "]
pub type Pru1GpiSbPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_GPI_SHIFT_EN` reader - "]
pub type Pru1GpiShiftEnR = crate::BitReader;
#[doc = "Field `PRU1_GPI_SHIFT_EN` writer - "]
pub type Pru1GpiShiftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_GPO_SHIFT_SWAP` reader - "]
pub type Pru1GpoShiftSwapR = crate::BitReader;
#[doc = "Field `PRU1_GPO_SHIFT_SWAP` writer - "]
pub type Pru1GpoShiftSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_GPO_SHIFT_CLK_FREE` reader - "]
pub type Pru1GpoShiftClkFreeR = crate::BitReader;
#[doc = "Field `PRU1_GPO_SHIFT_CLK_FREE` writer - "]
pub type Pru1GpoShiftClkFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_GPO_SHIFT_GP_EN` reader - "]
pub type Pru1GpoShiftGpEnR = crate::BitReader;
#[doc = "Field `PRU1_GPO_SHIFT_GP_EN` writer - "]
pub type Pru1GpoShiftGpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_GPO_SHIFT_CNT` reader - "]
pub type Pru1GpoShiftCntR = crate::FieldReader;
#[doc = "Field `PRU1_GPO_SHIFT_CNT` writer - "]
pub type Pru1GpoShiftCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_GPO_SHIFT_CLK_HIGH` reader - "]
pub type Pru1GpoShiftClkHighR = crate::BitReader;
#[doc = "Field `PRU1_GPO_SHIFT_CLK_HIGH` writer - "]
pub type Pru1GpoShiftClkHighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_GPO_SHIFT_CLK_DONE` reader - "]
pub type Pru1GpoShiftClkDoneR = crate::BitReader;
#[doc = "Field `PRU1_GPO_SHIFT_CLK_DONE` writer - "]
pub type Pru1GpoShiftClkDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pru1_gpi_sb_p(&self) -> Pru1GpiSbPR {
        Pru1GpiSbPR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pru1_gpi_shift_en(&self) -> Pru1GpiShiftEnR {
        Pru1GpiShiftEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pru1_gpo_shift_swap(&self) -> Pru1GpoShiftSwapR {
        Pru1GpoShiftSwapR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pru1_gpo_shift_clk_free(&self) -> Pru1GpoShiftClkFreeR {
        Pru1GpoShiftClkFreeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pru1_gpo_shift_gp_en(&self) -> Pru1GpoShiftGpEnR {
        Pru1GpoShiftGpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pru1_gpo_shift_cnt(&self) -> Pru1GpoShiftCntR {
        Pru1GpoShiftCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_gpo_shift_clk_high(&self) -> Pru1GpoShiftClkHighR {
        Pru1GpoShiftClkHighR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pru1_gpo_shift_clk_done(&self) -> Pru1GpoShiftClkDoneR {
        Pru1GpoShiftClkDoneR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpi_sb_p(&mut self) -> Pru1GpiSbPW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpiSbPW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpi_shift_en(&mut self) -> Pru1GpiShiftEnW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpiShiftEnW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpo_shift_swap(&mut self) -> Pru1GpoShiftSwapW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpoShiftSwapW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpo_shift_clk_free(
        &mut self,
    ) -> Pru1GpoShiftClkFreeW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpoShiftClkFreeW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpo_shift_gp_en(&mut self) -> Pru1GpoShiftGpEnW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpoShiftGpEnW::new(self, 6)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpo_shift_cnt(&mut self) -> Pru1GpoShiftCntW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpoShiftCntW::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpo_shift_clk_high(
        &mut self,
    ) -> Pru1GpoShiftClkHighW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpoShiftClkHighW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_gpo_shift_clk_done(
        &mut self,
    ) -> Pru1GpoShiftClkDoneW<Pr1Cfg_Slv_RegsGpecfg1RegSpec> {
        Pru1GpoShiftClkDoneW::new(self, 17)
    }
}
#[doc = "PR1_CFG__SLV__REGS_gpecfg1_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_gpecfg1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_gpecfg1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsGpecfg1RegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsGpecfg1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_gpecfg1_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsGpecfg1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_gpecfg1_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsGpecfg1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_gpecfg1_reg to value 0x23"]
impl crate::Resettable for Pr1Cfg_Slv_RegsGpecfg1RegSpec {
    const RESET_VALUE: u32 = 0x23;
}
