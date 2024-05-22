#[doc = "Register `CFG_STS` reader"]
pub type R = crate::R<CfgStsSpec>;
#[doc = "Register `CFG_STS` writer"]
pub type W = crate::W<CfgStsSpec>;
#[doc = "Field `MSK` reader - 31:0\\]
This is the masked status/clear for errors in Group A"]
pub type MskR = crate::FieldReader<u32>;
#[doc = "Field `MSK` writer - 31:0\\]
This is the masked status/clear for errors in Group A"]
pub type MskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status/clear for errors in Group A"]
    #[inline(always)]
    pub fn msk(&self) -> MskR {
        MskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the masked status/clear for errors in Group A"]
    #[inline(always)]
    #[must_use]
    pub fn msk(&mut self) -> MskW<CfgStsSpec> {
        MskW::new(self, 0)
    }
}
#[doc = "Error Enable and Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgStsSpec;
impl crate::RegisterSpec for CfgStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_sts::R`](R) reader structure"]
impl crate::Readable for CfgStsSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_sts::W`](W) writer structure"]
impl crate::Writable for CfgStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_STS to value 0"]
impl crate::Resettable for CfgStsSpec {
    const RESET_VALUE: u32 = 0;
}
