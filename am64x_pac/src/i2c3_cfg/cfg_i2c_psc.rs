#[doc = "Register `CFG_I2C_PSC` reader"]
pub type R = crate::R<CfgI2cPscSpec>;
#[doc = "Register `CFG_I2C_PSC` writer"]
pub type W = crate::W<CfgI2cPscSpec>;
#[doc = "Field `PSC` reader - 7:0\\]
Fast/Standard mode prescale sampling clock divider value 0x0: Divide by 1 0x1: Divide by 2 0xFF: Divide by 256"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - 7:0\\]
Fast/Standard mode prescale sampling clock divider value 0x0: Divide by 1 0x1: Divide by 2 0xFF: Divide by 256"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Fast/Standard mode prescale sampling clock divider value 0x0: Divide by 1 0x1: Divide by 2 0xFF: Divide by 256"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Fast/Standard mode prescale sampling clock divider value 0x0: Divide by 1 0x1: Divide by 2 0xFF: Divide by 256"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PscW<CfgI2cPscSpec> {
        PscW::new(self, 0)
    }
}
#[doc = "I2C Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_psc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_psc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cPscSpec;
impl crate::RegisterSpec for CfgI2cPscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_psc::R`](R) reader structure"]
impl crate::Readable for CfgI2cPscSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_psc::W`](W) writer structure"]
impl crate::Writable for CfgI2cPscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_PSC to value 0"]
impl crate::Resettable for CfgI2cPscSpec {
    const RESET_VALUE: u32 = 0;
}
