#[doc = "Register `CFG_I2C_SYSS` reader"]
pub type R = crate::R<CfgI2cSyssSpec>;
#[doc = "Register `CFG_I2C_SYSS` writer"]
pub type W = crate::W<CfgI2cSyssSpec>;
#[doc = "Field `RDONE` reader - 0:0\\]
Reset done bit"]
pub type RdoneR = crate::BitReader;
#[doc = "Field `RDONE` writer - 0:0\\]
Reset done bit"]
pub type RdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reset done bit"]
    #[inline(always)]
    pub fn rdone(&self) -> RdoneR {
        RdoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reset done bit"]
    #[inline(always)]
    #[must_use]
    pub fn rdone(&mut self) -> RdoneW<CfgI2cSyssSpec> {
        RdoneW::new(self, 0)
    }
}
#[doc = "System Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_syss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_syss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cSyssSpec;
impl crate::RegisterSpec for CfgI2cSyssSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_syss::R`](R) reader structure"]
impl crate::Readable for CfgI2cSyssSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_syss::W`](W) writer structure"]
impl crate::Writable for CfgI2cSyssSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_SYSS to value 0"]
impl crate::Resettable for CfgI2cSyssSpec {
    const RESET_VALUE: u32 = 0;
}
