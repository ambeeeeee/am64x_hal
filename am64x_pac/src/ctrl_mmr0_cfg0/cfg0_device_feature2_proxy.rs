#[doc = "Register `CFG0_DEVICE_FEATURE2_PROXY` reader"]
pub type R = crate::R<Cfg0DeviceFeature2ProxySpec>;
#[doc = "Register `CFG0_DEVICE_FEATURE2_PROXY` writer"]
pub type W = crate::W<Cfg0DeviceFeature2ProxySpec>;
#[doc = "Field `DEVICE_FEATURE2_MCAN_FD_MODE_PROXY` reader - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
pub type DeviceFeature2McanFdModeProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_MCAN_FD_MODE_PROXY` writer - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
pub type DeviceFeature2McanFdModeProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_AES_AUTH_EN_PROXY` reader - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
pub type DeviceFeature2AesAuthEnProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_AES_AUTH_EN_PROXY` writer - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
pub type DeviceFeature2AesAuthEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_SHA_EN_PROXY` reader - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
pub type DeviceFeature2CryptoShaEnProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_SHA_EN_PROXY` writer - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
pub type DeviceFeature2CryptoShaEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_ENCR_EN_PROXY` reader - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
pub type DeviceFeature2CryptoEncrEnProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_ENCR_EN_PROXY` writer - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
pub type DeviceFeature2CryptoEncrEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_PKA_EN_PROXY` reader - 10:10\\]
SA2_UL Crypto Module PKA activated"]
pub type DeviceFeature2CryptoPkaEnProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_PKA_EN_PROXY` writer - 10:10\\]
SA2_UL Crypto Module PKA activated"]
pub type DeviceFeature2CryptoPkaEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
    #[inline(always)]
    pub fn device_feature2_mcan_fd_mode_proxy(&self) -> DeviceFeature2McanFdModeProxyR {
        DeviceFeature2McanFdModeProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
    #[inline(always)]
    pub fn device_feature2_aes_auth_en_proxy(&self) -> DeviceFeature2AesAuthEnProxyR {
        DeviceFeature2AesAuthEnProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
    #[inline(always)]
    pub fn device_feature2_crypto_sha_en_proxy(&self) -> DeviceFeature2CryptoShaEnProxyR {
        DeviceFeature2CryptoShaEnProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
    #[inline(always)]
    pub fn device_feature2_crypto_encr_en_proxy(&self) -> DeviceFeature2CryptoEncrEnProxyR {
        DeviceFeature2CryptoEncrEnProxyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SA2_UL Crypto Module PKA activated"]
    #[inline(always)]
    pub fn device_feature2_crypto_pka_en_proxy(&self) -> DeviceFeature2CryptoPkaEnProxyR {
        DeviceFeature2CryptoPkaEnProxyR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_mcan_fd_mode_proxy(
        &mut self,
    ) -> DeviceFeature2McanFdModeProxyW<Cfg0DeviceFeature2ProxySpec> {
        DeviceFeature2McanFdModeProxyW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_aes_auth_en_proxy(
        &mut self,
    ) -> DeviceFeature2AesAuthEnProxyW<Cfg0DeviceFeature2ProxySpec> {
        DeviceFeature2AesAuthEnProxyW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_crypto_sha_en_proxy(
        &mut self,
    ) -> DeviceFeature2CryptoShaEnProxyW<Cfg0DeviceFeature2ProxySpec> {
        DeviceFeature2CryptoShaEnProxyW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_crypto_encr_en_proxy(
        &mut self,
    ) -> DeviceFeature2CryptoEncrEnProxyW<Cfg0DeviceFeature2ProxySpec> {
        DeviceFeature2CryptoEncrEnProxyW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SA2_UL Crypto Module PKA activated"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_crypto_pka_en_proxy(
        &mut self,
    ) -> DeviceFeature2CryptoPkaEnProxyW<Cfg0DeviceFeature2ProxySpec> {
        DeviceFeature2CryptoPkaEnProxyW::new(self, 10)
    }
}
#[doc = "CFG0_DEVICE_FEATURE2_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature2_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature2_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DeviceFeature2ProxySpec;
impl crate::RegisterSpec for Cfg0DeviceFeature2ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_device_feature2_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0DeviceFeature2ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_device_feature2_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0DeviceFeature2ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DEVICE_FEATURE2_PROXY to value 0"]
impl crate::Resettable for Cfg0DeviceFeature2ProxySpec {
    const RESET_VALUE: u32 = 0;
}
