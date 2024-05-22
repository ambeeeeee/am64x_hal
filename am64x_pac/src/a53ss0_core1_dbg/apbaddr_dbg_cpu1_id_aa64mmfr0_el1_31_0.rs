#[doc = "Register `APBADDR_DBG_CPU1_ID_AA64MMFR0_EL1_31_0` reader"]
pub type R = crate::R<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_ID_AA64MMFR0_EL1_31_0` writer"]
pub type W = crate::W<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec>;
#[doc = "Field `PARANGE` reader - 3:0\\]
Physical Address range supported. Permitted values are: 0000 32 bits, 4 GB. 0001 36 bits, 64 GB. 0010 40 bits, 1 TB. 0011 42 bits, 4 TB. 0100 44 bits, 16 TB. 0101 48 bits, 256 TB. All other values are reserved."]
pub type ParangeR = crate::FieldReader;
#[doc = "Field `PARANGE` writer - 3:0\\]
Physical Address range supported. Permitted values are: 0000 32 bits, 4 GB. 0001 36 bits, 64 GB. 0010 40 bits, 1 TB. 0011 42 bits, 4 TB. 0100 44 bits, 16 TB. 0101 48 bits, 256 TB. All other values are reserved."]
pub type ParangeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASIDBITS` reader - 7:4\\]
Number of ASID bits. Permitted values are: 0000 8 bits. 0010 16 bits. All other values are reserved."]
pub type AsidbitsR = crate::FieldReader;
#[doc = "Field `ASIDBITS` writer - 7:4\\]
Number of ASID bits. Permitted values are: 0000 8 bits. 0010 16 bits. All other values are reserved."]
pub type AsidbitsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIGEND` reader - 11:8\\]
Mixed-endian configuration support. Permitted values are: 0000 No mixed-endian support. The SCTLR_ELx.EE bits have a fixed value. See the BigEndEL0 field, bits\\[19:16\\], for whether EL0 supports mixed-endian. 0001 Mixed-endian support. The SCTLR_ELx.EE and SCTLR_EL1.E0E bits can be configured. All other values are reserved."]
pub type BigendR = crate::FieldReader;
#[doc = "Field `BIGEND` writer - 11:8\\]
Mixed-endian configuration support. Permitted values are: 0000 No mixed-endian support. The SCTLR_ELx.EE bits have a fixed value. See the BigEndEL0 field, bits\\[19:16\\], for whether EL0 supports mixed-endian. 0001 Mixed-endian support. The SCTLR_ELx.EE and SCTLR_EL1.E0E bits can be configured. All other values are reserved."]
pub type BigendW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SNSMEM` reader - 15:12\\]
Secure versus Non-secure Memory distinction. Permitted values are: 0000 Does not support a distinction between Secure and Non-secure Memory. 0001 Does support a distinction between Secure and Non-secure Memory. All other values are reserved."]
pub type SnsmemR = crate::FieldReader;
#[doc = "Field `SNSMEM` writer - 15:12\\]
Secure versus Non-secure Memory distinction. Permitted values are: 0000 Does not support a distinction between Secure and Non-secure Memory. 0001 Does support a distinction between Secure and Non-secure Memory. All other values are reserved."]
pub type SnsmemW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIGENDEL0` reader - 19:16\\]
Mixed-endian support at EL0 only. Permitted values are: 0000 No mixed-endian support at EL0. The SCTLR_EL1.E0E bit has a fixed value. 0001 Mixed-endian support at EL0. The SCTLR_EL1.E0E bit can be configured. All other values are reserved.This field is invalid and is RES0 if the BigEnd field, bits \\[11:8\\], is not 0b0000."]
pub type Bigendel0R = crate::FieldReader;
#[doc = "Field `BIGENDEL0` writer - 19:16\\]
Mixed-endian support at EL0 only. Permitted values are: 0000 No mixed-endian support at EL0. The SCTLR_EL1.E0E bit has a fixed value. 0001 Mixed-endian support at EL0. The SCTLR_EL1.E0E bit can be configured. All other values are reserved.This field is invalid and is RES0 if the BigEnd field, bits \\[11:8\\], is not 0b0000."]
pub type Bigendel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TGRAN16` reader - 23:20\\]
Support for 16 Kbyte memory translation granule size. Permitted values are: 0000 16 KB granule not supported. 0001 16 KB granule supported. All other values are reserved."]
pub type Tgran16R = crate::FieldReader;
#[doc = "Field `TGRAN16` writer - 23:20\\]
Support for 16 Kbyte memory translation granule size. Permitted values are: 0000 16 KB granule not supported. 0001 16 KB granule supported. All other values are reserved."]
pub type Tgran16W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TGRAN64` reader - 27:24\\]
Support for 64 Kbyte memory translation granule size. Permitted values are: 0000 64 KB granule supported. 1111 64 KB granule not supported. All other values are reserved."]
pub type Tgran64R = crate::FieldReader;
#[doc = "Field `TGRAN64` writer - 27:24\\]
Support for 64 Kbyte memory translation granule size. Permitted values are: 0000 64 KB granule supported. 1111 64 KB granule not supported. All other values are reserved."]
pub type Tgran64W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TGRAN4` reader - 31:28\\]
Support for 4 Kbyte memory translation granule size. Permitted values are: 0000 4 KB granule supported. 1111 4 KB granule not supported. All other values are reserved."]
pub type Tgran4R = crate::FieldReader;
#[doc = "Field `TGRAN4` writer - 31:28\\]
Support for 4 Kbyte memory translation granule size. Permitted values are: 0000 4 KB granule supported. 1111 4 KB granule not supported. All other values are reserved."]
pub type Tgran4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Physical Address range supported. Permitted values are: 0000 32 bits, 4 GB. 0001 36 bits, 64 GB. 0010 40 bits, 1 TB. 0011 42 bits, 4 TB. 0100 44 bits, 16 TB. 0101 48 bits, 256 TB. All other values are reserved."]
    #[inline(always)]
    pub fn parange(&self) -> ParangeR {
        ParangeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of ASID bits. Permitted values are: 0000 8 bits. 0010 16 bits. All other values are reserved."]
    #[inline(always)]
    pub fn asidbits(&self) -> AsidbitsR {
        AsidbitsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Mixed-endian configuration support. Permitted values are: 0000 No mixed-endian support. The SCTLR_ELx.EE bits have a fixed value. See the BigEndEL0 field, bits\\[19:16\\], for whether EL0 supports mixed-endian. 0001 Mixed-endian support. The SCTLR_ELx.EE and SCTLR_EL1.E0E bits can be configured. All other values are reserved."]
    #[inline(always)]
    pub fn bigend(&self) -> BigendR {
        BigendR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Secure versus Non-secure Memory distinction. Permitted values are: 0000 Does not support a distinction between Secure and Non-secure Memory. 0001 Does support a distinction between Secure and Non-secure Memory. All other values are reserved."]
    #[inline(always)]
    pub fn snsmem(&self) -> SnsmemR {
        SnsmemR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Mixed-endian support at EL0 only. Permitted values are: 0000 No mixed-endian support at EL0. The SCTLR_EL1.E0E bit has a fixed value. 0001 Mixed-endian support at EL0. The SCTLR_EL1.E0E bit can be configured. All other values are reserved.This field is invalid and is RES0 if the BigEnd field, bits \\[11:8\\], is not 0b0000."]
    #[inline(always)]
    pub fn bigendel0(&self) -> Bigendel0R {
        Bigendel0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Support for 16 Kbyte memory translation granule size. Permitted values are: 0000 16 KB granule not supported. 0001 16 KB granule supported. All other values are reserved."]
    #[inline(always)]
    pub fn tgran16(&self) -> Tgran16R {
        Tgran16R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Support for 64 Kbyte memory translation granule size. Permitted values are: 0000 64 KB granule supported. 1111 64 KB granule not supported. All other values are reserved."]
    #[inline(always)]
    pub fn tgran64(&self) -> Tgran64R {
        Tgran64R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Support for 4 Kbyte memory translation granule size. Permitted values are: 0000 4 KB granule supported. 1111 4 KB granule not supported. All other values are reserved."]
    #[inline(always)]
    pub fn tgran4(&self) -> Tgran4R {
        Tgran4R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Physical Address range supported. Permitted values are: 0000 32 bits, 4 GB. 0001 36 bits, 64 GB. 0010 40 bits, 1 TB. 0011 42 bits, 4 TB. 0100 44 bits, 16 TB. 0101 48 bits, 256 TB. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn parange(&mut self) -> ParangeW<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        ParangeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of ASID bits. Permitted values are: 0000 8 bits. 0010 16 bits. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn asidbits(&mut self) -> AsidbitsW<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        AsidbitsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Mixed-endian configuration support. Permitted values are: 0000 No mixed-endian support. The SCTLR_ELx.EE bits have a fixed value. See the BigEndEL0 field, bits\\[19:16\\], for whether EL0 supports mixed-endian. 0001 Mixed-endian support. The SCTLR_ELx.EE and SCTLR_EL1.E0E bits can be configured. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bigend(&mut self) -> BigendW<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        BigendW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Secure versus Non-secure Memory distinction. Permitted values are: 0000 Does not support a distinction between Secure and Non-secure Memory. 0001 Does support a distinction between Secure and Non-secure Memory. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn snsmem(&mut self) -> SnsmemW<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        SnsmemW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Mixed-endian support at EL0 only. Permitted values are: 0000 No mixed-endian support at EL0. The SCTLR_EL1.E0E bit has a fixed value. 0001 Mixed-endian support at EL0. The SCTLR_EL1.E0E bit can be configured. All other values are reserved.This field is invalid and is RES0 if the BigEnd field, bits \\[11:8\\], is not 0b0000."]
    #[inline(always)]
    #[must_use]
    pub fn bigendel0(&mut self) -> Bigendel0W<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        Bigendel0W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Support for 16 Kbyte memory translation granule size. Permitted values are: 0000 16 KB granule not supported. 0001 16 KB granule supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tgran16(&mut self) -> Tgran16W<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        Tgran16W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Support for 64 Kbyte memory translation granule size. Permitted values are: 0000 64 KB granule supported. 1111 64 KB granule not supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tgran64(&mut self) -> Tgran64W<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        Tgran64W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Support for 4 Kbyte memory translation granule size. Permitted values are: 0000 4 KB granule supported. 1111 4 KB granule not supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tgran4(&mut self) -> Tgran4W<ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec> {
        Tgran4W::new(self, 28)
    }
}
#[doc = "Memory Model Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_ID_AA64MMFR0_EL1_31_0 to value 0x1122"]
impl crate::Resettable for ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec {
    const RESET_VALUE: u32 = 0x1122;
}
