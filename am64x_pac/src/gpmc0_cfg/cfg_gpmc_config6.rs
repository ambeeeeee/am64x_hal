#[doc = "Register `CFG_GPMC_CONFIG6` reader"]
pub type R = crate::R<CfgGpmcConfig6Spec>;
#[doc = "Register `CFG_GPMC_CONFIG6` writer"]
pub type W = crate::W<CfgGpmcConfig6Spec>;
#[doc = "Field `BUSTURNAROUND` reader - 3:0\\]
Bus turn around latency between two successive accesses to the same chip-select \\[rd to wr\\]
or to a different chip-select \\[read to read and read to write\\]
\\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type BusturnaroundR = crate::FieldReader;
#[doc = "Field `BUSTURNAROUND` writer - 3:0\\]
Bus turn around latency between two successive accesses to the same chip-select \\[rd to wr\\]
or to a different chip-select \\[read to read and read to write\\]
\\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type BusturnaroundW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CYCLE2CYCLEDIFFCSEN` reader - 6:6\\]
Add Cycle2CycleDelay between two successive accesses to a different chip-select \\[any access type\\]"]
pub type Cycle2cyclediffcsenR = crate::BitReader;
#[doc = "Field `CYCLE2CYCLEDIFFCSEN` writer - 6:6\\]
Add Cycle2CycleDelay between two successive accesses to a different chip-select \\[any access type\\]"]
pub type Cycle2cyclediffcsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCLE2CYCLESAMECSEN` reader - 7:7\\]
Add Cycle2CycleDelay between two successive accesses to the same chip-select \\[any access type\\]"]
pub type Cycle2cyclesamecsenR = crate::BitReader;
#[doc = "Field `CYCLE2CYCLESAMECSEN` writer - 7:7\\]
Add Cycle2CycleDelay between two successive accesses to the same chip-select \\[any access type\\]"]
pub type Cycle2cyclesamecsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYCLE2CYCLEDELAY` reader - 11:8\\]
Chip select high pulse delay between two successive accesses \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type Cycle2cycledelayR = crate::FieldReader;
#[doc = "Field `CYCLE2CYCLEDELAY` writer - 11:8\\]
Chip select high pulse delay between two successive accesses \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type Cycle2cycledelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRDATAONADMUXBUS` reader - 19:16\\]
Specifies on which GPMC.FCLK rising edge the first data of the synchronous burst write is driven in the add/data mux bus"]
pub type WrdataonadmuxbusR = crate::FieldReader;
#[doc = "Field `WRDATAONADMUXBUS` writer - 19:16\\]
Specifies on which GPMC.FCLK rising edge the first data of the synchronous burst write is driven in the add/data mux bus"]
pub type WrdataonadmuxbusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRACCESSTIME` reader - 28:24\\]
Delay from StartAccessTime to the GPMC.FCLK rising edge corresponding the the GPMC.CLK rising edge used by the attached memory for the first data capture \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type WraccesstimeR = crate::FieldReader;
#[doc = "Field `WRACCESSTIME` writer - 28:24\\]
Delay from StartAccessTime to the GPMC.FCLK rising edge corresponding the the GPMC.CLK rising edge used by the attached memory for the first data capture \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type WraccesstimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Bus turn around latency between two successive accesses to the same chip-select \\[rd to wr\\]
or to a different chip-select \\[read to read and read to write\\]
\\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn busturnaround(&self) -> BusturnaroundR {
        BusturnaroundR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Add Cycle2CycleDelay between two successive accesses to a different chip-select \\[any access type\\]"]
    #[inline(always)]
    pub fn cycle2cyclediffcsen(&self) -> Cycle2cyclediffcsenR {
        Cycle2cyclediffcsenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Add Cycle2CycleDelay between two successive accesses to the same chip-select \\[any access type\\]"]
    #[inline(always)]
    pub fn cycle2cyclesamecsen(&self) -> Cycle2cyclesamecsenR {
        Cycle2cyclesamecsenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Chip select high pulse delay between two successive accesses \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn cycle2cycledelay(&self) -> Cycle2cycledelayR {
        Cycle2cycledelayR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Specifies on which GPMC.FCLK rising edge the first data of the synchronous burst write is driven in the add/data mux bus"]
    #[inline(always)]
    pub fn wrdataonadmuxbus(&self) -> WrdataonadmuxbusR {
        WrdataonadmuxbusR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Delay from StartAccessTime to the GPMC.FCLK rising edge corresponding the the GPMC.CLK rising edge used by the attached memory for the first data capture \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn wraccesstime(&self) -> WraccesstimeR {
        WraccesstimeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Bus turn around latency between two successive accesses to the same chip-select \\[rd to wr\\]
or to a different chip-select \\[read to read and read to write\\]
\\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn busturnaround(&mut self) -> BusturnaroundW<CfgGpmcConfig6Spec> {
        BusturnaroundW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Add Cycle2CycleDelay between two successive accesses to a different chip-select \\[any access type\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cycle2cyclediffcsen(&mut self) -> Cycle2cyclediffcsenW<CfgGpmcConfig6Spec> {
        Cycle2cyclediffcsenW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Add Cycle2CycleDelay between two successive accesses to the same chip-select \\[any access type\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cycle2cyclesamecsen(&mut self) -> Cycle2cyclesamecsenW<CfgGpmcConfig6Spec> {
        Cycle2cyclesamecsenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Chip select high pulse delay between two successive accesses \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cycle2cycledelay(&mut self) -> Cycle2cycledelayW<CfgGpmcConfig6Spec> {
        Cycle2cycledelayW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Specifies on which GPMC.FCLK rising edge the first data of the synchronous burst write is driven in the add/data mux bus"]
    #[inline(always)]
    #[must_use]
    pub fn wrdataonadmuxbus(&mut self) -> WrdataonadmuxbusW<CfgGpmcConfig6Spec> {
        WrdataonadmuxbusW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Delay from StartAccessTime to the GPMC.FCLK rising edge corresponding the the GPMC.CLK rising edge used by the attached memory for the first data capture \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wraccesstime(&mut self) -> WraccesstimeW<CfgGpmcConfig6Spec> {
        WraccesstimeW::new(self, 24)
    }
}
#[doc = "WrAccessTime, WrDataOnADmuxBus, Cycle2Cycle and BusTurnAround parameters configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfig6Spec;
impl crate::RegisterSpec for CfgGpmcConfig6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config6::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfig6Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config6::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfig6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG6 to value 0x9507_0000"]
impl crate::Resettable for CfgGpmcConfig6Spec {
    const RESET_VALUE: u32 = 0x9507_0000;
}
