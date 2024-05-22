#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec>;
#[doc = "Field `PRU1_SD_CLK_SEL7` reader - "]
pub type Pru1SdClkSel7R = crate::FieldReader;
#[doc = "Field `PRU1_SD_CLK_SEL7` writer - "]
pub type Pru1SdClkSel7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_SD_CLK_INV7` reader - "]
pub type Pru1SdClkInv7R = crate::BitReader;
#[doc = "Field `PRU1_SD_CLK_INV7` writer - "]
pub type Pru1SdClkInv7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_ACC_SEL7` reader - "]
pub type Pru1SdAccSel7R = crate::FieldReader;
#[doc = "Field `PRU1_SD_ACC_SEL7` writer - "]
pub type Pru1SdAccSel7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_7` reader - "]
pub type Pru1FdZeroMinLimit7R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_7` writer - "]
pub type Pru1FdZeroMinLimit7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MIN_7` reader - "]
pub type Pru1FdZeroMin7R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_7` writer - "]
pub type Pru1FdZeroMin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_7` reader - "]
pub type Pru1FdZeroMaxLimit7R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_7` writer - "]
pub type Pru1FdZeroMaxLimit7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MAX_7` reader - "]
pub type Pru1FdZeroMax7R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_7` writer - "]
pub type Pru1FdZeroMax7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru1_sd_clk_sel7(&self) -> Pru1SdClkSel7R {
        Pru1SdClkSel7R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru1_sd_clk_inv7(&self) -> Pru1SdClkInv7R {
        Pru1SdClkInv7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru1_sd_acc_sel7(&self) -> Pru1SdAccSel7R {
        Pru1SdAccSel7R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_limit_7(&self) -> Pru1FdZeroMinLimit7R {
        Pru1FdZeroMinLimit7R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_7(&self) -> Pru1FdZeroMin7R {
        Pru1FdZeroMin7R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_limit_7(&self) -> Pru1FdZeroMaxLimit7R {
        Pru1FdZeroMaxLimit7R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_7(&self) -> Pru1FdZeroMax7R {
        Pru1FdZeroMax7R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_sel7(&mut self) -> Pru1SdClkSel7W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec> {
        Pru1SdClkSel7W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_inv7(&mut self) -> Pru1SdClkInv7W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec> {
        Pru1SdClkInv7W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_acc_sel7(&mut self) -> Pru1SdAccSel7W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec> {
        Pru1SdAccSel7W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_limit_7(
        &mut self,
    ) -> Pru1FdZeroMinLimit7W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec> {
        Pru1FdZeroMinLimit7W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_7(&mut self) -> Pru1FdZeroMin7W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec> {
        Pru1FdZeroMin7W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_limit_7(
        &mut self,
    ) -> Pru1FdZeroMaxLimit7W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec> {
        Pru1FdZeroMaxLimit7W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_7(&mut self) -> Pru1FdZeroMax7W<Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec> {
        Pru1FdZeroMax7W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg7::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg7 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdClkSelReg7Spec {
    const RESET_VALUE: u32 = 0;
}
