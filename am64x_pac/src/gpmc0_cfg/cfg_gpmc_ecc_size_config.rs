#[doc = "Register `CFG_GPMC_ECC_SIZE_CONFIG` reader"]
pub type R = crate::R<CfgGpmcEccSizeConfigSpec>;
#[doc = "Register `CFG_GPMC_ECC_SIZE_CONFIG` writer"]
pub type W = crate::W<CfgGpmcEccSizeConfigSpec>;
#[doc = "Field `ECC1RESULTSIZE` reader - 0:0\\]
Selects ECC size for ECC 1 result register"]
pub type Ecc1resultsizeR = crate::BitReader;
#[doc = "Field `ECC1RESULTSIZE` writer - 0:0\\]
Selects ECC size for ECC 1 result register"]
pub type Ecc1resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC2RESULTSIZE` reader - 1:1\\]
Selects ECC size for ECC 2 result register"]
pub type Ecc2resultsizeR = crate::BitReader;
#[doc = "Field `ECC2RESULTSIZE` writer - 1:1\\]
Selects ECC size for ECC 2 result register"]
pub type Ecc2resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC3RESULTSIZE` reader - 2:2\\]
Selects ECC size for ECC 3 result register"]
pub type Ecc3resultsizeR = crate::BitReader;
#[doc = "Field `ECC3RESULTSIZE` writer - 2:2\\]
Selects ECC size for ECC 3 result register"]
pub type Ecc3resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC4RESULTSIZE` reader - 3:3\\]
Selects ECC size for ECC 4 result register"]
pub type Ecc4resultsizeR = crate::BitReader;
#[doc = "Field `ECC4RESULTSIZE` writer - 3:3\\]
Selects ECC size for ECC 4 result register"]
pub type Ecc4resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC5RESULTSIZE` reader - 4:4\\]
Selects ECC size for ECC 5 result register"]
pub type Ecc5resultsizeR = crate::BitReader;
#[doc = "Field `ECC5RESULTSIZE` writer - 4:4\\]
Selects ECC size for ECC 5 result register"]
pub type Ecc5resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC6RESULTSIZE` reader - 5:5\\]
Selects ECC size for ECC 6 result register"]
pub type Ecc6resultsizeR = crate::BitReader;
#[doc = "Field `ECC6RESULTSIZE` writer - 5:5\\]
Selects ECC size for ECC 6 result register"]
pub type Ecc6resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC7RESULTSIZE` reader - 6:6\\]
Selects ECC size for ECC 7 result register"]
pub type Ecc7resultsizeR = crate::BitReader;
#[doc = "Field `ECC7RESULTSIZE` writer - 6:6\\]
Selects ECC size for ECC 7 result register"]
pub type Ecc7resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC8RESULTSIZE` reader - 7:7\\]
Selects ECC size for ECC 8 result register"]
pub type Ecc8resultsizeR = crate::BitReader;
#[doc = "Field `ECC8RESULTSIZE` writer - 7:7\\]
Selects ECC size for ECC 8 result register"]
pub type Ecc8resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC9RESULTSIZE` reader - 8:8\\]
Selects ECC size for ECC 9 result register"]
pub type Ecc9resultsizeR = crate::BitReader;
#[doc = "Field `ECC9RESULTSIZE` writer - 8:8\\]
Selects ECC size for ECC 9 result register"]
pub type Ecc9resultsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSIZE0` reader - 19:12\\]
Defines ECC size 0 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
pub type Eccsize0R = crate::FieldReader;
#[doc = "Field `ECCSIZE0` writer - 19:12\\]
Defines ECC size 0 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
pub type Eccsize0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ECCSIZE1` reader - 29:22\\]
Defines ECC size 1 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
pub type Eccsize1R = crate::FieldReader;
#[doc = "Field `ECCSIZE1` writer - 29:22\\]
Defines ECC size 1 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
pub type Eccsize1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects ECC size for ECC 1 result register"]
    #[inline(always)]
    pub fn ecc1resultsize(&self) -> Ecc1resultsizeR {
        Ecc1resultsizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Selects ECC size for ECC 2 result register"]
    #[inline(always)]
    pub fn ecc2resultsize(&self) -> Ecc2resultsizeR {
        Ecc2resultsizeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Selects ECC size for ECC 3 result register"]
    #[inline(always)]
    pub fn ecc3resultsize(&self) -> Ecc3resultsizeR {
        Ecc3resultsizeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Selects ECC size for ECC 4 result register"]
    #[inline(always)]
    pub fn ecc4resultsize(&self) -> Ecc4resultsizeR {
        Ecc4resultsizeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Selects ECC size for ECC 5 result register"]
    #[inline(always)]
    pub fn ecc5resultsize(&self) -> Ecc5resultsizeR {
        Ecc5resultsizeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Selects ECC size for ECC 6 result register"]
    #[inline(always)]
    pub fn ecc6resultsize(&self) -> Ecc6resultsizeR {
        Ecc6resultsizeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Selects ECC size for ECC 7 result register"]
    #[inline(always)]
    pub fn ecc7resultsize(&self) -> Ecc7resultsizeR {
        Ecc7resultsizeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects ECC size for ECC 8 result register"]
    #[inline(always)]
    pub fn ecc8resultsize(&self) -> Ecc8resultsizeR {
        Ecc8resultsizeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects ECC size for ECC 9 result register"]
    #[inline(always)]
    pub fn ecc9resultsize(&self) -> Ecc9resultsizeR {
        Ecc9resultsizeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:19 - 19:12\\]
Defines ECC size 0 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
    #[inline(always)]
    pub fn eccsize0(&self) -> Eccsize0R {
        Eccsize0R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 22:29 - 29:22\\]
Defines ECC size 1 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
    #[inline(always)]
    pub fn eccsize1(&self) -> Eccsize1R {
        Eccsize1R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects ECC size for ECC 1 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc1resultsize(&mut self) -> Ecc1resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc1resultsizeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Selects ECC size for ECC 2 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc2resultsize(&mut self) -> Ecc2resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc2resultsizeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Selects ECC size for ECC 3 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc3resultsize(&mut self) -> Ecc3resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc3resultsizeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Selects ECC size for ECC 4 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc4resultsize(&mut self) -> Ecc4resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc4resultsizeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Selects ECC size for ECC 5 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc5resultsize(&mut self) -> Ecc5resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc5resultsizeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Selects ECC size for ECC 6 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc6resultsize(&mut self) -> Ecc6resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc6resultsizeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Selects ECC size for ECC 7 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc7resultsize(&mut self) -> Ecc7resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc7resultsizeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects ECC size for ECC 8 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc8resultsize(&mut self) -> Ecc8resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc8resultsizeW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects ECC size for ECC 9 result register"]
    #[inline(always)]
    #[must_use]
    pub fn ecc9resultsize(&mut self) -> Ecc9resultsizeW<CfgGpmcEccSizeConfigSpec> {
        Ecc9resultsizeW::new(self, 8)
    }
    #[doc = "Bits 12:19 - 19:12\\]
Defines ECC size 0 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eccsize0(&mut self) -> Eccsize0W<CfgGpmcEccSizeConfigSpec> {
        Eccsize0W::new(self, 12)
    }
    #[doc = "Bits 22:29 - 29:22\\]
Defines ECC size 1 \\[0x00 corresponds to 2 Bytes, 0x01 corresponds to 4 Bytes, 0x02 corresponds to 6 Bytes, 0x03 corresponds to 8 Bytes, &amp;, 0xFF corresponds to 512 Bytes\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eccsize1(&mut self) -> Eccsize1W<CfgGpmcEccSizeConfigSpec> {
        Eccsize1W::new(self, 22)
    }
}
#[doc = "ECC size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_size_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_size_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcEccSizeConfigSpec;
impl crate::RegisterSpec for CfgGpmcEccSizeConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_ecc_size_config::R`](R) reader structure"]
impl crate::Readable for CfgGpmcEccSizeConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_ecc_size_config::W`](W) writer structure"]
impl crate::Writable for CfgGpmcEccSizeConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_ECC_SIZE_CONFIG to value 0xd575_5000"]
impl crate::Resettable for CfgGpmcEccSizeConfigSpec {
    const RESET_VALUE: u32 = 0xd575_5000;
}
