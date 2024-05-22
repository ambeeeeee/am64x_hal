#[doc = "Register `CFG0_IO_VOLTAGE_STAT` reader"]
pub type R = crate::R<Cfg0IoVoltageStatSpec>;
#[doc = "Register `CFG0_IO_VOLTAGE_STAT` writer"]
pub type W = crate::W<Cfg0IoVoltageStatSpec>;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_GEN` reader - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
pub type IoVoltageStatMcuGenR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_GEN` writer - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
pub type IoVoltageStatMcuGenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_FLASH` reader - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
pub type IoVoltageStatMcuFlashR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MCU_FLASH` writer - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
pub type IoVoltageStatMcuFlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GEN` reader - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
pub type IoVoltageStatMainGenR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GEN` writer - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
pub type IoVoltageStatMainGenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_MMC1` reader - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
pub type IoVoltageStatMainMmc1R = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_MMC1` writer - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
pub type IoVoltageStatMainMmc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG0` reader - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
pub type IoVoltageStatMainPrg0R = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG0` writer - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
pub type IoVoltageStatMainPrg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG1` reader - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
pub type IoVoltageStatMainPrg1R = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_PRG1` writer - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
pub type IoVoltageStatMainPrg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GPMC` reader - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
pub type IoVoltageStatMainGpmcR = crate::BitReader;
#[doc = "Field `IO_VOLTAGE_STAT_MAIN_GPMC` writer - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
pub type IoVoltageStatMainGpmcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
    #[inline(always)]
    pub fn io_voltage_stat_mcu_gen(&self) -> IoVoltageStatMcuGenR {
        IoVoltageStatMcuGenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
    #[inline(always)]
    pub fn io_voltage_stat_mcu_flash(&self) -> IoVoltageStatMcuFlashR {
        IoVoltageStatMcuFlashR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_gen(&self) -> IoVoltageStatMainGenR {
        IoVoltageStatMainGenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_mmc1(&self) -> IoVoltageStatMainMmc1R {
        IoVoltageStatMainMmc1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_prg0(&self) -> IoVoltageStatMainPrg0R {
        IoVoltageStatMainPrg0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_prg1(&self) -> IoVoltageStatMainPrg1R {
        IoVoltageStatMainPrg1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
    #[inline(always)]
    pub fn io_voltage_stat_main_gpmc(&self) -> IoVoltageStatMainGpmcR {
        IoVoltageStatMainGpmcR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the voltage for the MCU General I/O group (VDDSHV0_MCU)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_mcu_gen(&mut self) -> IoVoltageStatMcuGenW<Cfg0IoVoltageStatSpec> {
        IoVoltageStatMcuGenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates the voltage for the Flash I/O group (VDDSHV4)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_mcu_flash(&mut self) -> IoVoltageStatMcuFlashW<Cfg0IoVoltageStatSpec> {
        IoVoltageStatMcuFlashW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates the voltage for the General I/O group (VDDSHV0)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_gen(&mut self) -> IoVoltageStatMainGenW<Cfg0IoVoltageStatSpec> {
        IoVoltageStatMainGenW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
Indicates the voltage for the MMC1 I/O group (VDDSHV5)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_mmc1(&mut self) -> IoVoltageStatMainMmc1W<Cfg0IoVoltageStatSpec> {
        IoVoltageStatMainMmc1W::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
Indicates the voltage for the PRG0 I/O group (VDDSHV2)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_prg0(&mut self) -> IoVoltageStatMainPrg0W<Cfg0IoVoltageStatSpec> {
        IoVoltageStatMainPrg0W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Indicates the voltage for the PRG1 I/O group (VDDSHV1)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_prg1(&mut self) -> IoVoltageStatMainPrg1W<Cfg0IoVoltageStatSpec> {
        IoVoltageStatMainPrg1W::new(self, 13)
    }
    #[doc = "Bit 17 - 17:17\\]
Indicates the voltage for the GPMC I/O group (VDDSHV3)"]
    #[inline(always)]
    #[must_use]
    pub fn io_voltage_stat_main_gpmc(&mut self) -> IoVoltageStatMainGpmcW<Cfg0IoVoltageStatSpec> {
        IoVoltageStatMainGpmcW::new(self, 17)
    }
}
#[doc = "CFG0_IO_VOLTAGE_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_io_voltage_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_io_voltage_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IoVoltageStatSpec;
impl crate::RegisterSpec for Cfg0IoVoltageStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_io_voltage_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0IoVoltageStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_io_voltage_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0IoVoltageStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_IO_VOLTAGE_STAT to value 0"]
impl crate::Resettable for Cfg0IoVoltageStatSpec {
    const RESET_VALUE: u32 = 0;
}
