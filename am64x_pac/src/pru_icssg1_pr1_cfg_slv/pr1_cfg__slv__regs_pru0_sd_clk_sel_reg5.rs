#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec>;
#[doc = "Field `PRU0_SD_CLK_SEL5` reader - "]
pub type Pru0SdClkSel5R = crate::FieldReader;
#[doc = "Field `PRU0_SD_CLK_SEL5` writer - "]
pub type Pru0SdClkSel5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_SD_CLK_INV5` reader - "]
pub type Pru0SdClkInv5R = crate::BitReader;
#[doc = "Field `PRU0_SD_CLK_INV5` writer - "]
pub type Pru0SdClkInv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_ACC_SEL5` reader - "]
pub type Pru0SdAccSel5R = crate::FieldReader;
#[doc = "Field `PRU0_SD_ACC_SEL5` writer - "]
pub type Pru0SdAccSel5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_5` reader - "]
pub type Pru0FdZeroMinLimit5R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_5` writer - "]
pub type Pru0FdZeroMinLimit5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MIN_5` reader - "]
pub type Pru0FdZeroMin5R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_5` writer - "]
pub type Pru0FdZeroMin5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_5` reader - "]
pub type Pru0FdZeroMaxLimit5R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_5` writer - "]
pub type Pru0FdZeroMaxLimit5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MAX_5` reader - "]
pub type Pru0FdZeroMax5R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_5` writer - "]
pub type Pru0FdZeroMax5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru0_sd_clk_sel5(&self) -> Pru0SdClkSel5R {
        Pru0SdClkSel5R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru0_sd_clk_inv5(&self) -> Pru0SdClkInv5R {
        Pru0SdClkInv5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru0_sd_acc_sel5(&self) -> Pru0SdAccSel5R {
        Pru0SdAccSel5R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_limit_5(&self) -> Pru0FdZeroMinLimit5R {
        Pru0FdZeroMinLimit5R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_5(&self) -> Pru0FdZeroMin5R {
        Pru0FdZeroMin5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_limit_5(&self) -> Pru0FdZeroMaxLimit5R {
        Pru0FdZeroMaxLimit5R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_5(&self) -> Pru0FdZeroMax5R {
        Pru0FdZeroMax5R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_sel5(&mut self) -> Pru0SdClkSel5W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec> {
        Pru0SdClkSel5W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_inv5(&mut self) -> Pru0SdClkInv5W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec> {
        Pru0SdClkInv5W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_acc_sel5(&mut self) -> Pru0SdAccSel5W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec> {
        Pru0SdAccSel5W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_limit_5(
        &mut self,
    ) -> Pru0FdZeroMinLimit5W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec> {
        Pru0FdZeroMinLimit5W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_5(&mut self) -> Pru0FdZeroMin5W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec> {
        Pru0FdZeroMin5W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_limit_5(
        &mut self,
    ) -> Pru0FdZeroMaxLimit5W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec> {
        Pru0FdZeroMaxLimit5W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_5(&mut self) -> Pru0FdZeroMax5W<Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec> {
        Pru0FdZeroMax5W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg5::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg5 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdClkSelReg5Spec {
    const RESET_VALUE: u32 = 0;
}
