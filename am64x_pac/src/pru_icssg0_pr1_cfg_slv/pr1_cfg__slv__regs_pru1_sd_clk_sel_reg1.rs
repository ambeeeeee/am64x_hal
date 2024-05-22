#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec>;
#[doc = "Field `PRU1_SD_CLK_SEL1` reader - "]
pub type Pru1SdClkSel1R = crate::FieldReader;
#[doc = "Field `PRU1_SD_CLK_SEL1` writer - "]
pub type Pru1SdClkSel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_SD_CLK_INV1` reader - "]
pub type Pru1SdClkInv1R = crate::BitReader;
#[doc = "Field `PRU1_SD_CLK_INV1` writer - "]
pub type Pru1SdClkInv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_ACC_SEL1` reader - "]
pub type Pru1SdAccSel1R = crate::FieldReader;
#[doc = "Field `PRU1_SD_ACC_SEL1` writer - "]
pub type Pru1SdAccSel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_1` reader - "]
pub type Pru1FdZeroMinLimit1R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_1` writer - "]
pub type Pru1FdZeroMinLimit1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MIN_1` reader - "]
pub type Pru1FdZeroMin1R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_1` writer - "]
pub type Pru1FdZeroMin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_1` reader - "]
pub type Pru1FdZeroMaxLimit1R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_1` writer - "]
pub type Pru1FdZeroMaxLimit1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MAX_1` reader - "]
pub type Pru1FdZeroMax1R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_1` writer - "]
pub type Pru1FdZeroMax1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru1_sd_clk_sel1(&self) -> Pru1SdClkSel1R {
        Pru1SdClkSel1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru1_sd_clk_inv1(&self) -> Pru1SdClkInv1R {
        Pru1SdClkInv1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru1_sd_acc_sel1(&self) -> Pru1SdAccSel1R {
        Pru1SdAccSel1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_limit_1(&self) -> Pru1FdZeroMinLimit1R {
        Pru1FdZeroMinLimit1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_1(&self) -> Pru1FdZeroMin1R {
        Pru1FdZeroMin1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_limit_1(&self) -> Pru1FdZeroMaxLimit1R {
        Pru1FdZeroMaxLimit1R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_1(&self) -> Pru1FdZeroMax1R {
        Pru1FdZeroMax1R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_sel1(&mut self) -> Pru1SdClkSel1W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec> {
        Pru1SdClkSel1W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_inv1(&mut self) -> Pru1SdClkInv1W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec> {
        Pru1SdClkInv1W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_acc_sel1(&mut self) -> Pru1SdAccSel1W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec> {
        Pru1SdAccSel1W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_limit_1(
        &mut self,
    ) -> Pru1FdZeroMinLimit1W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec> {
        Pru1FdZeroMinLimit1W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_1(&mut self) -> Pru1FdZeroMin1W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec> {
        Pru1FdZeroMin1W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_limit_1(
        &mut self,
    ) -> Pru1FdZeroMaxLimit1W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec> {
        Pru1FdZeroMaxLimit1W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_1(&mut self) -> Pru1FdZeroMax1W<Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec> {
        Pru1FdZeroMax1W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg1::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg1 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdClkSelReg1Spec {
    const RESET_VALUE: u32 = 0;
}
