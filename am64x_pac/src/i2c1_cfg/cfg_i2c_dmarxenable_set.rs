#[doc = "Register `CFG_I2C_DMARXENABLE_SET` reader"]
pub type R = crate::R<CfgI2cDmarxenableSetSpec>;
#[doc = "Register `CFG_I2C_DMARXENABLE_SET` writer"]
pub type W = crate::W<CfgI2cDmarxenableSetSpec>;
#[doc = "Field `DMARX_ENABLE_SET` reader - 0:0\\]
Receive DMA channel enable set"]
pub type DmarxEnableSetR = crate::BitReader;
#[doc = "Field `DMARX_ENABLE_SET` writer - 0:0\\]
Receive DMA channel enable set"]
pub type DmarxEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA channel enable set"]
    #[inline(always)]
    pub fn dmarx_enable_set(&self) -> DmarxEnableSetR {
        DmarxEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA channel enable set"]
    #[inline(always)]
    #[must_use]
    pub fn dmarx_enable_set(&mut self) -> DmarxEnableSetW<CfgI2cDmarxenableSetSpec> {
        DmarxEnableSetW::new(self, 0)
    }
}
#[doc = "Per-event DMA RX enable set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmarxenable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmarxenable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cDmarxenableSetSpec;
impl crate::RegisterSpec for CfgI2cDmarxenableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_dmarxenable_set::R`](R) reader structure"]
impl crate::Readable for CfgI2cDmarxenableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_dmarxenable_set::W`](W) writer structure"]
impl crate::Writable for CfgI2cDmarxenableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_DMARXENABLE_SET to value 0"]
impl crate::Resettable for CfgI2cDmarxenableSetSpec {
    const RESET_VALUE: u32 = 0;
}
