#[doc = "Register `CFG_I2C_DATA` reader"]
pub type R = crate::R<CfgI2cDataSpec>;
#[doc = "Register `CFG_I2C_DATA` writer"]
pub type W = crate::W<CfgI2cDataSpec>;
#[doc = "Field `DATA` reader - 7:0\\]
Transmit/Receive data FIFO endpoint"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - 7:0\\]
Transmit/Receive data FIFO endpoint"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit/Receive data FIFO endpoint"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit/Receive data FIFO endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<CfgI2cDataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Data access register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cDataSpec;
impl crate::RegisterSpec for CfgI2cDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_data::R`](R) reader structure"]
impl crate::Readable for CfgI2cDataSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_data::W`](W) writer structure"]
impl crate::Writable for CfgI2cDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_DATA to value 0"]
impl crate::Resettable for CfgI2cDataSpec {
    const RESET_VALUE: u32 = 0;
}
