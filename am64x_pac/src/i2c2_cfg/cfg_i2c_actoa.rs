#[doc = "Register `CFG_I2C_ACTOA` reader"]
pub type R = crate::R<CfgI2cActoaSpec>;
#[doc = "Register `CFG_I2C_ACTOA` writer"]
pub type W = crate::W<CfgI2cActoaSpec>;
#[doc = "Field `OA0_ACT` reader - 0:0\\]
Own Address 0 active"]
pub type Oa0ActR = crate::BitReader;
#[doc = "Field `OA0_ACT` writer - 0:0\\]
Own Address 0 active"]
pub type Oa0ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1_ACT` reader - 1:1\\]
Own Address 1 active"]
pub type Oa1ActR = crate::BitReader;
#[doc = "Field `OA1_ACT` writer - 1:1\\]
Own Address 1 active"]
pub type Oa1ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA2_ACT` reader - 2:2\\]
Own Address 2 active"]
pub type Oa2ActR = crate::BitReader;
#[doc = "Field `OA2_ACT` writer - 2:2\\]
Own Address 2 active"]
pub type Oa2ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA3_ACT` reader - 3:3\\]
Own Address 3 active"]
pub type Oa3ActR = crate::BitReader;
#[doc = "Field `OA3_ACT` writer - 3:3\\]
Own Address 3 active"]
pub type Oa3ActW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Own Address 0 active"]
    #[inline(always)]
    pub fn oa0_act(&self) -> Oa0ActR {
        Oa0ActR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Own Address 1 active"]
    #[inline(always)]
    pub fn oa1_act(&self) -> Oa1ActR {
        Oa1ActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Own Address 2 active"]
    #[inline(always)]
    pub fn oa2_act(&self) -> Oa2ActR {
        Oa2ActR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Own Address 3 active"]
    #[inline(always)]
    pub fn oa3_act(&self) -> Oa3ActR {
        Oa3ActR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Own Address 0 active"]
    #[inline(always)]
    #[must_use]
    pub fn oa0_act(&mut self) -> Oa0ActW<CfgI2cActoaSpec> {
        Oa0ActW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Own Address 1 active"]
    #[inline(always)]
    #[must_use]
    pub fn oa1_act(&mut self) -> Oa1ActW<CfgI2cActoaSpec> {
        Oa1ActW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Own Address 2 active"]
    #[inline(always)]
    #[must_use]
    pub fn oa2_act(&mut self) -> Oa2ActW<CfgI2cActoaSpec> {
        Oa2ActW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Own Address 3 active"]
    #[inline(always)]
    #[must_use]
    pub fn oa3_act(&mut self) -> Oa3ActW<CfgI2cActoaSpec> {
        Oa3ActW::new(self, 3)
    }
}
#[doc = "I2C Active Own Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_actoa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_actoa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cActoaSpec;
impl crate::RegisterSpec for CfgI2cActoaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_actoa::R`](R) reader structure"]
impl crate::Readable for CfgI2cActoaSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_actoa::W`](W) writer structure"]
impl crate::Writable for CfgI2cActoaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_ACTOA to value 0"]
impl crate::Resettable for CfgI2cActoaSpec {
    const RESET_VALUE: u32 = 0;
}
