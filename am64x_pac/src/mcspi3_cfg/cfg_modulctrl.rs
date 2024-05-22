#[doc = "Register `CFG_MODULCTRL` reader"]
pub type R = crate::R<CfgModulctrlSpec>;
#[doc = "Register `CFG_MODULCTRL` writer"]
pub type W = crate::W<CfgModulctrlSpec>;
#[doc = "Field `SINGLE` reader - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]"]
pub type SingleR = crate::BitReader;
#[doc = "Field `SINGLE` writer - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIN34` reader - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode, in master or slave mode If asserted the controller only use SIMO,SOMI and SPICLK clock pin for spi transfers"]
pub type Pin34R = crate::BitReader;
#[doc = "Field `PIN34` writer - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode, in master or slave mode If asserted the controller only use SIMO,SOMI and SPICLK clock pin for spi transfers"]
pub type Pin34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - 2:2\\]
Master/ Slave"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - 2:2\\]
Master/ Slave"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTEM_TEST` reader - 3:3\\]
Enables the system test mode"]
pub type SystemTestR = crate::BitReader;
#[doc = "Field `SYSTEM_TEST` writer - 3:3\\]
Enables the system test mode"]
pub type SystemTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITDLY` reader - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode, The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock, No clock output provided to the boundary and chip select is not active in 4 pin mode within this period"]
pub type InitdlyR = crate::FieldReader;
#[doc = "Field `INITDLY` writer - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode, The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock, No clock output provided to the boundary and chip select is not active in 4 pin mode within this period"]
pub type InitdlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MOA` reader - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16"]
pub type MoaR = crate::BitReader;
#[doc = "Field `MOA` writer - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16"]
pub type MoaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDAA` reader - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers"]
pub type FdaaR = crate::BitReader;
#[doc = "Field `FDAA` writer - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers"]
pub type FdaaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode, in master or slave mode If asserted the controller only use SIMO,SOMI and SPICLK clock pin for spi transfers"]
    #[inline(always)]
    pub fn pin34(&self) -> Pin34R {
        Pin34R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Master/ Slave"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the system test mode"]
    #[inline(always)]
    pub fn system_test(&self) -> SystemTestR {
        SystemTestR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode, The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock, No clock output provided to the boundary and chip select is not active in 4 pin mode within this period"]
    #[inline(always)]
    pub fn initdly(&self) -> InitdlyR {
        InitdlyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16"]
    #[inline(always)]
    pub fn moa(&self) -> MoaR {
        MoaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers"]
    #[inline(always)]
    pub fn fdaa(&self) -> FdaaR {
        FdaaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Single channel / Multi Channel \\[master mode only\\]"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SingleW<CfgModulctrlSpec> {
        SingleW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Pin mode selection: This register is used to configure the SPI pin mode, in master or slave mode If asserted the controller only use SIMO,SOMI and SPICLK clock pin for spi transfers"]
    #[inline(always)]
    #[must_use]
    pub fn pin34(&mut self) -> Pin34W<CfgModulctrlSpec> {
        Pin34W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Master/ Slave"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<CfgModulctrlSpec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables the system test mode"]
    #[inline(always)]
    #[must_use]
    pub fn system_test(&mut self) -> SystemTestW<CfgModulctrlSpec> {
        SystemTestW::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Initial spi delay for first transfer: This register is an option only available in SINGLE master mode, The controller waits for a delay to transmit the first spi word after channel enabled and corresponding TX register filled This Delay is based on SPI output frequency clock, No clock output provided to the boundary and chip select is not active in 4 pin mode within this period"]
    #[inline(always)]
    #[must_use]
    pub fn initdly(&mut self) -> InitdlyW<CfgModulctrlSpec> {
        InitdlyW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
Multiple word ocp access: This register can only be used when a channel is enabled using a FIFO It allows the system to perform multiple SPI word access for a single 32-bit OCP word access This is possible for WL &lt; 16"]
    #[inline(always)]
    #[must_use]
    pub fn moa(&mut self) -> MoaW<CfgModulctrlSpec> {
        MoaW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
FIFO DMA Address 256-bit aligned This register is used when a FIFO is managed by the module and DMA connected to the controller provides only 256 bit aligned address If this bit is set the enabled channel which uses the FIFO has its datas managed through MCSPI_DAFTX and MCSPI_DAFRX registers instead of MCSPI_TX\\[i\\]
and MCSPI_RX\\[i\\]
registers"]
    #[inline(always)]
    #[must_use]
    pub fn fdaa(&mut self) -> FdaaW<CfgModulctrlSpec> {
        FdaaW::new(self, 8)
    }
}
#[doc = "This register is dedicated to the configuration of the serial port interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_modulctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_modulctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgModulctrlSpec;
impl crate::RegisterSpec for CfgModulctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_modulctrl::R`](R) reader structure"]
impl crate::Readable for CfgModulctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_modulctrl::W`](W) writer structure"]
impl crate::Writable for CfgModulctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_MODULCTRL to value 0x04"]
impl crate::Resettable for CfgModulctrlSpec {
    const RESET_VALUE: u32 = 0x04;
}
