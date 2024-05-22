#[doc = "Register `CFG0_BOOT_PROGRESS` reader"]
pub type R = crate::R<Cfg0BootProgressSpec>;
#[doc = "Register `CFG0_BOOT_PROGRESS` writer"]
pub type W = crate::W<Cfg0BootProgressSpec>;
#[doc = "Field `BOOT_PROGRESS_PROGRESS` reader - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
pub type BootProgressProgressR = crate::FieldReader<u32>;
#[doc = "Field `BOOT_PROGRESS_PROGRESS` writer - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
pub type BootProgressProgressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
    #[inline(always)]
    pub fn boot_progress_progress(&self) -> BootProgressProgressR {
        BootProgressProgressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Written by ROM to indicate boot progression. Values and their meaning are determined by the ROM."]
    #[inline(always)]
    #[must_use]
    pub fn boot_progress_progress(&mut self) -> BootProgressProgressW<Cfg0BootProgressSpec> {
        BootProgressProgressW::new(self, 0)
    }
}
#[doc = "CFG0_BOOT_PROGRESS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_boot_progress::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_boot_progress::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0BootProgressSpec;
impl crate::RegisterSpec for Cfg0BootProgressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_boot_progress::R`](R) reader structure"]
impl crate::Readable for Cfg0BootProgressSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_boot_progress::W`](W) writer structure"]
impl crate::Writable for Cfg0BootProgressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_BOOT_PROGRESS to value 0"]
impl crate::Resettable for Cfg0BootProgressSpec {
    const RESET_VALUE: u32 = 0;
}
