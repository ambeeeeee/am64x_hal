#[doc = "Register `CFG_GPMC_CONFIG7` reader"]
pub type R = crate::R<CfgGpmcConfig7Spec>;
#[doc = "Register `CFG_GPMC_CONFIG7` writer"]
pub type W = crate::W<CfgGpmcConfig7Spec>;
#[doc = "Field `BASEADDRESS` reader - 5:0\\]
Chip-select base address"]
pub type BaseaddressR = crate::FieldReader;
#[doc = "Field `BASEADDRESS` writer - 5:0\\]
Chip-select base address"]
pub type BaseaddressW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CSVALID` reader - 6:6\\]
Chip-select enable \\[reset value is 1 for CS0 and 0 for CS1-7\\]"]
pub type CsvalidR = crate::BitReader;
#[doc = "Field `CSVALID` writer - 6:6\\]
Chip-select enable \\[reset value is 1 for CS0 and 0 for CS1-7\\]"]
pub type CsvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASKADDRESS` reader - 11:8\\]
Chip-select mask address"]
pub type MaskaddressR = crate::FieldReader;
#[doc = "Field `MASKADDRESS` writer - 11:8\\]
Chip-select mask address"]
pub type MaskaddressW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Chip-select base address"]
    #[inline(always)]
    pub fn baseaddress(&self) -> BaseaddressR {
        BaseaddressR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Chip-select enable \\[reset value is 1 for CS0 and 0 for CS1-7\\]"]
    #[inline(always)]
    pub fn csvalid(&self) -> CsvalidR {
        CsvalidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Chip-select mask address"]
    #[inline(always)]
    pub fn maskaddress(&self) -> MaskaddressR {
        MaskaddressR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Chip-select base address"]
    #[inline(always)]
    #[must_use]
    pub fn baseaddress(&mut self) -> BaseaddressW<CfgGpmcConfig7Spec> {
        BaseaddressW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Chip-select enable \\[reset value is 1 for CS0 and 0 for CS1-7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn csvalid(&mut self) -> CsvalidW<CfgGpmcConfig7Spec> {
        CsvalidW::new(self, 6)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Chip-select mask address"]
    #[inline(always)]
    #[must_use]
    pub fn maskaddress(&mut self) -> MaskaddressW<CfgGpmcConfig7Spec> {
        MaskaddressW::new(self, 8)
    }
}
#[doc = "Chip-select address mapping configuration Note: For CS0, the register reset is 0xf40 while for all the other instances CS1-CS7, the reset is 0xf00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfig7Spec;
impl crate::RegisterSpec for CfgGpmcConfig7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config7::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfig7Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config7::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfig7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG7 to value 0x1540"]
impl crate::Resettable for CfgGpmcConfig7Spec {
    const RESET_VALUE: u32 = 0x1540;
}
