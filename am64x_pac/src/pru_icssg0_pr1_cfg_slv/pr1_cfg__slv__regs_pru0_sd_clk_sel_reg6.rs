#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec>;
#[doc = "Field `PRU0_SD_CLK_SEL6` reader - "]
pub type Pru0SdClkSel6R = crate::FieldReader;
#[doc = "Field `PRU0_SD_CLK_SEL6` writer - "]
pub type Pru0SdClkSel6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_SD_CLK_INV6` reader - "]
pub type Pru0SdClkInv6R = crate::BitReader;
#[doc = "Field `PRU0_SD_CLK_INV6` writer - "]
pub type Pru0SdClkInv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_ACC_SEL6` reader - "]
pub type Pru0SdAccSel6R = crate::FieldReader;
#[doc = "Field `PRU0_SD_ACC_SEL6` writer - "]
pub type Pru0SdAccSel6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_6` reader - "]
pub type Pru0FdZeroMinLimit6R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_6` writer - "]
pub type Pru0FdZeroMinLimit6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MIN_6` reader - "]
pub type Pru0FdZeroMin6R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_6` writer - "]
pub type Pru0FdZeroMin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_6` reader - "]
pub type Pru0FdZeroMaxLimit6R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_6` writer - "]
pub type Pru0FdZeroMaxLimit6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MAX_6` reader - "]
pub type Pru0FdZeroMax6R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_6` writer - "]
pub type Pru0FdZeroMax6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru0_sd_clk_sel6(&self) -> Pru0SdClkSel6R {
        Pru0SdClkSel6R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru0_sd_clk_inv6(&self) -> Pru0SdClkInv6R {
        Pru0SdClkInv6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru0_sd_acc_sel6(&self) -> Pru0SdAccSel6R {
        Pru0SdAccSel6R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_limit_6(&self) -> Pru0FdZeroMinLimit6R {
        Pru0FdZeroMinLimit6R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_6(&self) -> Pru0FdZeroMin6R {
        Pru0FdZeroMin6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_limit_6(&self) -> Pru0FdZeroMaxLimit6R {
        Pru0FdZeroMaxLimit6R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_6(&self) -> Pru0FdZeroMax6R {
        Pru0FdZeroMax6R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_sel6(&mut self) -> Pru0SdClkSel6W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec> {
        Pru0SdClkSel6W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_inv6(&mut self) -> Pru0SdClkInv6W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec> {
        Pru0SdClkInv6W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_acc_sel6(&mut self) -> Pru0SdAccSel6W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec> {
        Pru0SdAccSel6W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_limit_6(
        &mut self,
    ) -> Pru0FdZeroMinLimit6W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec> {
        Pru0FdZeroMinLimit6W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_6(&mut self) -> Pru0FdZeroMin6W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec> {
        Pru0FdZeroMin6W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_limit_6(
        &mut self,
    ) -> Pru0FdZeroMaxLimit6W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec> {
        Pru0FdZeroMaxLimit6W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_6(&mut self) -> Pru0FdZeroMax6W<Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec> {
        Pru0FdZeroMax6W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg6::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg6 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdClkSelReg6Spec {
    const RESET_VALUE: u32 = 0;
}
