#[doc = "Register `CFG_GPMC_ECC_CONFIG` reader"]
pub type R = crate::R<CfgGpmcEccConfigSpec>;
#[doc = "Register `CFG_GPMC_ECC_CONFIG` writer"]
pub type W = crate::W<CfgGpmcEccConfigSpec>;
#[doc = "Field `ECCENABLE` reader - 0:0\\]
Enables the ECC feature"]
pub type EccenableR = crate::BitReader;
#[doc = "Field `ECCENABLE` writer - 0:0\\]
Enables the ECC feature"]
pub type EccenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCCS` reader - 3:1\\]
Selects the CS where ECC is computed"]
pub type EcccsR = crate::FieldReader;
#[doc = "Field `ECCCS` writer - 3:1\\]
Selects the CS where ECC is computed"]
pub type EcccsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ECCTOPSECTOR` reader - 6:4\\]
Number of sectors to process with the BCH algorithm 0x0: 1 sector \\[512kB page\\]
0x1: 2 sectors ... 0x3: 4 sectors \\[2kB page\\]
... 0x7: 8 sectors \\[4kB page\\]"]
pub type EcctopsectorR = crate::FieldReader;
#[doc = "Field `ECCTOPSECTOR` writer - 6:4\\]
Number of sectors to process with the BCH algorithm 0x0: 1 sector \\[512kB page\\]
0x1: 2 sectors ... 0x3: 4 sectors \\[2kB page\\]
... 0x7: 8 sectors \\[4kB page\\]"]
pub type EcctopsectorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ECC16B` reader - 7:7\\]
Selects an ECC calculated on 16 columns"]
pub type Ecc16bR = crate::BitReader;
#[doc = "Field `ECC16B` writer - 7:7\\]
Selects an ECC calculated on 16 columns"]
pub type Ecc16bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCWRAPMODE` reader - 11:8\\]
Spare area organization definition for the BCH algorithm. See the BCH syndrome/parity calculator module functional specification for more details"]
pub type EccwrapmodeR = crate::FieldReader;
#[doc = "Field `ECCWRAPMODE` writer - 11:8\\]
Spare area organization definition for the BCH algorithm. See the BCH syndrome/parity calculator module functional specification for more details"]
pub type EccwrapmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCBCHTSEL` reader - 13:12\\]
Error correction capability used for BCH 0x0: up to 4 bits error correction \\[t = 4\\]
0x1: up to 8 bits error correction \\[t=8\\]
0x2: up to 16 bits error correction \\[t=16\\]
0x3: reserved"]
pub type EccbchtselR = crate::FieldReader;
#[doc = "Field `ECCBCHTSEL` writer - 13:12\\]
Error correction capability used for BCH 0x0: up to 4 bits error correction \\[t = 4\\]
0x1: up to 8 bits error correction \\[t=8\\]
0x2: up to 16 bits error correction \\[t=16\\]
0x3: reserved"]
pub type EccbchtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCALGORITHM` reader - 16:16\\]
ECC algorithm used 0x0: Hamming code 0x1: BCH code"]
pub type EccalgorithmR = crate::BitReader;
#[doc = "Field `ECCALGORITHM` writer - 16:16\\]
ECC algorithm used 0x0: Hamming code 0x1: BCH code"]
pub type EccalgorithmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the ECC feature"]
    #[inline(always)]
    pub fn eccenable(&self) -> EccenableR {
        EccenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Selects the CS where ECC is computed"]
    #[inline(always)]
    pub fn ecccs(&self) -> EcccsR {
        EcccsR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Number of sectors to process with the BCH algorithm 0x0: 1 sector \\[512kB page\\]
0x1: 2 sectors ... 0x3: 4 sectors \\[2kB page\\]
... 0x7: 8 sectors \\[4kB page\\]"]
    #[inline(always)]
    pub fn ecctopsector(&self) -> EcctopsectorR {
        EcctopsectorR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects an ECC calculated on 16 columns"]
    #[inline(always)]
    pub fn ecc16b(&self) -> Ecc16bR {
        Ecc16bR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Spare area organization definition for the BCH algorithm. See the BCH syndrome/parity calculator module functional specification for more details"]
    #[inline(always)]
    pub fn eccwrapmode(&self) -> EccwrapmodeR {
        EccwrapmodeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Error correction capability used for BCH 0x0: up to 4 bits error correction \\[t = 4\\]
0x1: up to 8 bits error correction \\[t=8\\]
0x2: up to 16 bits error correction \\[t=16\\]
0x3: reserved"]
    #[inline(always)]
    pub fn eccbchtsel(&self) -> EccbchtselR {
        EccbchtselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
ECC algorithm used 0x0: Hamming code 0x1: BCH code"]
    #[inline(always)]
    pub fn eccalgorithm(&self) -> EccalgorithmR {
        EccalgorithmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the ECC feature"]
    #[inline(always)]
    #[must_use]
    pub fn eccenable(&mut self) -> EccenableW<CfgGpmcEccConfigSpec> {
        EccenableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Selects the CS where ECC is computed"]
    #[inline(always)]
    #[must_use]
    pub fn ecccs(&mut self) -> EcccsW<CfgGpmcEccConfigSpec> {
        EcccsW::new(self, 1)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Number of sectors to process with the BCH algorithm 0x0: 1 sector \\[512kB page\\]
0x1: 2 sectors ... 0x3: 4 sectors \\[2kB page\\]
... 0x7: 8 sectors \\[4kB page\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ecctopsector(&mut self) -> EcctopsectorW<CfgGpmcEccConfigSpec> {
        EcctopsectorW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
Selects an ECC calculated on 16 columns"]
    #[inline(always)]
    #[must_use]
    pub fn ecc16b(&mut self) -> Ecc16bW<CfgGpmcEccConfigSpec> {
        Ecc16bW::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Spare area organization definition for the BCH algorithm. See the BCH syndrome/parity calculator module functional specification for more details"]
    #[inline(always)]
    #[must_use]
    pub fn eccwrapmode(&mut self) -> EccwrapmodeW<CfgGpmcEccConfigSpec> {
        EccwrapmodeW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Error correction capability used for BCH 0x0: up to 4 bits error correction \\[t = 4\\]
0x1: up to 8 bits error correction \\[t=8\\]
0x2: up to 16 bits error correction \\[t=16\\]
0x3: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn eccbchtsel(&mut self) -> EccbchtselW<CfgGpmcEccConfigSpec> {
        EccbchtselW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
ECC algorithm used 0x0: Hamming code 0x1: BCH code"]
    #[inline(always)]
    #[must_use]
    pub fn eccalgorithm(&mut self) -> EccalgorithmW<CfgGpmcEccConfigSpec> {
        EccalgorithmW::new(self, 16)
    }
}
#[doc = "ECC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_ecc_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_ecc_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcEccConfigSpec;
impl crate::RegisterSpec for CfgGpmcEccConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_ecc_config::R`](R) reader structure"]
impl crate::Readable for CfgGpmcEccConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_ecc_config::W`](W) writer structure"]
impl crate::Writable for CfgGpmcEccConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_ECC_CONFIG to value 0x1030"]
impl crate::Resettable for CfgGpmcEccConfigSpec {
    const RESET_VALUE: u32 = 0x1030;
}
