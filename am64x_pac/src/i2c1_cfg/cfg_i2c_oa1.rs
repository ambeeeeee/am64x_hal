#[doc = "Register `CFG_I2C_OA1` reader"]
pub type R = crate::R<CfgI2cOa1Spec>;
#[doc = "Register `CFG_I2C_OA1` writer"]
pub type W = crate::W<CfgI2cOa1Spec>;
#[doc = "Field `OA1` reader - 9:0\\]
Own address 1"]
pub type Oa1R = crate::FieldReader<u16>;
#[doc = "Field `OA1` writer - 9:0\\]
Own address 1"]
pub type Oa1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Own address 1"]
    #[inline(always)]
    pub fn oa1(&self) -> Oa1R {
        Oa1R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Own address 1"]
    #[inline(always)]
    #[must_use]
    pub fn oa1(&mut self) -> Oa1W<CfgI2cOa1Spec> {
        Oa1W::new(self, 0)
    }
}
#[doc = "I2C Own Address 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_oa1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_oa1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cOa1Spec;
impl crate::RegisterSpec for CfgI2cOa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_oa1::R`](R) reader structure"]
impl crate::Readable for CfgI2cOa1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_oa1::W`](W) writer structure"]
impl crate::Writable for CfgI2cOa1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_OA1 to value 0"]
impl crate::Resettable for CfgI2cOa1Spec {
    const RESET_VALUE: u32 = 0;
}
