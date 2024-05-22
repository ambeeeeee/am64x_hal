#[doc = "Register `CFG_I2C_BUF` reader"]
pub type R = crate::R<CfgI2cBufSpec>;
#[doc = "Register `CFG_I2C_BUF` writer"]
pub type W = crate::W<CfgI2cBufSpec>;
#[doc = "Field `TXTRSH` reader - 5:0\\]
Threshold value for FIFO buffer in TX mode"]
pub type TxtrshR = crate::FieldReader;
#[doc = "Field `TXTRSH` writer - 5:0\\]
Threshold value for FIFO buffer in TX mode"]
pub type TxtrshW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TXFIFO_CLR` reader - 6:6\\]
Transmit FIFO clear"]
pub type TxfifoClrR = crate::BitReader;
#[doc = "Field `TXFIFO_CLR` writer - 6:6\\]
Transmit FIFO clear"]
pub type TxfifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XDMA_EN` reader - 7:7\\]
Transmit DMA channel enable"]
pub type XdmaEnR = crate::BitReader;
#[doc = "Field `XDMA_EN` writer - 7:7\\]
Transmit DMA channel enable"]
pub type XdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTRSH` reader - 13:8\\]
Threshold value for FIFO buffer in RX mode"]
pub type RxtrshR = crate::FieldReader;
#[doc = "Field `RXTRSH` writer - 13:8\\]
Threshold value for FIFO buffer in RX mode"]
pub type RxtrshW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RXFIFO_CLR` reader - 14:14\\]
Receive FIFO clear"]
pub type RxfifoClrR = crate::BitReader;
#[doc = "Field `RXFIFO_CLR` writer - 14:14\\]
Receive FIFO clear"]
pub type RxfifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMA_EN` reader - 15:15\\]
Receive DMA channel enable"]
pub type RdmaEnR = crate::BitReader;
#[doc = "Field `RDMA_EN` writer - 15:15\\]
Receive DMA channel enable"]
pub type RdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Threshold value for FIFO buffer in TX mode"]
    #[inline(always)]
    pub fn txtrsh(&self) -> TxtrshR {
        TxtrshR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit FIFO clear"]
    #[inline(always)]
    pub fn txfifo_clr(&self) -> TxfifoClrR {
        TxfifoClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Transmit DMA channel enable"]
    #[inline(always)]
    pub fn xdma_en(&self) -> XdmaEnR {
        XdmaEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Threshold value for FIFO buffer in RX mode"]
    #[inline(always)]
    pub fn rxtrsh(&self) -> RxtrshR {
        RxtrshR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receive FIFO clear"]
    #[inline(always)]
    pub fn rxfifo_clr(&self) -> RxfifoClrR {
        RxfifoClrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Receive DMA channel enable"]
    #[inline(always)]
    pub fn rdma_en(&self) -> RdmaEnR {
        RdmaEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Threshold value for FIFO buffer in TX mode"]
    #[inline(always)]
    #[must_use]
    pub fn txtrsh(&mut self) -> TxtrshW<CfgI2cBufSpec> {
        TxtrshW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit FIFO clear"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_clr(&mut self) -> TxfifoClrW<CfgI2cBufSpec> {
        TxfifoClrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Transmit DMA channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn xdma_en(&mut self) -> XdmaEnW<CfgI2cBufSpec> {
        XdmaEnW::new(self, 7)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Threshold value for FIFO buffer in RX mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxtrsh(&mut self) -> RxtrshW<CfgI2cBufSpec> {
        RxtrshW::new(self, 8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receive FIFO clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_clr(&mut self) -> RxfifoClrW<CfgI2cBufSpec> {
        RxfifoClrW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Receive DMA channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdma_en(&mut self) -> RdmaEnW<CfgI2cBufSpec> {
        RdmaEnW::new(self, 15)
    }
}
#[doc = "Buffer Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_buf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_buf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cBufSpec;
impl crate::RegisterSpec for CfgI2cBufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_buf::R`](R) reader structure"]
impl crate::Readable for CfgI2cBufSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_buf::W`](W) writer structure"]
impl crate::Writable for CfgI2cBufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_BUF to value 0"]
impl crate::Resettable for CfgI2cBufSpec {
    const RESET_VALUE: u32 = 0;
}
