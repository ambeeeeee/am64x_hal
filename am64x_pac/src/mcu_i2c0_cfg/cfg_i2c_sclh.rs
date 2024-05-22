#[doc = "Register `CFG_I2C_SCLH` reader"]
pub type R = crate::R<CfgI2cSclhSpec>;
#[doc = "Register `CFG_I2C_SCLH` writer"]
pub type W = crate::W<CfgI2cSclhSpec>;
#[doc = "Field `SCLH` reader - 7:0\\]
Fast/Standard mode SCL high time"]
pub type SclhR = crate::FieldReader;
#[doc = "Field `SCLH` writer - 7:0\\]
Fast/Standard mode SCL high time"]
pub type SclhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HSSCLH` reader - 15:8\\]
High Speed mode SCL high time"]
pub type HssclhR = crate::FieldReader;
#[doc = "Field `HSSCLH` writer - 15:8\\]
High Speed mode SCL high time"]
pub type HssclhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Fast/Standard mode SCL high time"]
    #[inline(always)]
    pub fn sclh(&self) -> SclhR {
        SclhR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
High Speed mode SCL high time"]
    #[inline(always)]
    pub fn hssclh(&self) -> HssclhR {
        HssclhR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Fast/Standard mode SCL high time"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SclhW<CfgI2cSclhSpec> {
        SclhW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
High Speed mode SCL high time"]
    #[inline(always)]
    #[must_use]
    pub fn hssclh(&mut self) -> HssclhW<CfgI2cSclhSpec> {
        HssclhW::new(self, 8)
    }
}
#[doc = "I2C SCL High Time Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sclh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sclh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cSclhSpec;
impl crate::RegisterSpec for CfgI2cSclhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_sclh::R`](R) reader structure"]
impl crate::Readable for CfgI2cSclhSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_sclh::W`](W) writer structure"]
impl crate::Writable for CfgI2cSclhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_SCLH to value 0"]
impl crate::Resettable for CfgI2cSclhSpec {
    const RESET_VALUE: u32 = 0;
}
