#[doc = "Register `CFG_I2C_CNT` reader"]
pub type R = crate::R<CfgI2cCntSpec>;
#[doc = "Register `CFG_I2C_CNT` writer"]
pub type W = crate::W<CfgI2cCntSpec>;
#[doc = "Field `DCOUNT` reader - 15:0\\]
Data count"]
pub type DcountR = crate::FieldReader<u16>;
#[doc = "Field `DCOUNT` writer - 15:0\\]
Data count"]
pub type DcountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Data count"]
    #[inline(always)]
    pub fn dcount(&self) -> DcountR {
        DcountR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Data count"]
    #[inline(always)]
    #[must_use]
    pub fn dcount(&mut self) -> DcountW<CfgI2cCntSpec> {
        DcountW::new(self, 0)
    }
}
#[doc = "Data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cCntSpec;
impl crate::RegisterSpec for CfgI2cCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_cnt::R`](R) reader structure"]
impl crate::Readable for CfgI2cCntSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_cnt::W`](W) writer structure"]
impl crate::Writable for CfgI2cCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_CNT to value 0"]
impl crate::Resettable for CfgI2cCntSpec {
    const RESET_VALUE: u32 = 0;
}
