#[doc = "Register `CPTS_VBUSP_TS_PUSH_REG` reader"]
pub type R = crate::R<CptsVbuspTsPushRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_PUSH_REG` writer"]
pub type W = crate::W<CptsVbuspTsPushRegSpec>;
#[doc = "Field `TS_PUSH` reader - 0:0\\]
Time stamp event push"]
pub type TsPushR = crate::BitReader;
#[doc = "Field `TS_PUSH` writer - 0:0\\]
Time stamp event push"]
pub type TsPushW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time stamp event push"]
    #[inline(always)]
    pub fn ts_push(&self) -> TsPushR {
        TsPushR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time stamp event push"]
    #[inline(always)]
    #[must_use]
    pub fn ts_push(&mut self) -> TsPushW<CptsVbuspTsPushRegSpec> {
        TsPushW::new(self, 0)
    }
}
#[doc = "Time Stamp Event Push Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_push_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_push_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsPushRegSpec;
impl crate::RegisterSpec for CptsVbuspTsPushRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_push_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsPushRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_push_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsPushRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_PUSH_REG to value 0"]
impl crate::Resettable for CptsVbuspTsPushRegSpec {
    const RESET_VALUE: u32 = 0;
}
