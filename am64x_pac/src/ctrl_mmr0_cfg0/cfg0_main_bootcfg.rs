#[doc = "Register `CFG0_MAIN_BOOTCFG` reader"]
pub type R = crate::R<Cfg0MainBootcfgSpec>;
#[doc = "Register `CFG0_MAIN_BOOTCFG` writer"]
pub type W = crate::W<Cfg0MainBootcfgSpec>;
#[doc = "Field `MAIN_BOOTCFG_BOOTMODE` reader - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
pub type MainBootcfgBootmodeR = crate::FieldReader<u16>;
#[doc = "Field `MAIN_BOOTCFG_BOOTMODE` writer - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
pub type MainBootcfgBootmodeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
    #[inline(always)]
    pub fn main_bootcfg_bootmode(&self) -> MainBootcfgBootmodeR {
        MainBootcfgBootmodeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
    #[inline(always)]
    #[must_use]
    pub fn main_bootcfg_bootmode(&mut self) -> MainBootcfgBootmodeW<Cfg0MainBootcfgSpec> {
        MainBootcfgBootmodeW::new(self, 0)
    }
}
#[doc = "CFG0_MAIN_BOOTCFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_bootcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_bootcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainBootcfgSpec;
impl crate::RegisterSpec for Cfg0MainBootcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_bootcfg::R`](R) reader structure"]
impl crate::Readable for Cfg0MainBootcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_bootcfg::W`](W) writer structure"]
impl crate::Writable for Cfg0MainBootcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_BOOTCFG to value 0"]
impl crate::Resettable for Cfg0MainBootcfgSpec {
    const RESET_VALUE: u32 = 0;
}
