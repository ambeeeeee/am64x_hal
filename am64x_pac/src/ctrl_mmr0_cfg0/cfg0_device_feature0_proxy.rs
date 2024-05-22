#[doc = "Register `CFG0_DEVICE_FEATURE0_PROXY` reader"]
pub type R = crate::R<Cfg0DeviceFeature0ProxySpec>;
#[doc = "Register `CFG0_DEVICE_FEATURE0_PROXY` writer"]
pub type W = crate::W<Cfg0DeviceFeature0ProxySpec>;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE0_PROXY` reader - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
pub type DeviceFeature0MpuCluster0Core0ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE0_PROXY` writer - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
pub type DeviceFeature0MpuCluster0Core0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE1_PROXY` reader - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
pub type DeviceFeature0MpuCluster0Core1ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE1_PROXY` writer - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
pub type DeviceFeature0MpuCluster0Core1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE0_PROXY` reader - 16:16\\]
R5FSS0 CPU0 activated when set."]
pub type DeviceFeature0R5fss0Core0ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE0_PROXY` writer - 16:16\\]
R5FSS0 CPU0 activated when set."]
pub type DeviceFeature0R5fss0Core0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE1_PROXY` reader - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss0Core1ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE1_PROXY` writer - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss0Core1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE0_PROXY` reader - 18:18\\]
R5FSS1 CPU0 activated when set."]
pub type DeviceFeature0R5fss1Core0ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE0_PROXY` writer - 18:18\\]
R5FSS1 CPU0 activated when set."]
pub type DeviceFeature0R5fss1Core0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE1_PROXY` reader - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss1Core1ProxyR = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE1_PROXY` writer - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss1Core1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
    #[inline(always)]
    pub fn device_feature0_mpu_cluster0_core0_proxy(&self) -> DeviceFeature0MpuCluster0Core0ProxyR {
        DeviceFeature0MpuCluster0Core0ProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
    #[inline(always)]
    pub fn device_feature0_mpu_cluster0_core1_proxy(&self) -> DeviceFeature0MpuCluster0Core1ProxyR {
        DeviceFeature0MpuCluster0Core1ProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
R5FSS0 CPU0 activated when set."]
    #[inline(always)]
    pub fn device_feature0_r5fss0_core0_proxy(&self) -> DeviceFeature0R5fss0Core0ProxyR {
        DeviceFeature0R5fss0Core0ProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    pub fn device_feature0_r5fss0_core1_proxy(&self) -> DeviceFeature0R5fss0Core1ProxyR {
        DeviceFeature0R5fss0Core1ProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
R5FSS1 CPU0 activated when set."]
    #[inline(always)]
    pub fn device_feature0_r5fss1_core0_proxy(&self) -> DeviceFeature0R5fss1Core0ProxyR {
        DeviceFeature0R5fss1Core0ProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    pub fn device_feature0_r5fss1_core1_proxy(&self) -> DeviceFeature0R5fss1Core1ProxyR {
        DeviceFeature0R5fss1Core1ProxyR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_mpu_cluster0_core0_proxy(
        &mut self,
    ) -> DeviceFeature0MpuCluster0Core0ProxyW<Cfg0DeviceFeature0ProxySpec> {
        DeviceFeature0MpuCluster0Core0ProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_mpu_cluster0_core1_proxy(
        &mut self,
    ) -> DeviceFeature0MpuCluster0Core1ProxyW<Cfg0DeviceFeature0ProxySpec> {
        DeviceFeature0MpuCluster0Core1ProxyW::new(self, 1)
    }
    #[doc = "Bit 16 - 16:16\\]
R5FSS0 CPU0 activated when set."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss0_core0_proxy(
        &mut self,
    ) -> DeviceFeature0R5fss0Core0ProxyW<Cfg0DeviceFeature0ProxySpec> {
        DeviceFeature0R5fss0Core0ProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss0_core1_proxy(
        &mut self,
    ) -> DeviceFeature0R5fss0Core1ProxyW<Cfg0DeviceFeature0ProxySpec> {
        DeviceFeature0R5fss0Core1ProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
R5FSS1 CPU0 activated when set."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss1_core0_proxy(
        &mut self,
    ) -> DeviceFeature0R5fss1Core0ProxyW<Cfg0DeviceFeature0ProxySpec> {
        DeviceFeature0R5fss1Core0ProxyW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss1_core1_proxy(
        &mut self,
    ) -> DeviceFeature0R5fss1Core1ProxyW<Cfg0DeviceFeature0ProxySpec> {
        DeviceFeature0R5fss1Core1ProxyW::new(self, 19)
    }
}
#[doc = "CFG0_DEVICE_FEATURE0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature0_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature0_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DeviceFeature0ProxySpec;
impl crate::RegisterSpec for Cfg0DeviceFeature0ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_device_feature0_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0DeviceFeature0ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_device_feature0_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0DeviceFeature0ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DEVICE_FEATURE0_PROXY to value 0"]
impl crate::Resettable for Cfg0DeviceFeature0ProxySpec {
    const RESET_VALUE: u32 = 0;
}
