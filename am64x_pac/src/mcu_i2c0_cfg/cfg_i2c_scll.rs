#[doc = "Register `CFG_I2C_SCLL` reader"]
pub type R = crate::R<CfgI2cScllSpec>;
#[doc = "Register `CFG_I2C_SCLL` writer"]
pub type W = crate::W<CfgI2cScllSpec>;
#[doc = "Field `SCLL` reader - 7:0\\]
Fast/Standard mode SCL low time"]
pub type ScllR = crate::FieldReader;
#[doc = "Field `SCLL` writer - 7:0\\]
Fast/Standard mode SCL low time"]
pub type ScllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSSCLL` reader - 15:8\\]
High Speed mode SCL low time"]
pub type HsscllR = crate::FieldReader;
#[doc = "Field `HSSCLL` writer - 15:8\\]
High Speed mode SCL low time"]
pub type HsscllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Fast/Standard mode SCL low time"]
    #[inline(always)]
    pub fn scll(&self) -> ScllR {
        ScllR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
High Speed mode SCL low time"]
    #[inline(always)]
    pub fn hsscll(&self) -> HsscllR {
        HsscllR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Fast/Standard mode SCL low time"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> ScllW<CfgI2cScllSpec> {
        ScllW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
High Speed mode SCL low time"]
    #[inline(always)]
    #[must_use]
    pub fn hsscll(&mut self) -> HsscllW<CfgI2cScllSpec> {
        HsscllW::new(self, 8)
    }
}
#[doc = "I2C SCL Low Time Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_scll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_scll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cScllSpec;
impl crate::RegisterSpec for CfgI2cScllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_scll::R`](R) reader structure"]
impl crate::Readable for CfgI2cScllSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_scll::W`](W) writer structure"]
impl crate::Writable for CfgI2cScllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_SCLL to value 0"]
impl crate::Resettable for CfgI2cScllSpec {
    const RESET_VALUE: u32 = 0;
}
