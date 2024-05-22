#[doc = "Register `CFG_I2C_SBLOCK` reader"]
pub type R = crate::R<CfgI2cSblockSpec>;
#[doc = "Register `CFG_I2C_SBLOCK` writer"]
pub type W = crate::W<CfgI2cSblockSpec>;
#[doc = "Field `OA0_EN` reader - 0:0\\]
Enable I2C Clock Blocking for Own Address 0"]
pub type Oa0EnR = crate::BitReader;
#[doc = "Field `OA0_EN` writer - 0:0\\]
Enable I2C Clock Blocking for Own Address 0"]
pub type Oa0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1_EN` reader - 1:1\\]
Enable I2C Clock Blocking for Own Address 1"]
pub type Oa1EnR = crate::BitReader;
#[doc = "Field `OA1_EN` writer - 1:1\\]
Enable I2C Clock Blocking for Own Address 1"]
pub type Oa1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA2_EN` reader - 2:2\\]
Enable I2C Clock Blocking for Own Address 2"]
pub type Oa2EnR = crate::BitReader;
#[doc = "Field `OA2_EN` writer - 2:2\\]
Enable I2C Clock Blocking for Own Address 2"]
pub type Oa2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA3_EN` reader - 3:3\\]
Enable I2C Clock Blocking for Own Address 3"]
pub type Oa3EnR = crate::BitReader;
#[doc = "Field `OA3_EN` writer - 3:3\\]
Enable I2C Clock Blocking for Own Address 3"]
pub type Oa3EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable I2C Clock Blocking for Own Address 0"]
    #[inline(always)]
    pub fn oa0_en(&self) -> Oa0EnR {
        Oa0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable I2C Clock Blocking for Own Address 1"]
    #[inline(always)]
    pub fn oa1_en(&self) -> Oa1EnR {
        Oa1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable I2C Clock Blocking for Own Address 2"]
    #[inline(always)]
    pub fn oa2_en(&self) -> Oa2EnR {
        Oa2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable I2C Clock Blocking for Own Address 3"]
    #[inline(always)]
    pub fn oa3_en(&self) -> Oa3EnR {
        Oa3EnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable I2C Clock Blocking for Own Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_en(&mut self) -> Oa0EnW<CfgI2cSblockSpec> {
        Oa0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable I2C Clock Blocking for Own Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_en(&mut self) -> Oa1EnW<CfgI2cSblockSpec> {
        Oa1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable I2C Clock Blocking for Own Address 2"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_en(&mut self) -> Oa2EnW<CfgI2cSblockSpec> {
        Oa2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable I2C Clock Blocking for Own Address 3"]
    #[inline(always)]
    #[must_use]
    pub fn oa3_en(&mut self) -> Oa3EnW<CfgI2cSblockSpec> {
        Oa3EnW::new(self, 3)
    }
}
#[doc = "I2C Clock Blocking Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_sblock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_sblock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cSblockSpec;
impl crate::RegisterSpec for CfgI2cSblockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_sblock::R`](R) reader structure"]
impl crate::Readable for CfgI2cSblockSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_sblock::W`](W) writer structure"]
impl crate::Writable for CfgI2cSblockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_SBLOCK to value 0"]
impl crate::Resettable for CfgI2cSblockSpec {
    const RESET_VALUE: u32 = 0;
}
