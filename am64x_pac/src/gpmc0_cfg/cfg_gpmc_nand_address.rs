#[doc = "Register `CFG_GPMC_NAND_ADDRESS` reader"]
pub type R = crate::R<CfgGpmcNandAddressSpec>;
#[doc = "Register `CFG_GPMC_NAND_ADDRESS` writer"]
pub type W = crate::W<CfgGpmcNandAddressSpec>;
#[doc = "Field `GPMC_NAND_ADDRESS_0` reader - "]
pub type GpmcNandAddress0R = crate::FieldReader<u32>;
#[doc = "Field `GPMC_NAND_ADDRESS_0` writer - "]
pub type GpmcNandAddress0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpmc_nand_address_0(&self) -> GpmcNandAddress0R {
        GpmcNandAddress0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn gpmc_nand_address_0(&mut self) -> GpmcNandAddress0W<CfgGpmcNandAddressSpec> {
        GpmcNandAddress0W::new(self, 0)
    }
}
#[doc = "This Register is not a true register, just a address location.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_nand_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_nand_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcNandAddressSpec;
impl crate::RegisterSpec for CfgGpmcNandAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_nand_address::R`](R) reader structure"]
impl crate::Readable for CfgGpmcNandAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_nand_address::W`](W) writer structure"]
impl crate::Writable for CfgGpmcNandAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_NAND_ADDRESS to value 0"]
impl crate::Resettable for CfgGpmcNandAddressSpec {
    const RESET_VALUE: u32 = 0;
}
