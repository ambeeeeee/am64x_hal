#[doc = "Register `CPTS_VBUSP_TS_LOAD_EN_REG` reader"]
pub type R = crate::R<CptsVbuspTsLoadEnRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_LOAD_EN_REG` writer"]
pub type W = crate::W<CptsVbuspTsLoadEnRegSpec>;
#[doc = "Field `TS_LOAD_EN` reader - 0:0\\]
Time stamp load enable"]
pub type TsLoadEnR = crate::BitReader;
#[doc = "Field `TS_LOAD_EN` writer - 0:0\\]
Time stamp load enable"]
pub type TsLoadEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time stamp load enable"]
    #[inline(always)]
    pub fn ts_load_en(&self) -> TsLoadEnR {
        TsLoadEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time stamp load enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts_load_en(&mut self) -> TsLoadEnW<CptsVbuspTsLoadEnRegSpec> {
        TsLoadEnW::new(self, 0)
    }
}
#[doc = "Time Stamp Load Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_load_en_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_load_en_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsLoadEnRegSpec;
impl crate::RegisterSpec for CptsVbuspTsLoadEnRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_load_en_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsLoadEnRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_load_en_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsLoadEnRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_LOAD_EN_REG to value 0"]
impl crate::Resettable for CptsVbuspTsLoadEnRegSpec {
    const RESET_VALUE: u32 = 0;
}
