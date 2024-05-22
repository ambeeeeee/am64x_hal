#[doc = "Register `CFG0_IO_VOLTAGE_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0IoVoltageStatProxySpec>;
#[doc = "Register `CFG0_IO_VOLTAGE_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0IoVoltageStatProxySpec>;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_GEN_PROXY` reader - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
pub type IoVoltageStatMcuGenProxyR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_GEN_PROXY` writer - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
pub type IoVoltageStatMcuGenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_FLASH_PROXY` reader - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
pub type IoVoltageStatMcuFlashProxyR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_FLASH_PROXY` writer - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
pub type IoVoltageStatMcuFlashProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GEN_PROXY` reader - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
pub type IoVoltageStatMainGenProxyR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GEN_PROXY` writer - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
pub type IoVoltageStatMainGenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_MMC1_PROXY` reader - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
pub type IoVoltageStatMainMmc1ProxyR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_MMC1_PROXY` writer - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
pub type IoVoltageStatMainMmc1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG0_PROXY` reader - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
pub type IoVoltageStatMainPrg0ProxyR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG0_PROXY` writer - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
pub type IoVoltageStatMainPrg0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG1_PROXY` reader - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
pub type IoVoltageStatMainPrg1ProxyR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG1_PROXY` writer - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
pub type IoVoltageStatMainPrg1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GPMC_PROXY` reader - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
pub type IoVoltageStatMainGpmcProxyR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GPMC_PROXY` writer - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
pub type IoVoltageStatMainGpmcProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
    #[inline(always)]
    pub fn io_voltage_stat_mcu_gen_proxy(&self) -> IoVoltageStatMcuGenProxyR {
        IoVoltageStatMcuGenProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
    #[inline(always)]
    pub fn io_voltage_stat_mcu_flash_proxy(&self) -> IoVoltageStatMcuFlashProxyR {
        IoVoltageStatMcuFlashProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_gen_proxy(&self) -> IoVoltageStatMainGenProxyR {
        IoVoltageStatMainGenProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_mmc1_proxy(&self) -> IoVoltageStatMainMmc1ProxyR {
        IoVoltageStatMainMmc1ProxyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_prg0_proxy(&self) -> IoVoltageStatMainPrg0ProxyR {
        IoVoltageStatMainPrg0ProxyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_prg1_proxy(&self) -> IoVoltageStatMainPrg1ProxyR {
        IoVoltageStatMainPrg1ProxyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_gpmc_proxy(&self) -> IoVoltageStatMainGpmcProxyR {
        IoVoltageStatMainGpmcProxyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_mcu_gen_proxy(
        &mut self,
    ) -> IoVoltageStatMcuGenProxyW<Cfg0IoVoltageStatProxySpec> {
        IoVoltageStatMcuGenProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_mcu_flash_proxy(
        &mut self,
    ) -> IoVoltageStatMcuFlashProxyW<Cfg0IoVoltageStatProxySpec> {
        IoVoltageStatMcuFlashProxyW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_gen_proxy(
        &mut self,
    ) -> IoVoltageStatMainGenProxyW<Cfg0IoVoltageStatProxySpec> {
        IoVoltageStatMainGenProxyW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_mmc1_proxy(
        &mut self,
    ) -> IoVoltageStatMainMmc1ProxyW<Cfg0IoVoltageStatProxySpec> {
        IoVoltageStatMainMmc1ProxyW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_prg0_proxy(
        &mut self,
    ) -> IoVoltageStatMainPrg0ProxyW<Cfg0IoVoltageStatProxySpec> {
        IoVoltageStatMainPrg0ProxyW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_prg1_proxy(
        &mut self,
    ) -> IoVoltageStatMainPrg1ProxyW<Cfg0IoVoltageStatProxySpec> {
        IoVoltageStatMainPrg1ProxyW::new(self, 13)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_gpmc_proxy(
        &mut self,
    ) -> IoVoltageStatMainGpmcProxyW<Cfg0IoVoltageStatProxySpec> {
        IoVoltageStatMainGpmcProxyW::new(self, 17)
    }
}
#[doc = "CFG0_IO_VOLTAGE_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_io_voltage_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_io_voltage_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IoVoltageStatProxySpec;
impl crate::RegisterSpec for Cfg0IoVoltageStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_io_voltage_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0IoVoltageStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_io_voltage_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0IoVoltageStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_IO_VOLTAGE_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0IoVoltageStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
