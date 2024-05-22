#[doc = "Register `CFG_I2C_DMATXENABLE_SET` reader"]
pub type R = crate::R<CfgI2cDmatxenableSetSpec>;
#[doc = "Register `CFG_I2C_DMATXENABLE_SET` writer"]
pub type W = crate::W<CfgI2cDmatxenableSetSpec>;
#[doc = "Field `DMATX_ENABLE_SET` reader - 0:0\\]
Transmit DMA channel enable set"]
pub type DmatxEnableSetR = crate::BitReader;
#[doc = "Field `DMATX_ENABLE_SET` writer - 0:0\\]
Transmit DMA channel enable set"]
pub type DmatxEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit DMA channel enable set"]
    #[inline(always)]
    pub fn dmatx_enable_set(&self) -> DmatxEnableSetR {
        DmatxEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit DMA channel enable set"]
    #[inline(always)]
    #[must_use]
    pub fn dmatx_enable_set(&mut self) -> DmatxEnableSetW<CfgI2cDmatxenableSetSpec> {
        DmatxEnableSetW::new(self, 0)
    }
}
#[doc = "Per-event DMA TX enable set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmatxenable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmatxenable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cDmatxenableSetSpec;
impl crate::RegisterSpec for CfgI2cDmatxenableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_dmatxenable_set::R`](R) reader structure"]
impl crate::Readable for CfgI2cDmatxenableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_dmatxenable_set::W`](W) writer structure"]
impl crate::Writable for CfgI2cDmatxenableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_DMATXENABLE_SET to value 0"]
impl crate::Resettable for CfgI2cDmatxenableSetSpec {
    const RESET_VALUE: u32 = 0;
}
