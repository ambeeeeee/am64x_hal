#[doc = "Register `REGS_sec_enable_clr_reg0` reader"]
pub type R = crate::R<RegsSecEnableClrReg0Spec>;
#[doc = "Register `REGS_sec_enable_clr_reg0` writer"]
pub type W = crate::W<RegsSecEnableClrReg0Spec>;
#[doc = "Field `ICB_RAMECC_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for icb_ramecc_pend"]
pub type IcbRameccEnableClrR = crate::BitReader;
#[doc = "Field `ICB_RAMECC_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for icb_ramecc_pend"]
pub type IcbRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITE_RAMECC_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for ite_ramecc_pend"]
pub type IteRameccEnableClrR = crate::BitReader;
#[doc = "Field `ITE_RAMECC_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for ite_ramecc_pend"]
pub type IteRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPI_RAMECC_ENABLE_CLR` reader - 2:2\\]
Interrupt Enable Clear Register for lpi_ramecc_pend"]
pub type LpiRameccEnableClrR = crate::BitReader;
#[doc = "Field `LPI_RAMECC_ENABLE_CLR` writer - 2:2\\]
Interrupt Enable Clear Register for lpi_ramecc_pend"]
pub type LpiRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for icb_ramecc_pend"]
    #[inline(always)]
    pub fn icb_ramecc_enable_clr(&self) -> IcbRameccEnableClrR {
        IcbRameccEnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ite_ramecc_pend"]
    #[inline(always)]
    pub fn ite_ramecc_enable_clr(&self) -> IteRameccEnableClrR {
        IteRameccEnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for lpi_ramecc_pend"]
    #[inline(always)]
    pub fn lpi_ramecc_enable_clr(&self) -> LpiRameccEnableClrR {
        LpiRameccEnableClrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for icb_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn icb_ramecc_enable_clr(&mut self) -> IcbRameccEnableClrW<RegsSecEnableClrReg0Spec> {
        IcbRameccEnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ite_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ite_ramecc_enable_clr(&mut self) -> IteRameccEnableClrW<RegsSecEnableClrReg0Spec> {
        IteRameccEnableClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for lpi_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_ramecc_enable_clr(&mut self) -> LpiRameccEnableClrW<RegsSecEnableClrReg0Spec> {
        LpiRameccEnableClrW::new(self, 2)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_sec_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsSecEnableClrReg0Spec;
impl crate::RegisterSpec for RegsSecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for RegsSecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`regs_sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for RegsSecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_sec_enable_clr_reg0 to value 0"]
impl crate::Resettable for RegsSecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
