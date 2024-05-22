#[doc = "Register `PR1_IEP1__SLV__REGS_wd_prediv_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsWdPredivRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_wd_prediv_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsWdPredivRegSpec>;
#[doc = "Field `PRE_DIV` reader - "]
pub type PreDivR = crate::FieldReader<u16>;
#[doc = "Field `PRE_DIV` writer - "]
pub type PreDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pre_div(&self) -> PreDivR {
        PreDivR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn pre_div(&mut self) -> PreDivW<Pr1Iep1_Slv_RegsWdPredivRegSpec> {
        PreDivW::new(self, 0)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_wd_prediv_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_wd_prediv_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_wd_prediv_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsWdPredivRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsWdPredivRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_wd_prediv_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsWdPredivRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_wd_prediv_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsWdPredivRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_wd_prediv_reg to value 0x0002_0000"]
impl crate::Resettable for Pr1Iep1_Slv_RegsWdPredivRegSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
