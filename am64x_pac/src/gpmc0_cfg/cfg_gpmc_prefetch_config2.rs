#[doc = "Register `CFG_GPMC_PREFETCH_CONFIG2` reader"]
pub type R = crate::R<CfgGpmcPrefetchConfig2Spec>;
#[doc = "Register `CFG_GPMC_PREFETCH_CONFIG2` writer"]
pub type W = crate::W<CfgGpmcPrefetchConfig2Spec>;
#[doc = "Field `TRANSFERCOUNT` reader - 13:0\\]
Selects the number of bytes to be read or written by the engine to the selected CS \\[0x0000 corresponds to 0 byte, 0x0001 corresponds to 1 byte, &amp;, 0x2000 corresponds to 8 Kbytes\\]"]
pub type TransfercountR = crate::FieldReader<u16>;
#[doc = "Field `TRANSFERCOUNT` writer - 13:0\\]
Selects the number of bytes to be read or written by the engine to the selected CS \\[0x0000 corresponds to 0 byte, 0x0001 corresponds to 1 byte, &amp;, 0x2000 corresponds to 8 Kbytes\\]"]
pub type TransfercountW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Selects the number of bytes to be read or written by the engine to the selected CS \\[0x0000 corresponds to 0 byte, 0x0001 corresponds to 1 byte, &amp;, 0x2000 corresponds to 8 Kbytes\\]"]
    #[inline(always)]
    pub fn transfercount(&self) -> TransfercountR {
        TransfercountR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Selects the number of bytes to be read or written by the engine to the selected CS \\[0x0000 corresponds to 0 byte, 0x0001 corresponds to 1 byte, &amp;, 0x2000 corresponds to 8 Kbytes\\]"]
    #[inline(always)]
    #[must_use]
    pub fn transfercount(&mut self) -> TransfercountW<CfgGpmcPrefetchConfig2Spec> {
        TransfercountW::new(self, 0)
    }
}
#[doc = "Prefetch engine configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_prefetch_config2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_prefetch_config2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcPrefetchConfig2Spec;
impl crate::RegisterSpec for CfgGpmcPrefetchConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_prefetch_config2::R`](R) reader structure"]
impl crate::Readable for CfgGpmcPrefetchConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_prefetch_config2::W`](W) writer structure"]
impl crate::Writable for CfgGpmcPrefetchConfig2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_PREFETCH_CONFIG2 to value 0"]
impl crate::Resettable for CfgGpmcPrefetchConfig2Spec {
    const RESET_VALUE: u32 = 0;
}
