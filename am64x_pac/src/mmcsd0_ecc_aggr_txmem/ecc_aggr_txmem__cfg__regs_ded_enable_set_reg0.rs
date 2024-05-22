#[doc = "Register `ECC_AGGR_TXMEM__CFG__REGS_ded_enable_set_reg0` reader"]
pub type R = crate::R<EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec>;
#[doc = "Register `ECC_AGGR_TXMEM__CFG__REGS_ded_enable_set_reg0` writer"]
pub type W = crate::W<EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec>;
#[doc = "Field `TXMEM_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for txmem_pend"]
pub type TxmemEnableSetR = crate::BitReader;
#[doc = "Field `TXMEM_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for txmem_pend"]
pub type TxmemEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for txmem_pend"]
    #[inline(always)]
    pub fn txmem_enable_set(&self) -> TxmemEnableSetR {
        TxmemEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for txmem_pend"]
    #[inline(always)]
    #[must_use]
    pub fn txmem_enable_set(
        &mut self,
    ) -> TxmemEnableSetW<EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec> {
        TxmemEnableSetW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_txmem__cfg__regs_ded_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_txmem__cfg__regs_ded_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec;
impl crate::RegisterSpec for EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_txmem__cfg__regs_ded_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_txmem__cfg__regs_ded_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_TXMEM__CFG__REGS_ded_enable_set_reg0 to value 0"]
impl crate::Resettable for EccAggrTxmem_Cfg_RegsDedEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
