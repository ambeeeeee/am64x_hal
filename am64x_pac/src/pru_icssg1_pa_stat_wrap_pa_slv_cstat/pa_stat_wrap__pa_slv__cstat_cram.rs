#[doc = "Register `PA_STAT_WRAP__PA_SLV__CSTAT_CRAM` reader"]
pub type R = crate::R<PaStatWrap_PaSlv_CstatCramSpec>;
#[doc = "Register `PA_STAT_WRAP__PA_SLV__CSTAT_CRAM` writer"]
pub type W = crate::W<PaStatWrap_PaSlv_CstatCramSpec>;
#[doc = "Field `VALUE` reader - 31:0\\]
collect statistic"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - 31:0\\]
collect statistic"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
collect statistic"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
collect statistic"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<PaStatWrap_PaSlv_CstatCramSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "query mode RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_stat_wrap__pa_slv__cstat_cram::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_stat_wrap__pa_slv__cstat_cram::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PaStatWrap_PaSlv_CstatCramSpec;
impl crate::RegisterSpec for PaStatWrap_PaSlv_CstatCramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_stat_wrap__pa_slv__cstat_cram::R`](R) reader structure"]
impl crate::Readable for PaStatWrap_PaSlv_CstatCramSpec {}
#[doc = "`write(|w| ..)` method takes [`pa_stat_wrap__pa_slv__cstat_cram::W`](W) writer structure"]
impl crate::Writable for PaStatWrap_PaSlv_CstatCramSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA_STAT_WRAP__PA_SLV__CSTAT_CRAM to value 0"]
impl crate::Resettable for PaStatWrap_PaSlv_CstatCramSpec {
    const RESET_VALUE: u32 = 0;
}
