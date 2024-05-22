#[doc = "Register `ETLSW_MMRS_config` reader"]
pub type R = crate::R<EtlswMmrsConfigSpec>;
#[doc = "Register `ETLSW_MMRS_config` writer"]
pub type W = crate::W<EtlswMmrsConfigSpec>;
#[doc = "Field `ENDPOINTS` reader - 15:0\\]
Number of endpoints supported."]
pub type EndpointsR = crate::FieldReader<u16>;
#[doc = "Field `ENDPOINTS` writer - 15:0\\]
Number of endpoints supported."]
pub type EndpointsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of endpoints supported."]
    #[inline(always)]
    pub fn endpoints(&self) -> EndpointsR {
        EndpointsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Number of endpoints supported."]
    #[inline(always)]
    #[must_use]
    pub fn endpoints(&mut self) -> EndpointsW<EtlswMmrsConfigSpec> {
        EndpointsW::new(self, 0)
    }
}
#[doc = "The Config Register shows configured params.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtlswMmrsConfigSpec;
impl crate::RegisterSpec for EtlswMmrsConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etlsw_mmrs_config::R`](R) reader structure"]
impl crate::Readable for EtlswMmrsConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`etlsw_mmrs_config::W`](W) writer structure"]
impl crate::Writable for EtlswMmrsConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETLSW_MMRS_config to value 0x04"]
impl crate::Resettable for EtlswMmrsConfigSpec {
    const RESET_VALUE: u32 = 0x04;
}
