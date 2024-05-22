#[doc = "Register `ECC_AGGR_CORE0_REGS_sec_eoi_reg` reader"]
pub type R = crate::R<EccAggrCore0RegsSecEoiRegSpec>;
#[doc = "Register `ECC_AGGR_CORE0_REGS_sec_eoi_reg` writer"]
pub type W = crate::W<EccAggrCore0RegsSecEoiRegSpec>;
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
    pub fn eoi_wr(&mut self) -> EoiWrW<EccAggrCore0RegsSecEoiRegSpec> {
        EoiWrW::new(self, 0)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_aggr_core0_regs_sec_eoi_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_aggr_core0_regs_sec_eoi_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccAggrCore0RegsSecEoiRegSpec;
impl crate::RegisterSpec for EccAggrCore0RegsSecEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_aggr_core0_regs_sec_eoi_reg::R`](R) reader structure"]
impl crate::Readable for EccAggrCore0RegsSecEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_aggr_core0_regs_sec_eoi_reg::W`](W) writer structure"]
impl crate::Writable for EccAggrCore0RegsSecEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_AGGR_CORE0_REGS_sec_eoi_reg to value 0"]
impl crate::Resettable for EccAggrCore0RegsSecEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
