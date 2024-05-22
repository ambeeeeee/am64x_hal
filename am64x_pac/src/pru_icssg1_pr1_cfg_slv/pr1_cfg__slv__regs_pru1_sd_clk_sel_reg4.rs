#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec>;
#[doc = "Field `PRU1_SD_CLK_SEL4` reader - "]
pub type Pru1SdClkSel4R = crate::FieldReader;
#[doc = "Field `PRU1_SD_CLK_SEL4` writer - "]
pub type Pru1SdClkSel4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_SD_CLK_INV4` reader - "]
pub type Pru1SdClkInv4R = crate::BitReader;
#[doc = "Field `PRU1_SD_CLK_INV4` writer - "]
pub type Pru1SdClkInv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_ACC_SEL4` reader - "]
pub type Pru1SdAccSel4R = crate::FieldReader;
#[doc = "Field `PRU1_SD_ACC_SEL4` writer - "]
pub type Pru1SdAccSel4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_4` reader - "]
pub type Pru1FdZeroMinLimit4R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_LIMIT_4` writer - "]
pub type Pru1FdZeroMinLimit4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MIN_4` reader - "]
pub type Pru1FdZeroMin4R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MIN_4` writer - "]
pub type Pru1FdZeroMin4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_4` reader - "]
pub type Pru1FdZeroMaxLimit4R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_LIMIT_4` writer - "]
pub type Pru1FdZeroMaxLimit4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ZERO_MAX_4` reader - "]
pub type Pru1FdZeroMax4R = crate::BitReader;
#[doc = "Field `PRU1_FD_ZERO_MAX_4` writer - "]
pub type Pru1FdZeroMax4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pru1_sd_clk_sel4(&self) -> Pru1SdClkSel4R {
        Pru1SdClkSel4R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pru1_sd_clk_inv4(&self) -> Pru1SdClkInv4R {
        Pru1SdClkInv4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pru1_sd_acc_sel4(&self) -> Pru1SdAccSel4R {
        Pru1SdAccSel4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_limit_4(&self) -> Pru1FdZeroMinLimit4R {
        Pru1FdZeroMinLimit4R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_zero_min_4(&self) -> Pru1FdZeroMin4R {
        Pru1FdZeroMin4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_limit_4(&self) -> Pru1FdZeroMaxLimit4R {
        Pru1FdZeroMaxLimit4R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_zero_max_4(&self) -> Pru1FdZeroMax4R {
        Pru1FdZeroMax4R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_sel4(&mut self) -> Pru1SdClkSel4W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec> {
        Pru1SdClkSel4W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_clk_inv4(&mut self) -> Pru1SdClkInv4W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec> {
        Pru1SdClkInv4W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_acc_sel4(&mut self) -> Pru1SdAccSel4W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec> {
        Pru1SdAccSel4W::new(self, 4)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_limit_4(
        &mut self,
    ) -> Pru1FdZeroMinLimit4W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec> {
        Pru1FdZeroMinLimit4W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_min_4(&mut self) -> Pru1FdZeroMin4W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec> {
        Pru1FdZeroMin4W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_limit_4(
        &mut self,
    ) -> Pru1FdZeroMaxLimit4W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec> {
        Pru1FdZeroMaxLimit4W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_zero_max_4(&mut self) -> Pru1FdZeroMax4W<Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec> {
        Pru1FdZeroMax4W::new(self, 22)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_clk_sel_reg4::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_clk_sel_reg4 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdClkSelReg4Spec {
    const RESET_VALUE: u32 = 0;
}
