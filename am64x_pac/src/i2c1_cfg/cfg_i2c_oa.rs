#[doc = "Register `CFG_I2C_OA` reader"]
pub type R = crate::R<CfgI2cOaSpec>;
#[doc = "Register `CFG_I2C_OA` writer"]
pub type W = crate::W<CfgI2cOaSpec>;
#[doc = "Field `OA` reader - 9:0\\]
Own address"]
pub type OaR = crate::FieldReader<u16>;
#[doc = "Field `OA` writer - 9:0\\]
Own address"]
pub type OaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MCODE` reader - 15:13\\]
Master Code"]
pub type McodeR = crate::FieldReader;
#[doc = "Field `MCODE` writer - 15:13\\]
Master Code"]
pub type McodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Own address"]
    #[inline(always)]
    pub fn oa(&self) -> OaR {
        OaR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Master Code"]
    #[inline(always)]
    pub fn mcode(&self) -> McodeR {
        McodeR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Own address"]
    #[inline(always)]
    #[must_use]
    pub fn oa(&mut self) -> OaW<CfgI2cOaSpec> {
        OaW::new(self, 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Master Code"]
    #[inline(always)]
    #[must_use]
    pub fn mcode(&mut self) -> McodeW<CfgI2cOaSpec> {
        McodeW::new(self, 13)
    }
}
#[doc = "Own address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_oa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_oa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cOaSpec;
impl crate::RegisterSpec for CfgI2cOaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_oa::R`](R) reader structure"]
impl crate::Readable for CfgI2cOaSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_oa::W`](W) writer structure"]
impl crate::Writable for CfgI2cOaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_OA to value 0"]
impl crate::Resettable for CfgI2cOaSpec {
    const RESET_VALUE: u32 = 0;
}
