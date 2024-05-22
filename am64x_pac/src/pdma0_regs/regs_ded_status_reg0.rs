#[doc = "Register `REGS_ded_status_reg0` reader"]
pub type R = crate::R<RegsDedStatusReg0Spec>;
#[doc = "Register `REGS_ded_status_reg0` writer"]
pub type W = crate::W<RegsDedStatusReg0Spec>;
#[doc = "Field `TPCF0_RAMECC_PEND` reader - 0:0\\]
Interrupt Pending Status for tpcf0_ramecc_pend"]
pub type Tpcf0RameccPendR = crate::BitReader;
#[doc = "Field `TPCF0_RAMECC_PEND` writer - 0:0\\]
Interrupt Pending Status for tpcf0_ramecc_pend"]
pub type Tpcf0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPCF1_RAMECC_PEND` reader - 1:1\\]
Interrupt Pending Status for tpcf1_ramecc_pend"]
pub type Tpcf1RameccPendR = crate::BitReader;
#[doc = "Field `TPCF1_RAMECC_PEND` writer - 1:1\\]
Interrupt Pending Status for tpcf1_ramecc_pend"]
pub type Tpcf1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCF0_RAMECC_PEND` reader - 2:2\\]
Interrupt Pending Status for rpcf0_ramecc_pend"]
pub type Rpcf0RameccPendR = crate::BitReader;
#[doc = "Field `RPCF0_RAMECC_PEND` writer - 2:2\\]
Interrupt Pending Status for rpcf0_ramecc_pend"]
pub type Rpcf0RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPCF1_RAMECC_PEND` reader - 3:3\\]
Interrupt Pending Status for rpcf1_ramecc_pend"]
pub type Rpcf1RameccPendR = crate::BitReader;
#[doc = "Field `RPCF1_RAMECC_PEND` writer - 3:3\\]
Interrupt Pending Status for rpcf1_ramecc_pend"]
pub type Rpcf1RameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for tpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn tpcf0_ramecc_pend(&self) -> Tpcf0RameccPendR {
        Tpcf0RameccPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for tpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn tpcf1_ramecc_pend(&self) -> Tpcf1RameccPendR {
        Tpcf1RameccPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for rpcf0_ramecc_pend"]
    #[inline(always)]
    pub fn rpcf0_ramecc_pend(&self) -> Rpcf0RameccPendR {
        Rpcf0RameccPendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for rpcf1_ramecc_pend"]
    #[inline(always)]
    pub fn rpcf1_ramecc_pend(&self) -> Rpcf1RameccPendR {
        Rpcf1RameccPendR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for tpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn tpcf0_ramecc_pend(&mut self) -> Tpcf0RameccPendW<RegsDedStatusReg0Spec> {
        Tpcf0RameccPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for tpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn tpcf1_ramecc_pend(&mut self) -> Tpcf1RameccPendW<RegsDedStatusReg0Spec> {
        Tpcf1RameccPendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for rpcf0_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn rpcf0_ramecc_pend(&mut self) -> Rpcf0RameccPendW<RegsDedStatusReg0Spec> {
        Rpcf0RameccPendW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for rpcf1_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn rpcf1_ramecc_pend(&mut self) -> Rpcf1RameccPendW<RegsDedStatusReg0Spec> {
        Rpcf1RameccPendW::new(self, 3)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_ded_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_ded_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsDedStatusReg0Spec;
impl crate::RegisterSpec for RegsDedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for RegsDedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`regs_ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for RegsDedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_ded_status_reg0 to value 0"]
impl crate::Resettable for RegsDedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
