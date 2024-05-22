#[doc = "Register `CFG0_RST_MAGIC_WORD_PROXY` reader"]
pub type R = crate::R<Cfg0RstMagicWordProxySpec>;
#[doc = "Register `CFG0_RST_MAGIC_WORD_PROXY` writer"]
pub type W = crate::W<Cfg0RstMagicWordProxySpec>;
#[doc = "Field `RST_MAGIC_WORD_MCU_MAGIC_WORD_PROXY` reader - 31:0\\]
After a MCU_PORz reset this bit field resets to 0x00000000."]
pub type RstMagicWordMcuMagicWordProxyR = crate::FieldReader<u32>;
#[doc = "Field `RST_MAGIC_WORD_MCU_MAGIC_WORD_PROXY` writer - 31:0\\]
After a MCU_PORz reset this bit field resets to 0x00000000."]
pub type RstMagicWordMcuMagicWordProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
After a MCU_PORz reset this bit field resets to 0x00000000."]
    #[inline(always)]
    pub fn rst_magic_word_mcu_magic_word_proxy(&self) -> RstMagicWordMcuMagicWordProxyR {
        RstMagicWordMcuMagicWordProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
After a MCU_PORz reset this bit field resets to 0x00000000."]
    #[inline(always)]
    #[must_use]
    pub fn rst_magic_word_mcu_magic_word_proxy(
        &mut self,
    ) -> RstMagicWordMcuMagicWordProxyW<Cfg0RstMagicWordProxySpec> {
        RstMagicWordMcuMagicWordProxyW::new(self, 0)
    }
}
#[doc = "CFG0_RST_MAGIC_WORD_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_magic_word_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_magic_word_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstMagicWordProxySpec;
impl crate::RegisterSpec for Cfg0RstMagicWordProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_magic_word_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0RstMagicWordProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_magic_word_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0RstMagicWordProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_MAGIC_WORD_PROXY to value 0"]
impl crate::Resettable for Cfg0RstMagicWordProxySpec {
    const RESET_VALUE: u32 = 0;
}
