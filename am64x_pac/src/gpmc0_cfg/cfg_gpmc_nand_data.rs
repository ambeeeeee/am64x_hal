#[doc = "Register `CFG_GPMC_NAND_DATA` reader"]
pub type R = crate::R<CfgGpmcNandDataSpec>;
#[doc = "Register `CFG_GPMC_NAND_DATA` writer"]
pub type W = crate::W<CfgGpmcNandDataSpec>;
#[doc = "Field `GPMC_NAND_DATA_0` reader - "]
pub type GpmcNandData0R = crate::FieldReader<u32>;
#[doc = "Field `GPMC_NAND_DATA_0` writer - "]
pub type GpmcNandData0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpmc_nand_data_0(&self) -> GpmcNandData0R {
        GpmcNandData0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn gpmc_nand_data_0(&mut self) -> GpmcNandData0W<CfgGpmcNandDataSpec> {
        GpmcNandData0W::new(self, 0)
    }
}
#[doc = "This Register is not a true register, just a address location.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_nand_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_nand_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcNandDataSpec;
impl crate::RegisterSpec for CfgGpmcNandDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_nand_data::R`](R) reader structure"]
impl crate::Readable for CfgGpmcNandDataSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_nand_data::W`](W) writer structure"]
impl crate::Writable for CfgGpmcNandDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_NAND_DATA to value 0"]
impl crate::Resettable for CfgGpmcNandDataSpec {
    const RESET_VALUE: u32 = 0;
}
