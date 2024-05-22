#[doc = "Register `CFG0_POR_STAT` reader"]
pub type R = crate::R<Cfg0PorStatSpec>;
#[doc = "Register `CFG0_POR_STAT` writer"]
pub type W = crate::W<Cfg0PorStatSpec>;
#[doc = "Field `POR_STAT_SOC_POR` reader - 4:4\\]
POR module status"]
pub type PorStatSocPorR = crate::BitReader;
#[doc = "Field `POR_STAT_SOC_POR` writer - 4:4\\]
POR module status"]
pub type PorStatSocPorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_STAT_BGOK` reader - 8:8\\]
Bandgap OK status"]
pub type PorStatBgokR = crate::BitReader;
#[doc = "Field `POR_STAT_BGOK` writer - 8:8\\]
Bandgap OK status"]
pub type PorStatBgokW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
POR module status"]
    #[inline(always)]
    pub fn por_stat_soc_por(&self) -> PorStatSocPorR {
        PorStatSocPorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Bandgap OK status"]
    #[inline(always)]
    pub fn por_stat_bgok(&self) -> PorStatBgokR {
        PorStatBgokR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
POR module status"]
    #[inline(always)]
    #[must_use]
    pub fn por_stat_soc_por(&mut self) -> PorStatSocPorW<Cfg0PorStatSpec> {
        PorStatSocPorW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Bandgap OK status"]
    #[inline(always)]
    #[must_use]
    pub fn por_stat_bgok(&mut self) -> PorStatBgokW<Cfg0PorStatSpec> {
        PorStatBgokW::new(self, 8)
    }
}
#[doc = "CFG0_POR_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PorStatSpec;
impl crate::RegisterSpec for Cfg0PorStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_por_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0PorStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_por_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0PorStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POR_STAT to value 0"]
impl crate::Resettable for Cfg0PorStatSpec {
    const RESET_VALUE: u32 = 0;
}
