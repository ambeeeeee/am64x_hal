#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec>;
#[doc = "Field `PRU0_SD_CLK_SEL8` reader - "]
pub type Pru0SdClkSel8R = crate::FieldReader;
#[doc = "Field `PRU0_SD_CLK_SEL8` writer - "]
pub type Pru0SdClkSel8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_SD_CLK_INV8` reader - "]
pub type Pru0SdClkInv8R = crate::BitReader;
#[doc = "Field `PRU0_SD_CLK_INV8` writer - "]
pub type Pru0SdClkInv8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_ACC_SEL8` reader - "]
pub type Pru0SdAccSel8R = crate::FieldReader;
#[doc = "Field `PRU0_SD_ACC_SEL8` writer - "]
pub type Pru0SdAccSel8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_8` reader - "]
pub type Pru0FdZeroMinLimit8R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_8` writer - "]
pub type Pru0FdZeroMinLimit8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MIN_8` reader - "]
pub type Pru0FdZeroMin8R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_8` writer - "]
pub type Pru0FdZeroMin8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_8` reader - "]
pub type Pru0FdZeroMaxLimit8R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_8` writer - "]
pub type Pru0FdZeroMaxLimit8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MAX_8` reader - "]
pub type Pru0FdZeroMax8R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_8` writer - "]
pub type Pru0FdZeroMax8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru0_sd_clk_sel8(&self) -> Pru0SdClkSel8R {
        Pru0SdClkSel8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru0_sd_clk_inv8(&self) -> Pru0SdClkInv8R {
        Pru0SdClkInv8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru0_sd_acc_sel8(&self) -> Pru0SdAccSel8R {
        Pru0SdAccSel8R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_limit_8(&self) -> Pru0FdZeroMinLimit8R {
        Pru0FdZeroMinLimit8R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_8(&self) -> Pru0FdZeroMin8R {
        Pru0FdZeroMin8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_limit_8(&self) -> Pru0FdZeroMaxLimit8R {
        Pru0FdZeroMaxLimit8R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_8(&self) -> Pru0FdZeroMax8R {
        Pru0FdZeroMax8R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_sel8(&mut self) -> Pru0SdClkSel8W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec> {
        Pru0SdClkSel8W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_inv8(&mut self) -> Pru0SdClkInv8W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec> {
        Pru0SdClkInv8W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_acc_sel8(&mut self) -> Pru0SdAccSel8W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec> {
        Pru0SdAccSel8W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_limit_8(
        &mut self,
    ) -> Pru0FdZeroMinLimit8W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec> {
        Pru0FdZeroMinLimit8W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_8(&mut self) -> Pru0FdZeroMin8W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec> {
        Pru0FdZeroMin8W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_limit_8(
        &mut self,
    ) -> Pru0FdZeroMaxLimit8W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec> {
        Pru0FdZeroMaxLimit8W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_8(&mut self) -> Pru0FdZeroMax8W<Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec> {
        Pru0FdZeroMax8W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg8::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg8 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdClkSelReg8Spec {
    const RESET_VALUE: u32 = 0;
}
