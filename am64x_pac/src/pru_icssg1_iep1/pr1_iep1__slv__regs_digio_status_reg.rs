#[doc = "Register `PR1_IEP1__SLV__REGS_digio_status_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsDigioStatusRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_digio_status_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsDigioStatusRegSpec>;
#[doc = "Field `DIGIO_STAT` reader - "]
pub type DigioStatR = crate::FieldReader<u32>;
#[doc = "Field `DIGIO_STAT` writer - "]
pub type DigioStatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn digio_stat(&self) -> DigioStatR {
        DigioStatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn digio_stat(&mut self) -> DigioStatW<Pr1Iep1_Slv_RegsDigioStatusRegSpec> {
        DigioStatW::new(self, 0)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_digio_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsDigioStatusRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsDigioStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_digio_status_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsDigioStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_digio_status_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsDigioStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_digio_status_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsDigioStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
