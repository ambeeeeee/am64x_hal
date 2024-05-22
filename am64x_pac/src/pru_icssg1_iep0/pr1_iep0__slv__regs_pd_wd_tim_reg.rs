#[doc = "Register `PR1_IEP0__SLV__REGS_pd_wd_tim_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsPdWdTimRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_pd_wd_tim_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsPdWdTimRegSpec>;
#[doc = "Field `PD_WD_TIME` reader - "]
pub type PdWdTimeR = crate::FieldReader<u16>;
#[doc = "Field `PD_WD_TIME` writer - "]
pub type PdWdTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pd_wd_time(&self) -> PdWdTimeR {
        PdWdTimeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pd_wd_time(&mut self) -> PdWdTimeW<Pr1Iep0_Slv_RegsPdWdTimRegSpec> {
        PdWdTimeW::new(self, 0)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_pd_wd_tim_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_pd_wd_tim_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_pd_wd_tim_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsPdWdTimRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsPdWdTimRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_pd_wd_tim_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsPdWdTimRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_pd_wd_tim_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsPdWdTimRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_pd_wd_tim_reg to value 0x1000"]
impl crate::Resettable for Pr1Iep0_Slv_RegsPdWdTimRegSpec {
    const RESET_VALUE: u32 = 0x1000;
}
