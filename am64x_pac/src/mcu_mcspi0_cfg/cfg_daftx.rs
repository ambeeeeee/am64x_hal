#[doc = "Register `CFG_DAFTX` reader"]
pub type R = crate::R<CfgDaftxSpec>;
#[doc = "Register `CFG_DAFTX` writer"]
pub type W = crate::W<CfgDaftxSpec>;
#[doc = "Field `DAFTDATA` reader - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to 1 and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value"]
pub type DaftdataR = crate::FieldReader<u32>;
#[doc = "Field `DAFTDATA` writer - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to 1 and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value"]
pub type DaftdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to 1 and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value"]
    #[inline(always)]
    pub fn daftdata(&self) -> DaftdataR {
        DaftdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
FIFO Data to transmit with DMA 256 bit aligned address This Register is only is used when MCSPI_MODULCTRL\\[FDAA\\]
is set to 1 and only one of the MCSPI_CH\\[i\\]CONF\\[FFEW\\]
of enabled channels is set If these conditions are not respected any access to this register return a null value"]
    #[inline(always)]
    #[must_use]
    pub fn daftdata(&mut self) -> DaftdataW<CfgDaftxSpec> {
        DaftdataW::new(self, 0)
    }
}
#[doc = "This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_daftx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_daftx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDaftxSpec;
impl crate::RegisterSpec for CfgDaftxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_daftx::R`](R) reader structure"]
impl crate::Readable for CfgDaftxSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_daftx::W`](W) writer structure"]
impl crate::Writable for CfgDaftxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DAFTX to value 0"]
impl crate::Resettable for CfgDaftxSpec {
    const RESET_VALUE: u32 = 0;
}
