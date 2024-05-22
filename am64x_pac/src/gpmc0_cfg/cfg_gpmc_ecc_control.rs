#[doc = "Register `CFG_GPMC_ECC_CONTROL` reader"]
pub type R = crate::R<CfgGpmcEccControlSpec>;
#[doc = "Register `CFG_GPMC_ECC_CONTROL` writer"]
pub type W = crate::W<CfgGpmcEccControlSpec>;
#[doc = "Field `ECCPOINTER` reader - 3:0\\]
Selects ECC result register \\[Reads to this field give the dynamic position of the ECC pointer - Writes to this field select the ECC result register where the first ECC computation will be stored\\]; Other enums: writing other values disables the ECC engine \\[ECCEnable bit of GPMC_ECC_CONFIG set to 0\\]"]
pub type EccpointerR = crate::FieldReader;
#[doc = "Field `ECCPOINTER` writer - 3:0\\]
Selects ECC result register \\[Reads to this field give the dynamic position of the ECC pointer - Writes to this field select the ECC result register where the first ECC computation will be stored\\]; Other enums: writing other values disables the ECC engine \\[ECCEnable bit of GPMC_ECC_CONFIG set to 0\\]"]
pub type EccpointerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCCLEAR` reader - 8:8\\]
Clear all ECC result registers \\[Reads returns 0 - Writes 1 to this field clear all ECC result registers - Writes 0 are ignored\\]"]
pub type EccclearR = crate::BitReader;
#[doc = "Field `ECCCLEAR` writer - 8:8\\]
Clear all ECC result registers \\[Reads returns 0 - Writes 1 to this field clear all ECC result registers - Writes 0 are ignored\\]"]
pub type EccclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selects ECC result register \\[Reads to this field give the dynamic position of the ECC pointer - Writes to this field select the ECC result register where the first ECC computation will be stored\\]; Other enums: writing other values disables the ECC engine \\[ECCEnable bit of GPMC_ECC_CONFIG set to 0\\]"]
    #[inline(always)]
    pub fn eccpointer(&self) -> EccpointerR {
        EccpointerR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear all ECC result registers \\[Reads returns 0 - Writes 1 to this field clear all ECC result registers - Writes 0 are ignored\\]"]
    #[inline(always)]
    pub fn eccclear(&self) -> EccclearR {
        EccclearR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selects ECC result register \\[Reads to this field give the dynamic position of the ECC pointer - Writes to this field select the ECC result register where the first ECC computation will be stored\\]; Other enums: writing other values disables the ECC engine \\[ECCEnable bit of GPMC_ECC_CONFIG set to 0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eccpointer(&mut self) -> EccpointerW<CfgGpmcEccControlSpec> {
        EccpointerW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear all ECC result registers \\[Reads returns 0 - Writes 1 to this field clear all ECC result registers - Writes 0 are ignored\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eccclear(&mut self) -> EccclearW<CfgGpmcEccControlSpec> {
        EccclearW::new(self, 8)
    }
}
#[doc = "ECC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcEccControlSpec;
impl crate::RegisterSpec for CfgGpmcEccControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_ecc_control::R`](R) reader structure"]
impl crate::Readable for CfgGpmcEccControlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_ecc_control::W`](W) writer structure"]
impl crate::Writable for CfgGpmcEccControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_ECC_CONTROL to value 0"]
impl crate::Resettable for CfgGpmcEccControlSpec {
    const RESET_VALUE: u32 = 0;
}
