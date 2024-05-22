#[doc = "Register `CFG0_DEVICE_FEATURE6` reader"]
pub type R = crate::R<Cfg0DeviceFeature6Spec>;
#[doc = "Register `CFG0_DEVICE_FEATURE6` writer"]
pub type W = crate::W<Cfg0DeviceFeature6Spec>;
#[doc = "Field `DEVICE_FEATURE6_SA2_UL` reader - 5:5\\]
MAIN domain security accelerator is activated when set"]
pub type DeviceFeature6Sa2UlR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE6_SA2_UL` writer - 5:5\\]
MAIN domain security accelerator is activated when set"]
pub type DeviceFeature6Sa2UlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE6_SPARE0` reader - 16:16\\]
Spare0 LPSC is activated when set"]
pub type DeviceFeature6Spare0R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE6_SPARE0` writer - 16:16\\]
Spare0 LPSC is activated when set"]
pub type DeviceFeature6Spare0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE6_SPARE1` reader - 17:17\\]
Spare1 LPSC is activated when set"]
pub type DeviceFeature6Spare1R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE6_SPARE1` writer - 17:17\\]
Spare1 LPSC is activated when set"]
pub type DeviceFeature6Spare1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - 5:5\\]
MAIN domain security accelerator is activated when set"]
    #[inline(always)]
    pub fn device_feature6_sa2_ul(&self) -> DeviceFeature6Sa2UlR {
        DeviceFeature6Sa2UlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Spare0 LPSC is activated when set"]
    #[inline(always)]
    pub fn device_feature6_spare0(&self) -> DeviceFeature6Spare0R {
        DeviceFeature6Spare0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Spare1 LPSC is activated when set"]
    #[inline(always)]
    pub fn device_feature6_spare1(&self) -> DeviceFeature6Spare1R {
        DeviceFeature6Spare1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - 5:5\\]
MAIN domain security accelerator is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature6_sa2_ul(&mut self) -> DeviceFeature6Sa2UlW<Cfg0DeviceFeature6Spec> {
        DeviceFeature6Sa2UlW::new(self, 5)
    }
    #[doc = "Bit 16 - 16:16\\]
Spare0 LPSC is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature6_spare0(&mut self) -> DeviceFeature6Spare0W<Cfg0DeviceFeature6Spec> {
        DeviceFeature6Spare0W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Spare1 LPSC is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature6_spare1(&mut self) -> DeviceFeature6Spare1W<Cfg0DeviceFeature6Spec> {
        DeviceFeature6Spare1W::new(self, 17)
    }
}
#[doc = "CFG0_DEVICE_FEATURE6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DeviceFeature6Spec;
impl crate::RegisterSpec for Cfg0DeviceFeature6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_device_feature6::R`](R) reader structure"]
impl crate::Readable for Cfg0DeviceFeature6Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_device_feature6::W`](W) writer structure"]
impl crate::Writable for Cfg0DeviceFeature6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DEVICE_FEATURE6 to value 0"]
impl crate::Resettable for Cfg0DeviceFeature6Spec {
    const RESET_VALUE: u32 = 0;
}
