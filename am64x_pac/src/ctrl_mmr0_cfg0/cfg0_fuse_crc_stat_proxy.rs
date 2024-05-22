#[doc = "Register `CFG0_FUSE_CRC_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0FuseCrcStatProxySpec>;
#[doc = "Register `CFG0_FUSE_CRC_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0FuseCrcStatProxySpec>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_1_PROXY` reader - 1:1\\]
Indicates eFuse CRC error on chain 1"]
pub type FuseCrcStatCrcErr1ProxyR = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_1_PROXY` writer - 1:1\\]
Indicates eFuse CRC error on chain 1"]
pub type FuseCrcStatCrcErr1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_2_PROXY` reader - 2:2\\]
Indicates eFuse CRC error on chain 2"]
pub type FuseCrcStatCrcErr2ProxyR = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_2_PROXY` writer - 2:2\\]
Indicates eFuse CRC error on chain 2"]
pub type FuseCrcStatCrcErr2ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_3_PROXY` reader - 3:3\\]
Indicates eFuse CRC error on chain 3"]
pub type FuseCrcStatCrcErr3ProxyR = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_3_PROXY` writer - 3:3\\]
Indicates eFuse CRC error on chain 3"]
pub type FuseCrcStatCrcErr3ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_4_PROXY` reader - 4:4\\]
Indicates eFuse CRC error on chain 4"]
pub type FuseCrcStatCrcErr4ProxyR = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_4_PROXY` writer - 4:4\\]
Indicates eFuse CRC error on chain 4"]
pub type FuseCrcStatCrcErr4ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_5_PROXY` reader - 5:5\\]
Indicates eFuse CRC error on chain 5"]
pub type FuseCrcStatCrcErr5ProxyR = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_5_PROXY` writer - 5:5\\]
Indicates eFuse CRC error on chain 5"]
pub type FuseCrcStatCrcErr5ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_6_PROXY` reader - 6:6\\]
Indicates eFuse CRC error on chain 6"]
pub type FuseCrcStatCrcErr6ProxyR = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_6_PROXY` writer - 6:6\\]
Indicates eFuse CRC error on chain 6"]
pub type FuseCrcStatCrcErr6ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_7_PROXY` reader - 7:7\\]
Indicates eFuse CRC error on chain 7"]
pub type FuseCrcStatCrcErr7ProxyR = crate::BitReader;
#[doc = "Field `FUSE_CRC_STAT_CRC_ERR_7_PROXY` writer - 7:7\\]
Indicates eFuse CRC error on chain 7"]
pub type FuseCrcStatCrcErr7ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Indicates eFuse CRC error on chain 1"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_1_proxy(&self) -> FuseCrcStatCrcErr1ProxyR {
        FuseCrcStatCrcErr1ProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates eFuse CRC error on chain 2"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_2_proxy(&self) -> FuseCrcStatCrcErr2ProxyR {
        FuseCrcStatCrcErr2ProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates eFuse CRC error on chain 3"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_3_proxy(&self) -> FuseCrcStatCrcErr3ProxyR {
        FuseCrcStatCrcErr3ProxyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates eFuse CRC error on chain 4"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_4_proxy(&self) -> FuseCrcStatCrcErr4ProxyR {
        FuseCrcStatCrcErr4ProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates eFuse CRC error on chain 5"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_5_proxy(&self) -> FuseCrcStatCrcErr5ProxyR {
        FuseCrcStatCrcErr5ProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates eFuse CRC error on chain 6"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_6_proxy(&self) -> FuseCrcStatCrcErr6ProxyR {
        FuseCrcStatCrcErr6ProxyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates eFuse CRC error on chain 7"]
    #[inline(always)]
    pub fn fuse_crc_stat_crc_err_7_proxy(&self) -> FuseCrcStatCrcErr7ProxyR {
        FuseCrcStatCrcErr7ProxyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Indicates eFuse CRC error on chain 1"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_1_proxy(
        &mut self,
    ) -> FuseCrcStatCrcErr1ProxyW<Cfg0FuseCrcStatProxySpec> {
        FuseCrcStatCrcErr1ProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates eFuse CRC error on chain 2"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_2_proxy(
        &mut self,
    ) -> FuseCrcStatCrcErr2ProxyW<Cfg0FuseCrcStatProxySpec> {
        FuseCrcStatCrcErr2ProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates eFuse CRC error on chain 3"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_3_proxy(
        &mut self,
    ) -> FuseCrcStatCrcErr3ProxyW<Cfg0FuseCrcStatProxySpec> {
        FuseCrcStatCrcErr3ProxyW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates eFuse CRC error on chain 4"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_4_proxy(
        &mut self,
    ) -> FuseCrcStatCrcErr4ProxyW<Cfg0FuseCrcStatProxySpec> {
        FuseCrcStatCrcErr4ProxyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates eFuse CRC error on chain 5"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_5_proxy(
        &mut self,
    ) -> FuseCrcStatCrcErr5ProxyW<Cfg0FuseCrcStatProxySpec> {
        FuseCrcStatCrcErr5ProxyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates eFuse CRC error on chain 6"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_6_proxy(
        &mut self,
    ) -> FuseCrcStatCrcErr6ProxyW<Cfg0FuseCrcStatProxySpec> {
        FuseCrcStatCrcErr6ProxyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Indicates eFuse CRC error on chain 7"]
    #[inline(always)]
    #[must_use]
    pub fn fuse_crc_stat_crc_err_7_proxy(
        &mut self,
    ) -> FuseCrcStatCrcErr7ProxyW<Cfg0FuseCrcStatProxySpec> {
        FuseCrcStatCrcErr7ProxyW::new(self, 7)
    }
}
#[doc = "CFG0_FUSE_CRC_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fuse_crc_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fuse_crc_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FuseCrcStatProxySpec;
impl crate::RegisterSpec for Cfg0FuseCrcStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fuse_crc_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0FuseCrcStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fuse_crc_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0FuseCrcStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_FUSE_CRC_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0FuseCrcStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
