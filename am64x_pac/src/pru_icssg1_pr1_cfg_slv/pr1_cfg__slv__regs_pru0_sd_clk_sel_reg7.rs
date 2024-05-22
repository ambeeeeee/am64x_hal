#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec>;
#[doc = "Field `PRU0_SD_CLK_SEL7` reader - "]
pub type Pru0SdClkSel7R = crate::FieldReader;
#[doc = "Field `PRU0_SD_CLK_SEL7` writer - "]
pub type Pru0SdClkSel7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_SD_CLK_INV7` reader - "]
pub type Pru0SdClkInv7R = crate::BitReader;
#[doc = "Field `PRU0_SD_CLK_INV7` writer - "]
pub type Pru0SdClkInv7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_ACC_SEL7` reader - "]
pub type Pru0SdAccSel7R = crate::FieldReader;
#[doc = "Field `PRU0_SD_ACC_SEL7` writer - "]
pub type Pru0SdAccSel7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_7` reader - "]
pub type Pru0FdZeroMinLimit7R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_7` writer - "]
pub type Pru0FdZeroMinLimit7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MIN_7` reader - "]
pub type Pru0FdZeroMin7R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_7` writer - "]
pub type Pru0FdZeroMin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_7` reader - "]
pub type Pru0FdZeroMaxLimit7R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_7` writer - "]
pub type Pru0FdZeroMaxLimit7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MAX_7` reader - "]
pub type Pru0FdZeroMax7R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_7` writer - "]
pub type Pru0FdZeroMax7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru0_sd_clk_sel7(&self) -> Pru0SdClkSel7R {
        Pru0SdClkSel7R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru0_sd_clk_inv7(&self) -> Pru0SdClkInv7R {
        Pru0SdClkInv7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru0_sd_acc_sel7(&self) -> Pru0SdAccSel7R {
        Pru0SdAccSel7R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_limit_7(&self) -> Pru0FdZeroMinLimit7R {
        Pru0FdZeroMinLimit7R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_7(&self) -> Pru0FdZeroMin7R {
        Pru0FdZeroMin7R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_limit_7(&self) -> Pru0FdZeroMaxLimit7R {
        Pru0FdZeroMaxLimit7R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_7(&self) -> Pru0FdZeroMax7R {
        Pru0FdZeroMax7R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_sel7(&mut self) -> Pru0SdClkSel7W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec> {
        Pru0SdClkSel7W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_inv7(&mut self) -> Pru0SdClkInv7W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec> {
        Pru0SdClkInv7W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_acc_sel7(&mut self) -> Pru0SdAccSel7W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec> {
        Pru0SdAccSel7W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_limit_7(
        &mut self,
    ) -> Pru0FdZeroMinLimit7W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec> {
        Pru0FdZeroMinLimit7W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_7(&mut self) -> Pru0FdZeroMin7W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec> {
        Pru0FdZeroMin7W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_limit_7(
        &mut self,
    ) -> Pru0FdZeroMaxLimit7W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec> {
        Pru0FdZeroMaxLimit7W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_7(&mut self) -> Pru0FdZeroMax7W<Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec> {
        Pru0FdZeroMax7W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg7::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg7 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdClkSelReg7Spec {
    const RESET_VALUE: u32 = 0;
}
