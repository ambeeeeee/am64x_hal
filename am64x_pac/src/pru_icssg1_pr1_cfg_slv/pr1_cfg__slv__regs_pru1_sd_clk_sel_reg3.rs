#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec>;
#[doc = "Field `PRU1_SD_CLK_SEL3` reader - "]
pub type Pru1SdClkSel3R = crate::FieldReader;
#[doc = "Field `PRU1_SD_CLK_SEL3` writer - "]
pub type Pru1SdClkSel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_SD_CLK_INV3` reader - "]
pub type Pru1SdClkInv3R = crate::BitReader;
#[doc = "Field `PRU1_SD_CLK_INV3` writer - "]
pub type Pru1SdClkInv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_ACC_SEL3` reader - "]
pub type Pru1SdAccSel3R = crate::FieldReader;
#[doc = "Field `PRU1_SD_ACC_SEL3` writer - "]
pub type Pru1SdAccSel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_3` reader - "]
pub type Pru1FdZeroMinLimit3R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_3` writer - "]
pub type Pru1FdZeroMinLimit3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MIN_3` reader - "]
pub type Pru1FdZeroMin3R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_3` writer - "]
pub type Pru1FdZeroMin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_3` reader - "]
pub type Pru1FdZeroMaxLimit3R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_3` writer - "]
pub type Pru1FdZeroMaxLimit3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MAX_3` reader - "]
pub type Pru1FdZeroMax3R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_3` writer - "]
pub type Pru1FdZeroMax3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru1_sd_clk_sel3(&self) -> Pru1SdClkSel3R {
        Pru1SdClkSel3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru1_sd_clk_inv3(&self) -> Pru1SdClkInv3R {
        Pru1SdClkInv3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru1_sd_acc_sel3(&self) -> Pru1SdAccSel3R {
        Pru1SdAccSel3R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_limit_3(&self) -> Pru1FdZeroMinLimit3R {
        Pru1FdZeroMinLimit3R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_3(&self) -> Pru1FdZeroMin3R {
        Pru1FdZeroMin3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_limit_3(&self) -> Pru1FdZeroMaxLimit3R {
        Pru1FdZeroMaxLimit3R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_3(&self) -> Pru1FdZeroMax3R {
        Pru1FdZeroMax3R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_sel3(&mut self) -> Pru1SdClkSel3W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec> {
        Pru1SdClkSel3W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_inv3(&mut self) -> Pru1SdClkInv3W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec> {
        Pru1SdClkInv3W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_acc_sel3(&mut self) -> Pru1SdAccSel3W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec> {
        Pru1SdAccSel3W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_limit_3(
        &mut self,
    ) -> Pru1FdZeroMinLimit3W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec> {
        Pru1FdZeroMinLimit3W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_3(&mut self) -> Pru1FdZeroMin3W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec> {
        Pru1FdZeroMin3W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_limit_3(
        &mut self,
    ) -> Pru1FdZeroMaxLimit3W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec> {
        Pru1FdZeroMaxLimit3W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_3(&mut self) -> Pru1FdZeroMax3W<Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec> {
        Pru1FdZeroMax3W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg3::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg3 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdClkSelReg3Spec {
    const RESET_VALUE: u32 = 0;
}
