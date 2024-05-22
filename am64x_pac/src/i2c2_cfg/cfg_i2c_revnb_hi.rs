#[doc = "Register `CFG_I2C_REVNB_HI` reader"]
pub type R = crate::R<CfgI2cRevnbHiSpec>;
#[doc = "Register `CFG_I2C_REVNB_HI` writer"]
pub type W = crate::W<CfgI2cRevnbHiSpec>;
#[doc = "Field `FUNC` reader - 11:0\\]
Function: Indicates a software compatible module family"]
pub type FuncR = crate::FieldReader<u16>;
#[doc = "Field `FUNC` writer - 11:0\\]
Function: Indicates a software compatible module family"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SCHEME` reader - 15:14\\]
Used to distinguish between old Scheme and current Spare bit to encode future schemes"]
pub type SchemeR = crate::FieldReader;
#[doc = "Field `SCHEME` writer - 15:14\\]
Used to distinguish between old Scheme and current Spare bit to encode future schemes"]
pub type SchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Function: Indicates a software compatible module family"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Used to distinguish between old Scheme and current Spare bit to encode future schemes"]
    #[inline(always)]
    pub fn scheme(&self) -> SchemeR {
        SchemeR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Function: Indicates a software compatible module family"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FuncW<CfgI2cRevnbHiSpec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Used to distinguish between old Scheme and current Spare bit to encode future schemes"]
    #[inline(always)]
    #[must_use]
    pub fn scheme(&mut self) -> SchemeW<CfgI2cRevnbHiSpec> {
        SchemeW::new(self, 14)
    }
}
#[doc = "Revision Number register (High)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_revnb_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_revnb_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cRevnbHiSpec;
impl crate::RegisterSpec for CfgI2cRevnbHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_revnb_hi::R`](R) reader structure"]
impl crate::Readable for CfgI2cRevnbHiSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_revnb_hi::W`](W) writer structure"]
impl crate::Writable for CfgI2cRevnbHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_REVNB_HI to value 0x5064"]
impl crate::Resettable for CfgI2cRevnbHiSpec {
    const RESET_VALUE: u32 = 0x5064;
}
