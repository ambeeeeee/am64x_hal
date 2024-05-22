#[doc = "Register `ECC_AGGR_RXMEM__CFG__REGS_ded_eoi_reg` reader"]
pub type R = crate::R<EccAggrRxmem_Cfg_RegsDedEoiRegSpec>;
#[doc = "Register `ECC_AGGR_RXMEM__CFG__REGS_ded_eoi_reg` writer"]
pub type W = crate::W<EccAggrRxmem_Cfg_RegsDedEoiRegSpec>;
#[doc = "Field `EOI_WR` reader - 0:0\\]
EOI Register"]
pub type EoiWrR = crate::BitReader;
#[doc = "Field `EOI_WR` writer - 0:0\\]
EOI Register"]
pub type EoiWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EOI Register"]
    #[inline(always)]
    pub fn eoi_wr(&self) -> EoiWrR {
        EoiWrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EOI Register"]
    #[inline(always)]
    #[must_use]
    pub fn eoi_wr(&mut self) -> EoiWrW<EccAggrRxmem_Cfg_RegsDedEoiRegSpec> {
        EoiWrW::new(self, 0)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_rxmem__cfg__regs_ded_eoi_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_rxmem__cfg__regs_ded_eoi_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrRxmem_Cfg_RegsDedEoiRegSpec;
impl crate::RegisterSpec for EccAggrRxmem_Cfg_RegsDedEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_rxmem__cfg__regs_ded_eoi_reg::R`](R) reader structure"]
impl crate::Readable for EccAggrRxmem_Cfg_RegsDedEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_rxmem__cfg__regs_ded_eoi_reg::W`](W) writer structure"]
impl crate::Writable for EccAggrRxmem_Cfg_RegsDedEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_RXMEM__CFG__REGS_ded_eoi_reg to value 0"]
impl crate::Resettable for EccAggrRxmem_Cfg_RegsDedEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
