#[doc = "Register `MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_status_reg0` reader"]
pub type R = crate::R<MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec>;
#[doc = "Register `MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_status_reg0` writer"]
pub type W = crate::W<MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec>;
#[doc = "Field `MSGMEM_PEND` reader - 0:0\\]
Interrupt Pending Status for msgmem_pend"]
pub type MsgmemPendR = crate::BitReader;
#[doc = "Field `MSGMEM_PEND` writer - 0:0\\]
Interrupt Pending Status for msgmem_pend"]
pub type MsgmemPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_PEND` reader - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend"]
pub type CtrlEdcVbussPendR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_PEND` writer - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend"]
pub type CtrlEdcVbussPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for msgmem_pend"]
    #[inline(always)]
    pub fn msgmem_pend(&self) -> MsgmemPendR {
        MsgmemPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend"]
    #[inline(always)]
    pub fn ctrl_edc_vbuss_pend(&self) -> CtrlEdcVbussPendR {
        CtrlEdcVbussPendR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for msgmem_pend"]
    #[inline(always)]
    #[must_use]
    pub fn msgmem_pend(&mut self) -> MsgmemPendW<MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec> {
        MsgmemPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for ctrl_edc_vbuss_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_pend(
        &mut self,
    ) -> CtrlEdcVbussPendW<MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec> {
        CtrlEdcVbussPendW::new(self, 1)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec;
impl crate::RegisterSpec for MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0::R`](R) reader structure"]
impl crate::Readable for MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`msgmem_wrap__ecc_aggr_vbp__regs_sec_status_reg0::W`](W) writer structure"]
impl crate::Writable for MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSGMEM_WRAP__ECC_AGGR_VBP__REGS_sec_status_reg0 to value 0"]
impl crate::Resettable for MsgmemWrap_EccAggrVbp_RegsSecStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}
