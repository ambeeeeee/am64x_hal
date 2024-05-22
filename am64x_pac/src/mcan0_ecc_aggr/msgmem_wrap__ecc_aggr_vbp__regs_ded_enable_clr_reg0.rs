#[doc = "Register `MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_enable_clr_reg0` reader"]
pub type R = crate::R<MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec>;
#[doc = "Register `MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_enable_clr_reg0` writer"]
pub type W = crate::W<MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec>;
#[doc = "Field `MSGMEM_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend"]
pub type MsgmemEnableClrR = crate::BitReader;
#[doc = "Field `MSGMEM_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend"]
pub type MsgmemEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend"]
pub type CtrlEdcVbussEnableClrR = crate::BitReader;
#[doc = "Field `CTRL_EDC_VBUSS_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend"]
pub type CtrlEdcVbussEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend"]
    #[inline(always)]
    pub fn msgmem_enable_clr(&self) -> MsgmemEnableClrR {
        MsgmemEnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend"]
    #[inline(always)]
    pub fn ctrl_edc_vbuss_enable_clr(&self) -> CtrlEdcVbussEnableClrR {
        CtrlEdcVbussEnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for msgmem_pend"]
    #[inline(always)]
    #[must_use]
    pub fn msgmem_enable_clr(
        &mut self,
    ) -> MsgmemEnableClrW<MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec> {
        MsgmemEnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for ctrl_edc_vbuss_pend"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_edc_vbuss_enable_clr(
        &mut self,
    ) -> CtrlEdcVbussEnableClrW<MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec> {
        CtrlEdcVbussEnableClrW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec;
impl crate::RegisterSpec for MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`msgmem_wrap__ecc_aggr_vbp__regs_ded_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSGMEM_WRAP__ECC_AGGR_VBP__REGS_ded_enable_clr_reg0 to value 0"]
impl crate::Resettable for MsgmemWrap_EccAggrVbp_RegsDedEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
