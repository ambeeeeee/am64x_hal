#[doc = "Register `ETLSW_MMRS_link` reader"]
pub type R = crate::R<EtlswMmrsLinkSpec>;
#[doc = "Register `ETLSW_MMRS_link` writer"]
pub type W = crate::W<EtlswMmrsLinkSpec>;
#[doc = "Field `STATUS` reader - 31:0\\]
The status of the endpoint links."]
pub type StatusR = crate::FieldReader<u32>;
#[doc = "Field `STATUS` writer - 31:0\\]
The status of the endpoint links."]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The status of the endpoint links."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The status of the endpoint links."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<EtlswMmrsLinkSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "The Link Register shows the current status of the endpoint links.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtlswMmrsLinkSpec;
impl crate::RegisterSpec for EtlswMmrsLinkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etlsw_mmrs_link::R`](R) reader structure"]
impl crate::Readable for EtlswMmrsLinkSpec {}
#[doc = "`write(|w| ..)` method takes [`etlsw_mmrs_link::W`](W) writer structure"]
impl crate::Writable for EtlswMmrsLinkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETLSW_MMRS_link to value 0"]
impl crate::Resettable for EtlswMmrsLinkSpec {
    const RESET_VALUE: u32 = 0;
}
