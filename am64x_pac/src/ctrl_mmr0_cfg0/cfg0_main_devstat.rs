#[doc = "Register `CFG0_MAIN_DEVSTAT` reader"]
pub type R = crate::R<Cfg0MainDevstatSpec>;
#[doc = "Register `CFG0_MAIN_DEVSTAT` writer"]
pub type W = crate::W<Cfg0MainDevstatSpec>;
#[doc = "Field `MAIN_DEVSTAT_BOOTMODE` reader - 15:0\\]
Specifies the device Primary and Backup boot media."]
pub type MainDevstatBootmodeR = crate::FieldReader<u16>;
#[doc = "Field `MAIN_DEVSTAT_BOOTMODE` writer - 15:0\\]
Specifies the device Primary and Backup boot media."]
pub type MainDevstatBootmodeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the device Primary and Backup boot media."]
    #[inline(always)]
    pub fn main_devstat_bootmode(&self) -> MainDevstatBootmodeR {
        MainDevstatBootmodeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the device Primary and Backup boot media."]
    #[inline(always)]
    #[must_use]
    pub fn main_devstat_bootmode(&mut self) -> MainDevstatBootmodeW<Cfg0MainDevstatSpec> {
        MainDevstatBootmodeW::new(self, 0)
    }
}
#[doc = "CFG0_MAIN_DEVSTAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_devstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_devstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainDevstatSpec;
impl crate::RegisterSpec for Cfg0MainDevstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_devstat::R`](R) reader structure"]
impl crate::Readable for Cfg0MainDevstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_devstat::W`](W) writer structure"]
impl crate::Writable for Cfg0MainDevstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_DEVSTAT to value 0"]
impl crate::Resettable for Cfg0MainDevstatSpec {
    const RESET_VALUE: u32 = 0;
}
