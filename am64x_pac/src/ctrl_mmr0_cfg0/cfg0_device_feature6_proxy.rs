#[doc = "Register `CFG0_DEVICE_FEATURE6_PROXY` reader"]
pub type R = crate::R<Cfg0DeviceFeature6ProxySpec>;
#[doc = "Register `CFG0_DEVICE_FEATURE6_PROXY` writer"]
pub type W = crate::W<Cfg0DeviceFeature6ProxySpec>;
#[doc = "Field `DEVICE_FEATURE6_SA2_UL_PROXY` reader - 5:5\\]
MAIN domain security accelerator is activated when set"]
pub type DeviceFeature6Sa2UlProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE6_SA2_UL_PROXY` writer - 5:5\\]
MAIN domain security accelerator is activated when set"]
pub type DeviceFeature6Sa2UlProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE6_SPARE0_PROXY` reader - 16:16\\]
Spare0 LPSC is activated when set"]
pub type DeviceFeature6Spare0ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE6_SPARE0_PROXY` writer - 16:16\\]
Spare0 LPSC is activated when set"]
pub type DeviceFeature6Spare0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE6_SPARE1_PROXY` reader - 17:17\\]
Spare1 LPSC is activated when set"]
pub type DeviceFeature6Spare1ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE6_SPARE1_PROXY` writer - 17:17\\]
Spare1 LPSC is activated when set"]
pub type DeviceFeature6Spare1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - 5:5\\]
MAIN domain security accelerator is activated when set"]
    #[inline(always)]
    pub fn device_feature6_sa2_ul_proxy(&self) -> DeviceFeature6Sa2UlProxyR {
        DeviceFeature6Sa2UlProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Spare0 LPSC is activated when set"]
    #[inline(always)]
    pub fn device_feature6_spare0_proxy(&self) -> DeviceFeature6Spare0ProxyR {
        DeviceFeature6Spare0ProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Spare1 LPSC is activated when set"]
    #[inline(always)]
    pub fn device_feature6_spare1_proxy(&self) -> DeviceFeature6Spare1ProxyR {
        DeviceFeature6Spare1ProxyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - 5:5\\]
MAIN domain security accelerator is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature6_sa2_ul_proxy(
        &mut self,
    ) -> DeviceFeature6Sa2UlProxyW<Cfg0DeviceFeature6ProxySpec> {
        DeviceFeature6Sa2UlProxyW::new(self, 5)
    }
    #[doc = "Bit 16 - 16:16\\]
Spare0 LPSC is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature6_spare0_proxy(
        &mut self,
    ) -> DeviceFeature6Spare0ProxyW<Cfg0DeviceFeature6ProxySpec> {
        DeviceFeature6Spare0ProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Spare1 LPSC is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature6_spare1_proxy(
        &mut self,
    ) -> DeviceFeature6Spare1ProxyW<Cfg0DeviceFeature6ProxySpec> {
        DeviceFeature6Spare1ProxyW::new(self, 17)
    }
}
#[doc = "CFG0_DEVICE_FEATURE6_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature6_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature6_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DeviceFeature6ProxySpec;
impl crate::RegisterSpec for Cfg0DeviceFeature6ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_device_feature6_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0DeviceFeature6ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_device_feature6_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0DeviceFeature6ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DEVICE_FEATURE6_PROXY to value 0"]
impl crate::Resettable for Cfg0DeviceFeature6ProxySpec {
    const RESET_VALUE: u32 = 0;
}
