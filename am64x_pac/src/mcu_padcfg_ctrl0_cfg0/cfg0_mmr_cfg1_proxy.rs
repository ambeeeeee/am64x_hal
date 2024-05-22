#[doc = "Register `CFG0_MMR_CFG1_PROXY` reader"]
pub type R = crate::R<Cfg0MmrCfg1ProxySpec>;
#[doc = "Register `CFG0_MMR_CFG1_PROXY` writer"]
pub type W = crate::W<Cfg0MmrCfg1ProxySpec>;
#[doc = "Field `MMR_CFG1_PARTITIONS_PROXY` reader - 7:0\\]
Indicates present partitions"]
pub type MmrCfg1PartitionsProxyR = crate::FieldReader;
#[doc = "Field `MMR_CFG1_PARTITIONS_PROXY` writer - 7:0\\]
Indicates present partitions"]
pub type MmrCfg1PartitionsProxyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MMR_CFG1_PROXY_EN_PROXY` reader - 31:31\\]
Proxy addressing enabled"]
pub type MmrCfg1ProxyEnProxyR = crate::BitReader;
#[doc = "Field `MMR_CFG1_PROXY_EN_PROXY` writer - 31:31\\]
Proxy addressing enabled"]
pub type MmrCfg1ProxyEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates present partitions"]
    #[inline(always)]
    pub fn mmr_cfg1_partitions_proxy(&self) -> MmrCfg1PartitionsProxyR {
        MmrCfg1PartitionsProxyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Proxy addressing enabled"]
    #[inline(always)]
    pub fn mmr_cfg1_proxy_en_proxy(&self) -> MmrCfg1ProxyEnProxyR {
        MmrCfg1ProxyEnProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates present partitions"]
    #[inline(always)]
    #[must_use]
    pub fn mmr_cfg1_partitions_proxy(&mut self) -> MmrCfg1PartitionsProxyW<Cfg0MmrCfg1ProxySpec> {
        MmrCfg1PartitionsProxyW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Proxy addressing enabled"]
    #[inline(always)]
    #[must_use]
    pub fn mmr_cfg1_proxy_en_proxy(&mut self) -> MmrCfg1ProxyEnProxyW<Cfg0MmrCfg1ProxySpec> {
        MmrCfg1ProxyEnProxyW::new(self, 31)
    }
}
#[doc = "CFG0_MMR_CFG1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mmr_cfg1_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mmr_cfg1_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MmrCfg1ProxySpec;
impl crate::RegisterSpec for Cfg0MmrCfg1ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mmr_cfg1_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0MmrCfg1ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mmr_cfg1_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0MmrCfg1ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MMR_CFG1_PROXY to value 0x8000_0191"]
impl crate::Resettable for Cfg0MmrCfg1ProxySpec {
    const RESET_VALUE: u32 = 0x8000_0191;
}
