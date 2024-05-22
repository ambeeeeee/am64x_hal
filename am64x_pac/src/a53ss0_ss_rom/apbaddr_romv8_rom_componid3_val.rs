#[doc = "Register `APBADDR_ROMV8_ROM_COMPONID3_VAL` reader"]
pub type R = crate::R<ApbaddrRomv8RomComponid3ValSpec>;
#[doc = "Register `APBADDR_ROMV8_ROM_COMPONID3_VAL` writer"]
pub type W = crate::W<ApbaddrRomv8RomComponid3ValSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ROM Component ID 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_componid3_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_componid3_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrRomv8RomComponid3ValSpec;
impl crate::RegisterSpec for ApbaddrRomv8RomComponid3ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_romv8_rom_componid3_val::R`](R) reader structure"]
impl crate::Readable for ApbaddrRomv8RomComponid3ValSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_romv8_rom_componid3_val::W`](W) writer structure"]
impl crate::Writable for ApbaddrRomv8RomComponid3ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ROMV8_ROM_COMPONID3_VAL to value 0"]
impl crate::Resettable for ApbaddrRomv8RomComponid3ValSpec {
    const RESET_VALUE: u32 = 0;
}