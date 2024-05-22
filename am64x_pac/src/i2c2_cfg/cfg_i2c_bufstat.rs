#[doc = "Register `CFG_I2C_BUFSTAT` reader"]
pub type R = crate::R<CfgI2cBufstatSpec>;
#[doc = "Register `CFG_I2C_BUFSTAT` writer"]
pub type W = crate::W<CfgI2cBufstatSpec>;
#[doc = "Field `TXSTAT` reader - 5:0\\]
TX Buffer Status"]
pub type TxstatR = crate::FieldReader;
#[doc = "Field `TXSTAT` writer - 5:0\\]
TX Buffer Status"]
pub type TxstatW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RXSTAT` reader - 13:8\\]
RX Buffer Status"]
pub type RxstatR = crate::FieldReader;
#[doc = "Field `RXSTAT` writer - 13:8\\]
RX Buffer Status"]
pub type RxstatW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FIFODEPTH` reader - 15:14\\]
Internal FIFO buffers depth"]
pub type FifodepthR = crate::FieldReader;
#[doc = "Field `FIFODEPTH` writer - 15:14\\]
Internal FIFO buffers depth"]
pub type FifodepthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
TX Buffer Status"]
    #[inline(always)]
    pub fn txstat(&self) -> TxstatR {
        TxstatR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
RX Buffer Status"]
    #[inline(always)]
    pub fn rxstat(&self) -> RxstatR {
        RxstatR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal FIFO buffers depth"]
    #[inline(always)]
    pub fn fifodepth(&self) -> FifodepthR {
        FifodepthR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
TX Buffer Status"]
    #[inline(always)]
    #[must_use]
    pub fn txstat(&mut self) -> TxstatW<CfgI2cBufstatSpec> {
        TxstatW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
RX Buffer Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxstat(&mut self) -> RxstatW<CfgI2cBufstatSpec> {
        RxstatW::new(self, 8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Internal FIFO buffers depth"]
    #[inline(always)]
    #[must_use]
    pub fn fifodepth(&mut self) -> FifodepthW<CfgI2cBufstatSpec> {
        FifodepthW::new(self, 14)
    }
}
#[doc = "I2C Buffer Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_bufstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_bufstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cBufstatSpec;
impl crate::RegisterSpec for CfgI2cBufstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_bufstat::R`](R) reader structure"]
impl crate::Readable for CfgI2cBufstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_bufstat::W`](W) writer structure"]
impl crate::Writable for CfgI2cBufstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_BUFSTAT to value 0x8000"]
impl crate::Resettable for CfgI2cBufstatSpec {
    const RESET_VALUE: u32 = 0x8000;
}
