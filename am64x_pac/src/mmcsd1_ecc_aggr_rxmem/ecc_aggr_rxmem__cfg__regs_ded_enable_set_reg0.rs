#[doc = "Register `ECC_AGGR_RXMEM__CFG__REGS_ded_enable_set_reg0` reader"]
pub type R = crate::R<EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec>;
#[doc = "Register `ECC_AGGR_RXMEM__CFG__REGS_ded_enable_set_reg0` writer"]
pub type W = crate::W<EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec>;
#[doc = "Field `RXMEM_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for rxmem_pend"]
pub type RxmemEnableSetR = crate::BitReader;
#[doc = "Field `RXMEM_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for rxmem_pend"]
pub type RxmemEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for rxmem_pend"]
    #[inline(always)]
    pub fn rxmem_enable_set(&self) -> RxmemEnableSetR {
        RxmemEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for rxmem_pend"]
    #[inline(always)]
    #[must_use]
    pub fn rxmem_enable_set(
        &mut self,
    ) -> RxmemEnableSetW<EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec> {
        RxmemEnableSetW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec;
impl crate::RegisterSpec for EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_rxmem__cfg__regs_ded_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_RXMEM__CFG__REGS_ded_enable_set_reg0 to value 0"]
impl crate::Resettable for EccAggrRxmem_Cfg_RegsDedEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
