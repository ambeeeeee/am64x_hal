#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec>;
#[doc = "Field `PRU0_SD_CLK_SEL2` reader - "]
pub type Pru0SdClkSel2R = crate::FieldReader;
#[doc = "Field `PRU0_SD_CLK_SEL2` writer - "]
pub type Pru0SdClkSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_SD_CLK_INV2` reader - "]
pub type Pru0SdClkInv2R = crate::BitReader;
#[doc = "Field `PRU0_SD_CLK_INV2` writer - "]
pub type Pru0SdClkInv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_ACC_SEL2` reader - "]
pub type Pru0SdAccSel2R = crate::FieldReader;
#[doc = "Field `PRU0_SD_ACC_SEL2` writer - "]
pub type Pru0SdAccSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_2` reader - "]
pub type Pru0FdZeroMinLimit2R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_LIMIT_2` writer - "]
pub type Pru0FdZeroMinLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MIN_2` reader - "]
pub type Pru0FdZeroMin2R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MIN_2` writer - "]
pub type Pru0FdZeroMin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_2` reader - "]
pub type Pru0FdZeroMaxLimit2R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_LIMIT_2` writer - "]
pub type Pru0FdZeroMaxLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ZERO_MAX_2` reader - "]
pub type Pru0FdZeroMax2R = crate::BitReader;
#[doc = "Field `PRU0_FD_ZERO_MAX_2` writer - "]
pub type Pru0FdZeroMax2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru0_sd_clk_sel2(&self) -> Pru0SdClkSel2R {
        Pru0SdClkSel2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru0_sd_clk_inv2(&self) -> Pru0SdClkInv2R {
        Pru0SdClkInv2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru0_sd_acc_sel2(&self) -> Pru0SdAccSel2R {
        Pru0SdAccSel2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_limit_2(&self) -> Pru0FdZeroMinLimit2R {
        Pru0FdZeroMinLimit2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_zero_min_2(&self) -> Pru0FdZeroMin2R {
        Pru0FdZeroMin2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_limit_2(&self) -> Pru0FdZeroMaxLimit2R {
        Pru0FdZeroMaxLimit2R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_zero_max_2(&self) -> Pru0FdZeroMax2R {
        Pru0FdZeroMax2R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_sel2(&mut self) -> Pru0SdClkSel2W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec> {
        Pru0SdClkSel2W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_clk_inv2(&mut self) -> Pru0SdClkInv2W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec> {
        Pru0SdClkInv2W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_acc_sel2(&mut self) -> Pru0SdAccSel2W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec> {
        Pru0SdAccSel2W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_limit_2(
        &mut self,
    ) -> Pru0FdZeroMinLimit2W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec> {
        Pru0FdZeroMinLimit2W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_min_2(&mut self) -> Pru0FdZeroMin2W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec> {
        Pru0FdZeroMin2W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_limit_2(
        &mut self,
    ) -> Pru0FdZeroMaxLimit2W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec> {
        Pru0FdZeroMaxLimit2W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_zero_max_2(&mut self) -> Pru0FdZeroMax2W<Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec> {
        Pru0FdZeroMax2W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_clk_sel_reg2::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_clk_sel_reg2 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdClkSelReg2Spec {
    const RESET_VALUE: u32 = 0;
}
