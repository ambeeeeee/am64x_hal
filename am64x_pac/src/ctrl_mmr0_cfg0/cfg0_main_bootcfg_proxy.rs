#[doc = "Register `CFG0_MAIN_BOOTCFG_PROXY` reader"]
pub type R = crate::R<Cfg0MainBootcfgProxySpec>;
#[doc = "Register `CFG0_MAIN_BOOTCFG_PROXY` writer"]
pub type W = crate::W<Cfg0MainBootcfgProxySpec>;
#[doc = "Field `MAIN_BOOTCFG_BOOTMODE_PROXY` reader - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
pub type MainBootcfgBootmodeProxyR = crate::FieldReader<u16>;
#[doc = "Field `MAIN_BOOTCFG_BOOTMODE_PROXY` writer - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
pub type MainBootcfgBootmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
    #[inline(always)]
    pub fn main_bootcfg_bootmode_proxy(&self) -> MainBootcfgBootmodeProxyR {
        MainBootcfgBootmodeProxyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Specifies the device Primary and Backup boot media as latched at PORz"]
    #[inline(always)]
    #[must_use]
    pub fn main_bootcfg_bootmode_proxy(
        &mut self,
    ) -> MainBootcfgBootmodeProxyW<Cfg0MainBootcfgProxySpec> {
        MainBootcfgBootmodeProxyW::new(self, 0)
    }
}
#[doc = "CFG0_MAIN_BOOTCFG_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_bootcfg_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_bootcfg_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainBootcfgProxySpec;
impl crate::RegisterSpec for Cfg0MainBootcfgProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_bootcfg_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0MainBootcfgProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_bootcfg_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0MainBootcfgProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_BOOTCFG_PROXY to value 0"]
impl crate::Resettable for Cfg0MainBootcfgProxySpec {
    const RESET_VALUE: u32 = 0;
}
