#[doc = "Register `CFG0_MMR_CFG1` reader"]
pub type R = crate::R<Cfg0MmrCfg1Spec>;
#[doc = "Register `CFG0_MMR_CFG1` writer"]
pub type W = crate::W<Cfg0MmrCfg1Spec>;
#[doc = "Field `MMR_CFG1_PARTITIONS` reader - 7:0\\]
Indicates present partitions"]
pub type MmrCfg1PartitionsR = crate::FieldReader;
#[doc = "Field `MMR_CFG1_PARTITIONS` writer - 7:0\\]
Indicates present partitions"]
pub type MmrCfg1PartitionsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MMR_CFG1_PROXY_EN` reader - 31:31\\]
Proxy addressing enabled"]
pub type MmrCfg1ProxyEnR = crate::BitReader;
#[doc = "Field `MMR_CFG1_PROXY_EN` writer - 31:31\\]
Proxy addressing enabled"]
pub type MmrCfg1ProxyEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates present partitions"]
    #[inline(always)]
    pub fn mmr_cfg1_partitions(&self) -> MmrCfg1PartitionsR {
        MmrCfg1PartitionsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Proxy addressing enabled"]
    #[inline(always)]
    pub fn mmr_cfg1_proxy_en(&self) -> MmrCfg1ProxyEnR {
        MmrCfg1ProxyEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Indicates present partitions"]
    #[inline(always)]
    #[must_use]
    pub fn mmr_cfg1_partitions(&mut self) -> MmrCfg1PartitionsW<Cfg0MmrCfg1Spec> {
        MmrCfg1PartitionsW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Proxy addressing enabled"]
    #[inline(always)]
    #[must_use]
    pub fn mmr_cfg1_proxy_en(&mut self) -> MmrCfg1ProxyEnW<Cfg0MmrCfg1Spec> {
        MmrCfg1ProxyEnW::new(self, 31)
    }
}
#[doc = "CFG0_MMR_CFG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mmr_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mmr_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MmrCfg1Spec;
impl crate::RegisterSpec for Cfg0MmrCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mmr_cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg0MmrCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mmr_cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg0MmrCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MMR_CFG1 to value 0x8000_0191"]
impl crate::Resettable for Cfg0MmrCfg1Spec {
    const RESET_VALUE: u32 = 0x8000_0191;
}
