#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec>;
#[doc = "Field `PRU1_SD_CLK_SEL2` reader - "]
pub type Pru1SdClkSel2R = crate::FieldReader;
#[doc = "Field `PRU1_SD_CLK_SEL2` writer - "]
pub type Pru1SdClkSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_SD_CLK_INV2` reader - "]
pub type Pru1SdClkInv2R = crate::BitReader;
#[doc = "Field `PRU1_SD_CLK_INV2` writer - "]
pub type Pru1SdClkInv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_ACC_SEL2` reader - "]
pub type Pru1SdAccSel2R = crate::FieldReader;
#[doc = "Field `PRU1_SD_ACC_SEL2` writer - "]
pub type Pru1SdAccSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_2` reader - "]
pub type Pru1FdZeroMinLimit2R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_2` writer - "]
pub type Pru1FdZeroMinLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MIN_2` reader - "]
pub type Pru1FdZeroMin2R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_2` writer - "]
pub type Pru1FdZeroMin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_2` reader - "]
pub type Pru1FdZeroMaxLimit2R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_2` writer - "]
pub type Pru1FdZeroMaxLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MAX_2` reader - "]
pub type Pru1FdZeroMax2R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_2` writer - "]
pub type Pru1FdZeroMax2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru1_sd_clk_sel2(&self) -> Pru1SdClkSel2R {
        Pru1SdClkSel2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru1_sd_clk_inv2(&self) -> Pru1SdClkInv2R {
        Pru1SdClkInv2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru1_sd_acc_sel2(&self) -> Pru1SdAccSel2R {
        Pru1SdAccSel2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_limit_2(&self) -> Pru1FdZeroMinLimit2R {
        Pru1FdZeroMinLimit2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_2(&self) -> Pru1FdZeroMin2R {
        Pru1FdZeroMin2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_limit_2(&self) -> Pru1FdZeroMaxLimit2R {
        Pru1FdZeroMaxLimit2R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_2(&self) -> Pru1FdZeroMax2R {
        Pru1FdZeroMax2R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_sel2(&mut self) -> Pru1SdClkSel2W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec> {
        Pru1SdClkSel2W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_inv2(&mut self) -> Pru1SdClkInv2W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec> {
        Pru1SdClkInv2W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_acc_sel2(&mut self) -> Pru1SdAccSel2W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec> {
        Pru1SdAccSel2W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_limit_2(
        &mut self,
    ) -> Pru1FdZeroMinLimit2W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec> {
        Pru1FdZeroMinLimit2W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_2(&mut self) -> Pru1FdZeroMin2W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec> {
        Pru1FdZeroMin2W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_limit_2(
        &mut self,
    ) -> Pru1FdZeroMaxLimit2W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec> {
        Pru1FdZeroMaxLimit2W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_2(&mut self) -> Pru1FdZeroMax2W<Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec> {
        Pru1FdZeroMax2W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg2::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg2 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdClkSelReg2Spec {
    const RESET_VALUE: u32 = 0;
}
