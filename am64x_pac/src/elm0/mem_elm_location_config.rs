#[doc = "Register `MEM_ELM_LOCATION_CONFIG` reader"]
pub type R = crate::R<MemElmLocationConfigSpec>;
#[doc = "Register `MEM_ELM_LOCATION_CONFIG` writer"]
pub type W = crate::W<MemElmLocationConfigSpec>;
#[doc = "Field `ECC_BCH_LEVEL` reader - 1:0\\]
Error correction level 0x0: 4 bits 0x1: 8 bits 0x2: 16 bits 0x3: reserved"]
pub type EccBchLevelR = crate::FieldReader;
#[doc = "Field `ECC_BCH_LEVEL` writer - 1:0\\]
Error correction level 0x0: 4 bits 0x1: 8 bits 0x2: 16 bits 0x3: reserved"]
pub type EccBchLevelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECC_SIZE` reader - 26:16\\]
Maximum size of the buffers for which the error location engine is used, in number of nibbles \\[4-bits entities\\]"]
pub type EccSizeR = crate::FieldReader<u16>;
#[doc = "Field `ECC_SIZE` writer - 26:16\\]
Maximum size of the buffers for which the error location engine is used, in number of nibbles \\[4-bits entities\\]"]
pub type EccSizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Error correction level 0x0: 4 bits 0x1: 8 bits 0x2: 16 bits 0x3: reserved"]
    #[inline(always)]
    pub fn ecc_bch_level(&self) -> EccBchLevelR {
        EccBchLevelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Maximum size of the buffers for which the error location engine is used, in number of nibbles \\[4-bits entities\\]"]
    #[inline(always)]
    pub fn ecc_size(&self) -> EccSizeR {
        EccSizeR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Error correction level 0x0: 4 bits 0x1: 8 bits 0x2: 16 bits 0x3: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_bch_level(&mut self) -> EccBchLevelW<MemElmLocationConfigSpec> {
        EccBchLevelW::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Maximum size of the buffers for which the error location engine is used, in number of nibbles \\[4-bits entities\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_size(&mut self) -> EccSizeW<MemElmLocationConfigSpec> {
        EccSizeW::new(self, 16)
    }
}
#[doc = "ECC algorithm parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_location_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_location_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmLocationConfigSpec;
impl crate::RegisterSpec for MemElmLocationConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_location_config::R`](R) reader structure"]
impl crate::Readable for MemElmLocationConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_location_config::W`](W) writer structure"]
impl crate::Writable for MemElmLocationConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_LOCATION_CONFIG to value 0"]
impl crate::Resettable for MemElmLocationConfigSpec {
    const RESET_VALUE: u32 = 0;
}
