#[doc = "Register `CFG0_BOOT_PROGRESS_PROXY` reader"]
pub type R = crate::R<Cfg0BootProgressProxySpec>;
#[doc = "Register `CFG0_BOOT_PROGRESS_PROXY` writer"]
pub type W = crate::W<Cfg0BootProgressProxySpec>;
#[doc = "Field `BOOT_PROGRESS_PROGRESS_PROXY` reader - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
pub type BootProgressProgressProxyR = crate::FieldReader<u32>;
#[doc = "Field `BOOT_PROGRESS_PROGRESS_PROXY` writer - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
pub type BootProgressProgressProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
    #[inline(always)]
    pub fn boot_progress_progress_proxy(&self) -> BootProgressProgressProxyR {
        BootProgressProgressProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
    #[inline(always)]
    #[must_use]
    pub fn boot_progress_progress_proxy(
        &mut self,
    ) -> BootProgressProgressProxyW<Cfg0BootProgressProxySpec> {
        BootProgressProgressProxyW::new(self, 0)
    }
}
#[doc = "CFG0_BOOT_PROGRESS_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_boot_progress_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_boot_progress_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0BootProgressProxySpec;
impl crate::RegisterSpec for Cfg0BootProgressProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_boot_progress_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0BootProgressProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_boot_progress_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0BootProgressProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_BOOT_PROGRESS_PROXY to value 0"]
impl crate::Resettable for Cfg0BootProgressProxySpec {
    const RESET_VALUE: u32 = 0;
}
