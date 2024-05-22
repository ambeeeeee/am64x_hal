#[doc = "Register `CFG_SFT_RST` reader"]
pub type R = crate::R<CfgSftRstSpec>;
#[doc = "Register `CFG_SFT_RST` writer"]
pub type W = crate::W<CfgSftRstSpec>;
#[doc = "Field `KEY` reader - 3:0\\]
Global Soft Reset"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 3:0\\]
Global Soft Reset"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Global Soft Reset"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Global Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgSftRstSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "The Global Soft Reset Register controls the global clear for raw status and enables\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sft_rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sft_rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSftRstSpec;
impl crate::RegisterSpec for CfgSftRstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_sft_rst::R`](R) reader structure"]
impl crate::Readable for CfgSftRstSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_sft_rst::W`](W) writer structure"]
impl crate::Writable for CfgSftRstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_SFT_RST to value 0"]
impl crate::Resettable for CfgSftRstSpec {
    const RESET_VALUE: u32 = 0;
}
