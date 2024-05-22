#[doc = "Register `CFG0_DEVICE_FEATURE2` reader"]
pub type R = crate::R<Cfg0DeviceFeature2Spec>;
#[doc = "Register `CFG0_DEVICE_FEATURE2` writer"]
pub type W = crate::W<Cfg0DeviceFeature2Spec>;
#[doc = "Field `DEVICE_FEATURE2_MCAN_FD_MODE` reader - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
pub type DeviceFeature2McanFdModeR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_MCAN_FD_MODE` writer - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
pub type DeviceFeature2McanFdModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_AES_AUTH_EN` reader - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
pub type DeviceFeature2AesAuthEnR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_AES_AUTH_EN` writer - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
pub type DeviceFeature2AesAuthEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_SHA_EN` reader - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
pub type DeviceFeature2CryptoShaEnR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_SHA_EN` writer - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
pub type DeviceFeature2CryptoShaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_ENCR_EN` reader - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
pub type DeviceFeature2CryptoEncrEnR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_ENCR_EN` writer - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
pub type DeviceFeature2CryptoEncrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_PKA_EN` reader - 10:10\\]
SA2_UL Crypto Module PKA activated"]
pub type DeviceFeature2CryptoPkaEnR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE2_CRYPTO_PKA_EN` writer - 10:10\\]
SA2_UL Crypto Module PKA activated"]
pub type DeviceFeature2CryptoPkaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
    #[inline(always)]
    pub fn device_feature2_mcan_fd_mode(&self) -> DeviceFeature2McanFdModeR {
        DeviceFeature2McanFdModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
    #[inline(always)]
    pub fn device_feature2_aes_auth_en(&self) -> DeviceFeature2AesAuthEnR {
        DeviceFeature2AesAuthEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
    #[inline(always)]
    pub fn device_feature2_crypto_sha_en(&self) -> DeviceFeature2CryptoShaEnR {
        DeviceFeature2CryptoShaEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
    #[inline(always)]
    pub fn device_feature2_crypto_encr_en(&self) -> DeviceFeature2CryptoEncrEnR {
        DeviceFeature2CryptoEncrEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SA2_UL Crypto Module PKA activated"]
    #[inline(always)]
    pub fn device_feature2_crypto_pka_en(&self) -> DeviceFeature2CryptoPkaEnR {
        DeviceFeature2CryptoPkaEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
FD mode is supported on MCAN\\[1:0\\]
when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_mcan_fd_mode(
        &mut self,
    ) -> DeviceFeature2McanFdModeW<Cfg0DeviceFeature2Spec> {
        DeviceFeature2McanFdModeW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AES authentication is activated in FlashSS and DMSC when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_aes_auth_en(
        &mut self,
    ) -> DeviceFeature2AesAuthEnW<Cfg0DeviceFeature2Spec> {
        DeviceFeature2AesAuthEnW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
SA2_UL Crypto Module SHA/MD5 activated"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_crypto_sha_en(
        &mut self,
    ) -> DeviceFeature2CryptoShaEnW<Cfg0DeviceFeature2Spec> {
        DeviceFeature2CryptoShaEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SA2_UL Crypto Module AES/3DES/DBRG activated"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_crypto_encr_en(
        &mut self,
    ) -> DeviceFeature2CryptoEncrEnW<Cfg0DeviceFeature2Spec> {
        DeviceFeature2CryptoEncrEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SA2_UL Crypto Module PKA activated"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature2_crypto_pka_en(
        &mut self,
    ) -> DeviceFeature2CryptoPkaEnW<Cfg0DeviceFeature2Spec> {
        DeviceFeature2CryptoPkaEnW::new(self, 10)
    }
}
#[doc = "CFG0_DEVICE_FEATURE2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DeviceFeature2Spec;
impl crate::RegisterSpec for Cfg0DeviceFeature2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_device_feature2::R`](R) reader structure"]
impl crate::Readable for Cfg0DeviceFeature2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_device_feature2::W`](W) writer structure"]
impl crate::Writable for Cfg0DeviceFeature2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DEVICE_FEATURE2 to value 0"]
impl crate::Resettable for Cfg0DeviceFeature2Spec {
    const RESET_VALUE: u32 = 0;
}
