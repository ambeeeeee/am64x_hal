#[doc = "Register `ECC_AGGR_REGSREGS_sec_status_reg0` reader"]
pub type R = crate::R<EccAggrRegsregsSecStatusReg0Spec>;
#[doc = "Register `ECC_AGGR_REGSREGS_sec_status_reg0` writer"]
pub type W = crate::W<EccAggrRegsregsSecStatusReg0Spec>;
#[doc = "Field `RAMECC0_PEND` reader - 0:0\\]
Interrupt Pending Status for ramecc0_pend"]
pub type Ramecc0PendR = crate::BitReader;
#[doc = "Field `RAMECC0_PEND` writer - 0:0\\]
Interrupt Pending Status for ramecc0_pend"]
pub type Ramecc0PendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for ramecc0_pend"]
    #[inline(always)]
    pub fn ramecc0_pend(&self) -> Ramecc0PendR {
        Ramecc0PendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for ramecc0_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ramecc0_pend(&mut self) -> Ramecc0PendW<EccAggrRegsregsSecStatusReg0Spec> {
        Ramecc0PendW::new(self, 0)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_regsregs_sec_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_regsregs_sec_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrRegsregsSecStatusReg0Spec;
impl crate::RegisterSpec for EccAggrRegsregsSecStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_regsregs_sec_status_reg0::R`](R) reader structure"]
impl crate::Readable for EccAggrRegsregsSecStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_regsregs_sec_status_reg0::W`](W) writer structure"]
impl crate::Writable for EccAggrRegsregsSecStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_REGSREGS_sec_status_reg0 to value 0"]
impl crate::Resettable for EccAggrRegsregsSecStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
