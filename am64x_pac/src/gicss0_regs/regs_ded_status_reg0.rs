#[doc = "Register `REGS_ded_status_reg0` reader"]
pub type R = crate::R<RegsDedStatusReg0Spec>;
#[doc = "Register `REGS_ded_status_reg0` writer"]
pub type W = crate::W<RegsDedStatusReg0Spec>;
#[doc = "Field `ICB_RAMECC_PEND` reader - 0:0\\]
Interrupt Pending Status for icb_ramecc_pend"]
pub type IcbRameccPendR = crate::BitReader;
#[doc = "Field `ICB_RAMECC_PEND` writer - 0:0\\]
Interrupt Pending Status for icb_ramecc_pend"]
pub type IcbRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITE_RAMECC_PEND` reader - 1:1\\]
Interrupt Pending Status for ite_ramecc_pend"]
pub type IteRameccPendR = crate::BitReader;
#[doc = "Field `ITE_RAMECC_PEND` writer - 1:1\\]
Interrupt Pending Status for ite_ramecc_pend"]
pub type IteRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPI_RAMECC_PEND` reader - 2:2\\]
Interrupt Pending Status for lpi_ramecc_pend"]
pub type LpiRameccPendR = crate::BitReader;
#[doc = "Field `LPI_RAMECC_PEND` writer - 2:2\\]
Interrupt Pending Status for lpi_ramecc_pend"]
pub type LpiRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for icb_ramecc_pend"]
    #[inline(always)]
    pub fn icb_ramecc_pend(&self) -> IcbRameccPendR {
        IcbRameccPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ite_ramecc_pend"]
    #[inline(always)]
    pub fn ite_ramecc_pend(&self) -> IteRameccPendR {
        IteRameccPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for lpi_ramecc_pend"]
    #[inline(always)]
    pub fn lpi_ramecc_pend(&self) -> LpiRameccPendR {
        LpiRameccPendR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for icb_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn icb_ramecc_pend(&mut self) -> IcbRameccPendW<RegsDedStatusReg0Spec> {
        IcbRameccPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ite_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ite_ramecc_pend(&mut self) -> IteRameccPendW<RegsDedStatusReg0Spec> {
        IteRameccPendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for lpi_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_ramecc_pend(&mut self) -> LpiRameccPendW<RegsDedStatusReg0Spec> {
        LpiRameccPendW::new(self, 2)
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
