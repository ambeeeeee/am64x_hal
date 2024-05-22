#[doc = "Register `CFG0_DEVICE_FEATURE0` reader"]
pub type R = crate::R<Cfg0DeviceFeature0Spec>;
#[doc = "Register `CFG0_DEVICE_FEATURE0` writer"]
pub type W = crate::W<Cfg0DeviceFeature0Spec>;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE0` reader - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
pub type DeviceFeature0MpuCluster0Core0R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE0` writer - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
pub type DeviceFeature0MpuCluster0Core0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE1` reader - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
pub type DeviceFeature0MpuCluster0Core1R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_MPU_CLUSTER0_CORE1` writer - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
pub type DeviceFeature0MpuCluster0Core1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE0` reader - 16:16\\]
R5FSS0 CPU0 activated when set."]
pub type DeviceFeature0R5fss0Core0R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE0` writer - 16:16\\]
R5FSS0 CPU0 activated when set."]
pub type DeviceFeature0R5fss0Core0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE1` reader - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss0Core1R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS0_CORE1` writer - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss0Core1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE0` reader - 18:18\\]
R5FSS1 CPU0 activated when set."]
pub type DeviceFeature0R5fss1Core0R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE0` writer - 18:18\\]
R5FSS1 CPU0 activated when set."]
pub type DeviceFeature0R5fss1Core0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE1` reader - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss1Core1R = crate::BitReader;
#[doc = "Field `DEVICE_FEATURE0_R5FSS1_CORE1` writer - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
pub type DeviceFeature0R5fss1Core1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
    #[inline(always)]
    pub fn device_feature0_mpu_cluster0_core0(&self) -> DeviceFeature0MpuCluster0Core0R {
        DeviceFeature0MpuCluster0Core0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
    #[inline(always)]
    pub fn device_feature0_mpu_cluster0_core1(&self) -> DeviceFeature0MpuCluster0Core1R {
        DeviceFeature0MpuCluster0Core1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
R5FSS0 CPU0 activated when set."]
    #[inline(always)]
    pub fn device_feature0_r5fss0_core0(&self) -> DeviceFeature0R5fss0Core0R {
        DeviceFeature0R5fss0Core0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    pub fn device_feature0_r5fss0_core1(&self) -> DeviceFeature0R5fss0Core1R {
        DeviceFeature0R5fss0Core1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
R5FSS1 CPU0 activated when set."]
    #[inline(always)]
    pub fn device_feature0_r5fss1_core0(&self) -> DeviceFeature0R5fss1Core0R {
        DeviceFeature0R5fss1Core0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    pub fn device_feature0_r5fss1_core1(&self) -> DeviceFeature0R5fss1Core1R {
        DeviceFeature0R5fss1Core1R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MPU Cluster0 Core 0 is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_mpu_cluster0_core0(
        &mut self,
    ) -> DeviceFeature0MpuCluster0Core0W<Cfg0DeviceFeature0Spec> {
        DeviceFeature0MpuCluster0Core0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MPU Cluster0 Core 1 is activated when set"]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_mpu_cluster0_core1(
        &mut self,
    ) -> DeviceFeature0MpuCluster0Core1W<Cfg0DeviceFeature0Spec> {
        DeviceFeature0MpuCluster0Core1W::new(self, 1)
    }
    #[doc = "Bit 16 - 16:16\\]
R5FSS0 CPU0 activated when set."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss0_core0(
        &mut self,
    ) -> DeviceFeature0R5fss0Core0W<Cfg0DeviceFeature0Spec> {
        DeviceFeature0R5fss0Core0W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
R5FSS0 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss0_core1(
        &mut self,
    ) -> DeviceFeature0R5fss0Core1W<Cfg0DeviceFeature0Spec> {
        DeviceFeature0R5fss0Core1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
R5FSS1 CPU0 activated when set."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss1_core0(
        &mut self,
    ) -> DeviceFeature0R5fss1Core0W<Cfg0DeviceFeature0Spec> {
        DeviceFeature0R5fss1Core0W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
R5FSS1 CPU1 activated when set. CPU0 must also be activated."]
    #[inline(always)]
    #[must_use]
    pub fn device_feature0_r5fss1_core1(
        &mut self,
    ) -> DeviceFeature0R5fss1Core1W<Cfg0DeviceFeature0Spec> {
        DeviceFeature0R5fss1Core1W::new(self, 19)
    }
}
#[doc = "CFG0_DEVICE_FEATURE0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_device_feature0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_device_feature0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DeviceFeature0Spec;
impl crate::RegisterSpec for Cfg0DeviceFeature0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_device_feature0::R`](R) reader structure"]
impl crate::Readable for Cfg0DeviceFeature0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_device_feature0::W`](W) writer structure"]
impl crate::Writable for Cfg0DeviceFeature0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DEVICE_FEATURE0 to value 0"]
impl crate::Resettable for Cfg0DeviceFeature0Spec {
    const RESET_VALUE: u32 = 0;
}
