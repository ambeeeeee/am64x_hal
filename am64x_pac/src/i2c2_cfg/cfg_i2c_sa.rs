#[doc = "Register `CFG_I2C_SA` reader"]
pub type R = crate::R<CfgI2cSaSpec>;
#[doc = "Register `CFG_I2C_SA` writer"]
pub type W = crate::W<CfgI2cSaSpec>;
#[doc = "Field `SA` reader - 9:0\\]
Slave address"]
pub type SaR = crate::FieldReader<u16>;
#[doc = "Field `SA` writer - 9:0\\]
Slave address"]
pub type SaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Slave address"]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Slave address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SaW<CfgI2cSaSpec> {
        SaW::new(self, 0)
    }
}
#[doc = "Slave address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cSaSpec;
impl crate::RegisterSpec for CfgI2cSaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_sa::R`](R) reader structure"]
impl crate::Readable for CfgI2cSaSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_sa::W`](W) writer structure"]
impl crate::Writable for CfgI2cSaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_SA to value 0x1023"]
impl crate::Resettable for CfgI2cSaSpec {
    const RESET_VALUE: u32 = 0x1023;
}
