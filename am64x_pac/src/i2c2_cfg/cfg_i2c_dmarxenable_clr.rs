#[doc = "Register `CFG_I2C_DMARXENABLE_CLR` reader"]
pub type R = crate::R<CfgI2cDmarxenableClrSpec>;
#[doc = "Register `CFG_I2C_DMARXENABLE_CLR` writer"]
pub type W = crate::W<CfgI2cDmarxenableClrSpec>;
#[doc = "Field `DMARX_ENABLE_CLEAR` reader - 0:0\\]
Receive DMA channel enable clear"]
pub type DmarxEnableClearR = crate::BitReader;
#[doc = "Field `DMARX_ENABLE_CLEAR` writer - 0:0\\]
Receive DMA channel enable clear"]
pub type DmarxEnableClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA channel enable clear"]
    #[inline(always)]
    pub fn dmarx_enable_clear(&self) -> DmarxEnableClearR {
        DmarxEnableClearR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receive DMA channel enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn dmarx_enable_clear(&mut self) -> DmarxEnableClearW<CfgI2cDmarxenableClrSpec> {
        DmarxEnableClearW::new(self, 0)
    }
}
#[doc = "Per-event DMA RX enable clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_i2c_dmarxenable_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_i2c_dmarxenable_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgI2cDmarxenableClrSpec;
impl crate::RegisterSpec for CfgI2cDmarxenableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_i2c_dmarxenable_clr::R`](R) reader structure"]
impl crate::Readable for CfgI2cDmarxenableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_i2c_dmarxenable_clr::W`](W) writer structure"]
impl crate::Writable for CfgI2cDmarxenableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_I2C_DMARXENABLE_CLR to value 0"]
impl crate::Resettable for CfgI2cDmarxenableClrSpec {
    const RESET_VALUE: u32 = 0;
}
