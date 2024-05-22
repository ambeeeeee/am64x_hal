#[doc = "Register `PR1_IEP1__SLV__REGS_digio_data_out_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsDigioDataOutRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_digio_data_out_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsDigioDataOutRegSpec>;
#[doc = "Field `DATA_OUT` reader - "]
pub type DataOutR = crate::FieldReader<u32>;
#[doc = "Field `DATA_OUT` writer - "]
pub type DataOutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn data_out(&self) -> DataOutR {
        DataOutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn data_out(&mut self) -> DataOutW<Pr1Iep1_Slv_RegsDigioDataOutRegSpec> {
        DataOutW::new(self, 0)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_digio_data_out_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_digio_data_out_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_digio_data_out_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsDigioDataOutRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsDigioDataOutRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_digio_data_out_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsDigioDataOutRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_digio_data_out_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsDigioDataOutRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_digio_data_out_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsDigioDataOutRegSpec {
    const RESET_VALUE: u32 = 0;
}
