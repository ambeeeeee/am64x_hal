#[doc = "Register `CFG0_RST_MAGIC_WORD` reader"]
pub type R = crate::R<Cfg0RstMagicWordSpec>;
#[doc = "Register `CFG0_RST_MAGIC_WORD` writer"]
pub type W = crate::W<Cfg0RstMagicWordSpec>;
#[doc = "Field `RST_MAGIC_WORD_MCU_MAGIC_WORD` reader - 31:0\\]
Magic Word Indicating Status of MCU Subsystem Boot"]
pub type RstMagicWordMcuMagicWordR = crate::FieldReader<u32>;
#[doc = "Field `RST_MAGIC_WORD_MCU_MAGIC_WORD` writer - 31:0\\]
Magic Word Indicating Status of MCU Subsystem Boot"]
pub type RstMagicWordMcuMagicWordW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Magic Word Indicating Status of MCU Subsystem Boot"]
    #[inline(always)]
    pub fn rst_magic_word_mcu_magic_word(&self) -> RstMagicWordMcuMagicWordR {
        RstMagicWordMcuMagicWordR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Magic Word Indicating Status of MCU Subsystem Boot"]
    #[inline(always)]
    #[must_use]
    pub fn rst_magic_word_mcu_magic_word(
        &mut self,
    ) -> RstMagicWordMcuMagicWordW<Cfg0RstMagicWordSpec> {
        RstMagicWordMcuMagicWordW::new(self, 0)
    }
}
#[doc = "CFG0_RST_MAGIC_WORD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_magic_word::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_magic_word::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstMagicWordSpec;
impl crate::RegisterSpec for Cfg0RstMagicWordSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_magic_word::R`](R) reader structure"]
impl crate::Readable for Cfg0RstMagicWordSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_magic_word::W`](W) writer structure"]
impl crate::Writable for Cfg0RstMagicWordSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_MAGIC_WORD to value 0"]
impl crate::Resettable for Cfg0RstMagicWordSpec {
    const RESET_VALUE: u32 = 0;
}
