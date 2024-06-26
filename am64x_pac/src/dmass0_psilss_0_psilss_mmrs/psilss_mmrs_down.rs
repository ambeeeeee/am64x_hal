#[doc = "Register `PSILSS_MMRS_down` reader"]
pub type R = crate::R<PsilssMmrsDownSpec>;
#[doc = "Register `PSILSS_MMRS_down` writer"]
pub type W = crate::W<PsilssMmrsDownSpec>;
#[doc = "Field `STATUS` reader - 31:0\\]
The down status of the endpoint links."]
pub type StatusR = crate::FieldReader<u32>;
#[doc = "Field `STATUS` writer - 31:0\\]
The down status of the endpoint links."]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The down status of the endpoint links."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The down status of the endpoint links."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<PsilssMmrsDownSpec> {
        StatusW::new(self, 0)
    }
}
#[doc = "The Link Down Register shows which links are down for the endpoints.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilss_mmrs_down::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilss_mmrs_down::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsilssMmrsDownSpec;
impl crate::RegisterSpec for PsilssMmrsDownSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psilss_mmrs_down::R`](R) reader structure"]
impl crate::Readable for PsilssMmrsDownSpec {}
#[doc = "`write(|w| ..)` method takes [`psilss_mmrs_down::W`](W) writer structure"]
impl crate::Writable for PsilssMmrsDownSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSILSS_MMRS_down to value 0"]
impl crate::Resettable for PsilssMmrsDownSpec {
    const RESET_VALUE: u32 = 0;
}
