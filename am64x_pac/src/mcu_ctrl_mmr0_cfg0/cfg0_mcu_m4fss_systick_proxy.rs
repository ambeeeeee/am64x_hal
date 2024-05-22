#[doc = "Register `CFG0_MCU_M4FSS_SYSTICK_PROXY` reader"]
pub type R = crate::R<Cfg0McuM4fssSystickProxySpec>;
#[doc = "Register `CFG0_MCU_M4FSS_SYSTICK_PROXY` writer"]
pub type W = crate::W<Cfg0McuM4fssSystickProxySpec>;
#[doc = "Field `MCU_M4FSS_SYSTICK_TENMS_PROXY` reader - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
pub type McuM4fssSystickTenmsProxyR = crate::FieldReader<u32>;
#[doc = "Field `MCU_M4FSS_SYSTICK_TENMS_PROXY` writer - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
pub type McuM4fssSystickTenmsProxyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `MCU_M4FSS_SYSTICK_SKEW_PROXY` reader - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
pub type McuM4fssSystickSkewProxyR = crate::BitReader;
#[doc = "Field `MCU_M4FSS_SYSTICK_SKEW_PROXY` writer - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
pub type McuM4fssSystickSkewProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_M4FSS_SYSTICK_NOREF_PROXY` reader - 25:25\\]
No External Reference Clock for SysTick is available"]
pub type McuM4fssSystickNorefProxyR = crate::BitReader;
#[doc = "Field `MCU_M4FSS_SYSTICK_NOREF_PROXY` writer - 25:25\\]
No External Reference Clock for SysTick is available"]
pub type McuM4fssSystickNorefProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
    #[inline(always)]
    pub fn mcu_m4fss_systick_tenms_proxy(&self) -> McuM4fssSystickTenmsProxyR {
        McuM4fssSystickTenmsProxyR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
    #[inline(always)]
    pub fn mcu_m4fss_systick_skew_proxy(&self) -> McuM4fssSystickSkewProxyR {
        McuM4fssSystickSkewProxyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
No External Reference Clock for SysTick is available"]
    #[inline(always)]
    pub fn mcu_m4fss_systick_noref_proxy(&self) -> McuM4fssSystickNorefProxyR {
        McuM4fssSystickNorefProxyR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss_systick_tenms_proxy(
        &mut self,
    ) -> McuM4fssSystickTenmsProxyW<Cfg0McuM4fssSystickProxySpec> {
        McuM4fssSystickTenmsProxyW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss_systick_skew_proxy(
        &mut self,
    ) -> McuM4fssSystickSkewProxyW<Cfg0McuM4fssSystickProxySpec> {
        McuM4fssSystickSkewProxyW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
No External Reference Clock for SysTick is available"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss_systick_noref_proxy(
        &mut self,
    ) -> McuM4fssSystickNorefProxyW<Cfg0McuM4fssSystickProxySpec> {
        McuM4fssSystickNorefProxyW::new(self, 25)
    }
}
#[doc = "CFG0_MCU_M4FSS_SYSTICK_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss_systick_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss_systick_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fssSystickProxySpec;
impl crate::RegisterSpec for Cfg0McuM4fssSystickProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss_systick_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fssSystickProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss_systick_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fssSystickProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS_SYSTICK_PROXY to value 0x0399_9999"]
impl crate::Resettable for Cfg0McuM4fssSystickProxySpec {
    const RESET_VALUE: u32 = 0x0399_9999;
}
