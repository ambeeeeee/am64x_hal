#[doc = "Register `CFG0_TEMP_DIODE_TRIM_PROXY` reader"]
pub type R = crate::R<Cfg0TempDiodeTrimProxySpec>;
#[doc = "Register `CFG0_TEMP_DIODE_TRIM_PROXY` writer"]
pub type W = crate::W<Cfg0TempDiodeTrimProxySpec>;
#[doc = "Field `TEMP_DIODE_TRIM_TRIM_PROXY` reader - 13:0\\]
Sets the diode non-ideality factor (n), starting from 100th place decimal and going down"]
pub type TempDiodeTrimTrimProxyR = crate::FieldReader<u16>;
#[doc = "Field `TEMP_DIODE_TRIM_TRIM_PROXY` writer - 13:0\\]
Sets the diode non-ideality factor (n), starting from 100th place decimal and going down"]
pub type TempDiodeTrimTrimProxyW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Sets the diode non-ideality factor (n), starting from 100th place decimal and going down"]
    #[inline(always)]
    pub fn temp_diode_trim_trim_proxy(&self) -> TempDiodeTrimTrimProxyR {
        TempDiodeTrimTrimProxyR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Sets the diode non-ideality factor (n), starting from 100th place decimal and going down"]
    #[inline(always)]
    #[must_use]
    pub fn temp_diode_trim_trim_proxy(
        &mut self,
    ) -> TempDiodeTrimTrimProxyW<Cfg0TempDiodeTrimProxySpec> {
        TempDiodeTrimTrimProxyW::new(self, 0)
    }
}
#[doc = "CFG0_TEMP_DIODE_TRIM_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_temp_diode_trim_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_temp_diode_trim_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0TempDiodeTrimProxySpec;
impl crate::RegisterSpec for Cfg0TempDiodeTrimProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_temp_diode_trim_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0TempDiodeTrimProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_temp_diode_trim_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0TempDiodeTrimProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_TEMP_DIODE_TRIM_PROXY to value 0"]
impl crate::Resettable for Cfg0TempDiodeTrimProxySpec {
    const RESET_VALUE: u32 = 0;
}
