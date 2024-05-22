#[doc = "Register `APBADDR_ROMV8_ROM_PERIPHID5_VAL` reader"]
pub type R = crate::R<ApbaddrRomv8RomPeriphid5ValSpec>;
#[doc = "Register `APBADDR_ROMV8_ROM_PERIPHID5_VAL` writer"]
pub type W = crate::W<ApbaddrRomv8RomPeriphid5ValSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ROM Peripheral ID 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_rom_periphid5_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_rom_periphid5_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrRomv8RomPeriphid5ValSpec;
impl crate::RegisterSpec for ApbaddrRomv8RomPeriphid5ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_romv8_rom_periphid5_val::R`](R) reader structure"]
impl crate::Readable for ApbaddrRomv8RomPeriphid5ValSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_romv8_rom_periphid5_val::W`](W) writer structure"]
impl crate::Writable for ApbaddrRomv8RomPeriphid5ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ROMV8_ROM_PERIPHID5_VAL to value 0"]
impl crate::Resettable for ApbaddrRomv8RomPeriphid5ValSpec {
    const RESET_VALUE: u32 = 0;
}
