#[doc = "Register `CFG0_MCU_M4FSS_SYSTICK` reader"]
pub type R = crate::R<Cfg0McuM4fssSystickSpec>;
#[doc = "Register `CFG0_MCU_M4FSS_SYSTICK` writer"]
pub type W = crate::W<Cfg0McuM4fssSystickSpec>;
#[doc = "Field `MCU_M4FSS_SYSTICK_TENMS` reader - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
pub type McuM4fssSystickTenmsR = crate::FieldReader<u32>;
#[doc = "Field `MCU_M4FSS_SYSTICK_TENMS` writer - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
pub type McuM4fssSystickTenmsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `MCU_M4FSS_SYSTICK_SKEW` reader - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
pub type McuM4fssSystickSkewR = crate::BitReader;
#[doc = "Field `MCU_M4FSS_SYSTICK_SKEW` writer - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
pub type McuM4fssSystickSkewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_M4FSS_SYSTICK_NOREF` reader - 25:25\\]
No External Reference Clock for SysTick is available"]
pub type McuM4fssSystickNorefR = crate::BitReader;
#[doc = "Field `MCU_M4FSS_SYSTICK_NOREF` writer - 25:25\\]
No External Reference Clock for SysTick is available"]
pub type McuM4fssSystickNorefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
    #[inline(always)]
    pub fn mcu_m4fss_systick_tenms(&self) -> McuM4fssSystickTenmsR {
        McuM4fssSystickTenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
    #[inline(always)]
    pub fn mcu_m4fss_systick_skew(&self) -> McuM4fssSystickSkewR {
        McuM4fssSystickSkewR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
No External Reference Clock for SysTick is available"]
    #[inline(always)]
    pub fn mcu_m4fss_systick_noref(&self) -> McuM4fssSystickNorefR {
        McuM4fssSystickNorefR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Integer Divider Value that produces a 100Hz SysTick from the M4FSS Clock Frequency. Default is 3,999,999 (4M - 1) to produce a 100Hz clock from a 400 MHz M4FSS Clock."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss_systick_tenms(&mut self) -> McuM4fssSystickTenmsW<Cfg0McuM4fssSystickSpec> {
        McuM4fssSystickTenmsW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates whether or not the M4FSS free running clock divided by TENMS produces a 100Hz tick exactly"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss_systick_skew(&mut self) -> McuM4fssSystickSkewW<Cfg0McuM4fssSystickSpec> {
        McuM4fssSystickSkewW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
No External Reference Clock for SysTick is available"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss_systick_noref(&mut self) -> McuM4fssSystickNorefW<Cfg0McuM4fssSystickSpec> {
        McuM4fssSystickNorefW::new(self, 25)
    }
}
#[doc = "CFG0_MCU_M4FSS_SYSTICK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss_systick::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss_systick::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fssSystickSpec;
impl crate::RegisterSpec for Cfg0McuM4fssSystickSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss_systick::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fssSystickSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss_systick::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fssSystickSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS_SYSTICK to value 0x0399_9999"]
impl crate::Resettable for Cfg0McuM4fssSystickSpec {
    const RESET_VALUE: u32 = 0x0399_9999;
}
