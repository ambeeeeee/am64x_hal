#[doc = "Register `REGS_sec_enable_clr_reg0` reader"]
pub type R = crate::R<RegsSecEnableClrReg0Spec>;
#[doc = "Register `REGS_sec_enable_clr_reg0` writer"]
pub type W = crate::W<RegsSecEnableClrReg0Spec>;
#[doc = "Field `TPCF0_RAMECC_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for tpcf0_ramecc_pend"]
pub type Tpcf0RameccEnableClrR = crate::BitReader;
#[doc = "Field `TPCF0_RAMECC_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for tpcf0_ramecc_pend"]
pub type Tpcf0RameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPCF1_RAMECC_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for tpcf1_ramecc_pend"]
pub type Tpcf1RameccEnableClrR = crate::BitReader;
#[doc = "Field `TPCF1_RAMECC_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for tpcf1_ramecc_pend"]
pub type Tpcf1RameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCF0_RAMECC_ENABLE_CLR` reader - 2:2\\]
Interrupt Enable Clear Register for rpcf0_ramecc_pend"]
pub type Rpcf0RameccEnableClrR = crate::BitReader;
#[doc = "Field `RPCF0_RAMECC_ENABLE_CLR` writer - 2:2\\]
Interrupt Enable Clear Register for rpcf0_ramecc_pend"]
pub type Rpcf0RameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCF1_RAMECC_ENABLE_CLR` reader - 3:3\\]
Interrupt Enable Clear Register for rpcf1_ramecc_pend"]
pub type Rpcf1RameccEnableClrR = crate::BitReader;
#[doc = "Field `RPCF1_RAMECC_ENABLE_CLR` writer - 3:3\\]
Interrupt Enable Clear Register for rpcf1_ramecc_pend"]
pub type Rpcf1RameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for tpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn tpcf0_ramecc_enable_clr(&self) -> Tpcf0RameccEnableClrR {
        Tpcf0RameccEnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for tpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn tpcf1_ramecc_enable_clr(&self) -> Tpcf1RameccEnableClrR {
        Tpcf1RameccEnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for rpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn rpcf0_ramecc_enable_clr(&self) -> Rpcf0RameccEnableClrR {
        Rpcf0RameccEnableClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for rpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn rpcf1_ramecc_enable_clr(&self) -> Rpcf1RameccEnableClrR {
        Rpcf1RameccEnableClrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for tpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn tpcf0_ramecc_enable_clr(&mut self) -> Tpcf0RameccEnableClrW<RegsSecEnableClrReg0Spec> {
        Tpcf0RameccEnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for tpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn tpcf1_ramecc_enable_clr(&mut self) -> Tpcf1RameccEnableClrW<RegsSecEnableClrReg0Spec> {
        Tpcf1RameccEnableClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for rpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn rpcf0_ramecc_enable_clr(&mut self) -> Rpcf0RameccEnableClrW<RegsSecEnableClrReg0Spec> {
        Rpcf0RameccEnableClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for rpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn rpcf1_ramecc_enable_clr(&mut self) -> Rpcf1RameccEnableClrW<RegsSecEnableClrReg0Spec> {
        Rpcf1RameccEnableClrW::new(self, 3)
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
