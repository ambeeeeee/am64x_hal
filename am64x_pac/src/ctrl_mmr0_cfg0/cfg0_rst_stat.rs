#[doc = "Register `CFG0_RST_STAT` reader"]
pub type R = crate::R<Cfg0RstStatSpec>;
#[doc = "Register `CFG0_RST_STAT` writer"]
pub type W = crate::W<Cfg0RstStatSpec>;
#[doc = "Field `RST_STAT_MCU_RESET_ISO_DONE_Z` reader - 0:0\\]
is an outstanding warm reset request for the main domain. Once notified, MCU needs to finish any outstanding"]
pub type RstStatMcuResetIsoDoneZR = crate::BitReader;
#[doc = "Field `RST_STAT_MCU_RESET_ISO_DONE_Z` writer - 0:0\\]
is an outstanding warm reset request for the main domain. Once notified, MCU needs to finish any outstanding"]
pub type RstStatMcuResetIsoDoneZW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
is an outstanding warm reset request for the main domain. Once notified, MCU needs to finish any outstanding"]
    #[inline(always)]
    pub fn rst_stat_mcu_reset_iso_done_z(&self) -> RstStatMcuResetIsoDoneZR {
        RstStatMcuResetIsoDoneZR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
is an outstanding warm reset request for the main domain. Once notified, MCU needs to finish any outstanding"]
    #[inline(always)]
    #[must_use]
    pub fn rst_stat_mcu_reset_iso_done_z(&mut self) -> RstStatMcuResetIsoDoneZW<Cfg0RstStatSpec> {
        RstStatMcuResetIsoDoneZW::new(self, 0)
    }
}
#[doc = "CFG0_RST_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstStatSpec;
impl crate::RegisterSpec for Cfg0RstStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0RstStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0RstStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_STAT to value 0"]
impl crate::Resettable for Cfg0RstStatSpec {
    const RESET_VALUE: u32 = 0;
}
