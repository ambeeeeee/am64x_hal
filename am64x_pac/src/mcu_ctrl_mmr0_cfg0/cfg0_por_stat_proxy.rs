#[doc = "Register `CFG0_POR_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0PorStatProxySpec>;
#[doc = "Register `CFG0_POR_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0PorStatProxySpec>;
#[doc = "Field `POR_STAT_SOC_POR_PROXY` reader - 4:4\\]
POR module status"]
pub type PorStatSocPorProxyR = crate::BitReader;
#[doc = "Field `POR_STAT_SOC_POR_PROXY` writer - 4:4\\]
POR module status"]
pub type PorStatSocPorProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_STAT_BGOK_PROXY` reader - 8:8\\]
Bandgap OK status"]
pub type PorStatBgokProxyR = crate::BitReader;
#[doc = "Field `POR_STAT_BGOK_PROXY` writer - 8:8\\]
Bandgap OK status"]
pub type PorStatBgokProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
POR module status"]
    #[inline(always)]
    pub fn por_stat_soc_por_proxy(&self) -> PorStatSocPorProxyR {
        PorStatSocPorProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Bandgap OK status"]
    #[inline(always)]
    pub fn por_stat_bgok_proxy(&self) -> PorStatBgokProxyR {
        PorStatBgokProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
POR module status"]
    #[inline(always)]
    #[must_use]
    pub fn por_stat_soc_por_proxy(&mut self) -> PorStatSocPorProxyW<Cfg0PorStatProxySpec> {
        PorStatSocPorProxyW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Bandgap OK status"]
    #[inline(always)]
    #[must_use]
    pub fn por_stat_bgok_proxy(&mut self) -> PorStatBgokProxyW<Cfg0PorStatProxySpec> {
        PorStatBgokProxyW::new(self, 8)
    }
}
#[doc = "CFG0_POR_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PorStatProxySpec;
impl crate::RegisterSpec for Cfg0PorStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_por_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PorStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_por_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PorStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POR_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0PorStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
