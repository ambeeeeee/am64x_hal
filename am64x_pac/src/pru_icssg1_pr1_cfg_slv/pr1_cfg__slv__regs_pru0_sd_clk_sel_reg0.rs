#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec>;
#[doc = "Field `PRU0_SD_CLK_SEL0` reader - "]
pub type Pru0SdClkSel0R = crate::FieldReader;
#[doc = "Field `PRU0_SD_CLK_SEL0` writer - "]
pub type Pru0SdClkSel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_SD_CLK_INV0` reader - "]
pub type Pru0SdClkInv0R = crate::BitReader;
#[doc = "Field `PRU0_SD_CLK_INV0` writer - "]
pub type Pru0SdClkInv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_ACC_SEL0` reader - "]
pub type Pru0SdAccSel0R = crate::FieldReader;
#[doc = "Field `PRU0_SD_ACC_SEL0` writer - "]
pub type Pru0SdAccSel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_0` reader - "]
pub type Pru0FdZeroMinLimit0R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_0` writer - "]
pub type Pru0FdZeroMinLimit0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MIN_0` reader - "]
pub type Pru0FdZeroMin0R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_0` writer - "]
pub type Pru0FdZeroMin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_0` reader - "]
pub type Pru0FdZeroMaxLimit0R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_0` writer - "]
pub type Pru0FdZeroMaxLimit0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MAX_0` reader - "]
pub type Pru0FdZeroMax0R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_0` writer - "]
pub type Pru0FdZeroMax0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru0_sd_clk_sel0(&self) -> Pru0SdClkSel0R {
        Pru0SdClkSel0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru0_sd_clk_inv0(&self) -> Pru0SdClkInv0R {
        Pru0SdClkInv0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru0_sd_acc_sel0(&self) -> Pru0SdAccSel0R {
        Pru0SdAccSel0R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_limit_0(&self) -> Pru0FdZeroMinLimit0R {
        Pru0FdZeroMinLimit0R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_0(&self) -> Pru0FdZeroMin0R {
        Pru0FdZeroMin0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_limit_0(&self) -> Pru0FdZeroMaxLimit0R {
        Pru0FdZeroMaxLimit0R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_0(&self) -> Pru0FdZeroMax0R {
        Pru0FdZeroMax0R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_sel0(&mut self) -> Pru0SdClkSel0W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec> {
        Pru0SdClkSel0W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_inv0(&mut self) -> Pru0SdClkInv0W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec> {
        Pru0SdClkInv0W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_acc_sel0(&mut self) -> Pru0SdAccSel0W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec> {
        Pru0SdAccSel0W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_limit_0(
        &mut self,
    ) -> Pru0FdZeroMinLimit0W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec> {
        Pru0FdZeroMinLimit0W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_0(&mut self) -> Pru0FdZeroMin0W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec> {
        Pru0FdZeroMin0W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_limit_0(
        &mut self,
    ) -> Pru0FdZeroMaxLimit0W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec> {
        Pru0FdZeroMaxLimit0W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_0(&mut self) -> Pru0FdZeroMax0W<Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec> {
        Pru0FdZeroMax0W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg0::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg0 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdClkSelReg0Spec {
    const RESET_VALUE: u32 = 0;
}
