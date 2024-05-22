#[doc = "Register `CFG_GPMC_REVISION` reader"]
pub type R = crate::R<CfgGpmcRevisionSpec>;
#[doc = "Register `CFG_GPMC_REVISION` writer"]
pub type W = crate::W<CfgGpmcRevisionSpec>;
#[doc = "Field `REV` reader - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0, 0x21 for 2.1"]
pub type RevR = crate::FieldReader;
#[doc = "Field `REV` writer - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0, 0x21 for 2.1"]
pub type RevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0, 0x21 for 2.1"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision \\[7:4\\]
Major revision \\[3:0\\]
Minor revision Examples: 0x10 for 1.0, 0x21 for 2.1"]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> RevW<CfgGpmcRevisionSpec> {
        RevW::new(self, 0)
    }
}
#[doc = "This register contains the IP revision code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_revision::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_revision::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcRevisionSpec;
impl crate::RegisterSpec for CfgGpmcRevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_revision::R`](R) reader structure"]
impl crate::Readable for CfgGpmcRevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_revision::W`](W) writer structure"]
impl crate::Writable for CfgGpmcRevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_REVISION to value 0x96"]
impl crate::Resettable for CfgGpmcRevisionSpec {
    const RESET_VALUE: u32 = 0x96;
}
