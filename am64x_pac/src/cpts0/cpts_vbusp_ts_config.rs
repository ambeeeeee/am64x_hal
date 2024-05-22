#[doc = "Register `CPTS_VBUSP_TS_CONFIG` reader"]
pub type R = crate::R<CptsVbuspTsConfigSpec>;
#[doc = "Register `CPTS_VBUSP_TS_CONFIG` writer"]
pub type W = crate::W<CptsVbuspTsConfigSpec>;
#[doc = "Field `NUM_GENF` reader - 7:0\\]
The number of CPTS GENF outputs"]
pub type NumGenfR = crate::FieldReader;
#[doc = "Field `NUM_GENF` writer - 7:0\\]
The number of CPTS GENF outputs"]
pub type NumGenfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EVNT_FIFO_DEPTH` reader - 15:8\\]
The Event FIFO Depth"]
pub type EvntFifoDepthR = crate::FieldReader;
#[doc = "Field `EVNT_FIFO_DEPTH` writer - 15:8\\]
The Event FIFO Depth"]
pub type EvntFifoDepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    pub fn num_genf(&self) -> NumGenfR {
        NumGenfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The Event FIFO Depth"]
    #[inline(always)]
    pub fn evnt_fifo_depth(&self) -> EvntFifoDepthR {
        EvntFifoDepthR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    #[must_use]
    pub fn num_genf(&mut self) -> NumGenfW<CptsVbuspTsConfigSpec> {
        NumGenfW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The Event FIFO Depth"]
    #[inline(always)]
    #[must_use]
    pub fn evnt_fifo_depth(&mut self) -> EvntFifoDepthW<CptsVbuspTsConfigSpec> {
        EvntFifoDepthW::new(self, 8)
    }
}
#[doc = "Time Stamp Configuration Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsConfigSpec;
impl crate::RegisterSpec for CptsVbuspTsConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_config::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_config::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_CONFIG to value 0x3206"]
impl crate::Resettable for CptsVbuspTsConfigSpec {
    const RESET_VALUE: u32 = 0x3206;
}
