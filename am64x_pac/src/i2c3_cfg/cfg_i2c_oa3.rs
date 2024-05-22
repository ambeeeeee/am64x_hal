#[doc = "Register `CFG_I2C_OA3` reader"]
pub type R = crate::R<CfgI2cOa3Spec>;
#[doc = "Register `CFG_I2C_OA3` writer"]
pub type W = crate::W<CfgI2cOa3Spec>;
#[doc = "Field `OA3` reader - 9:0\\]
Own address 3"]
pub type Oa3R = crate::FieldReader<u16>;
#[doc = "Field `OA3` writer - 9:0\\]
Own address 3"]
pub type Oa3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Own address 3"]
    #[inline(always)]
    pub fn oa3(&self) -> Oa3R {
        Oa3R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Own address 3"]
    #[inline(always)]
    #[must_use]
    pub fn oa3(&mut self) -> Oa3W<CfgI2cOa3Spec> {
        Oa3W::new(self, 0)
    }
}
#[doc = "I2C Own Address 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_oa3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_oa3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cOa3Spec;
impl crate::RegisterSpec for CfgI2cOa3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_oa3::R`](R) reader structure"]
impl crate::Readable for CfgI2cOa3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_oa3::W`](W) writer structure"]
impl crate::Writable for CfgI2cOa3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_OA3 to value 0"]
impl crate::Resettable for CfgI2cOa3Spec {
    const RESET_VALUE: u32 = 0;
}
