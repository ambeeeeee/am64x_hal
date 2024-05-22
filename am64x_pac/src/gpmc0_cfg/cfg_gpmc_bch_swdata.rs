#[doc = "Register `CFG_GPMC_BCH_SWDATA` reader"]
pub type R = crate::R<CfgGpmcBchSwdataSpec>;
#[doc = "Register `CFG_GPMC_BCH_SWDATA` writer"]
pub type W = crate::W<CfgGpmcBchSwdataSpec>;
#[doc = "Field `BCH_DATA` reader - 15:0\\]
Data to be included in the BCH calculation. Only bits 0 to 7 are taken into account if the calculator is configured to use 8 bits data \\[ECC16B = 0\\]"]
pub type BchDataR = crate::FieldReader<u16>;
#[doc = "Field `BCH_DATA` writer - 15:0\\]
Data to be included in the BCH calculation. Only bits 0 to 7 are taken into account if the calculator is configured to use 8 bits data \\[ECC16B = 0\\]"]
pub type BchDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Data to be included in the BCH calculation. Only bits 0 to 7 are taken into account if the calculator is configured to use 8 bits data \\[ECC16B = 0\\]"]
    #[inline(always)]
    pub fn bch_data(&self) -> BchDataR {
        BchDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Data to be included in the BCH calculation. Only bits 0 to 7 are taken into account if the calculator is configured to use 8 bits data \\[ECC16B = 0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn bch_data(&mut self) -> BchDataW<CfgGpmcBchSwdataSpec> {
        BchDataW::new(self, 0)
    }
}
#[doc = "This register is used to directly pass data to the BCH ECC calculator without accessing the actual NAND flash interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_bch_swdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_bch_swdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcBchSwdataSpec;
impl crate::RegisterSpec for CfgGpmcBchSwdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_bch_swdata::R`](R) reader structure"]
impl crate::Readable for CfgGpmcBchSwdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_bch_swdata::W`](W) writer structure"]
impl crate::Writable for CfgGpmcBchSwdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_BCH_SWDATA to value 0"]
impl crate::Resettable for CfgGpmcBchSwdataSpec {
    const RESET_VALUE: u32 = 0;
}
