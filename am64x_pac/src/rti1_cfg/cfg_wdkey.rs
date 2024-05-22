#[doc = "Register `CFG_WDKEY` reader"]
pub type R = crate::R<CfgWdkeySpec>;
#[doc = "Register `CFG_WDKEY` writer"]
pub type W = crate::W<CfgWdkeySpec>;
#[doc = "Field `WDKEY` reader - 15:0\\]
User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1's. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation. Table 2. Example of a WDKEY sequence Value written to Step WDKEY Result 1 0x0A35C No Action 2 0x0A35C No Action 3 0x0E51A WDKEY is enabled for reset by next 0x0A35C 4 0x0E51A WDKEY is enabled for reset by next 0x0A35C 5 0x0E51A WDKEY is enabled for reset by next 0x0A35C 6 0x0A35C Watchdog is reset 7 0x0A35C No Action 8 0x0E51A WDKEY is enabled for reset by next 0x0A35C 9 0x0A35C Watchdog is reset 10 0x0E51A WDKEY is enabled for reset by next 0x0A35C 11 0x02345 System reset; incorrect value written to WDKEY"]
pub type WdkeyR = crate::FieldReader<u16>;
#[doc = "Field `WDKEY` writer - 15:0\\]
User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1's. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation. Table 2. Example of a WDKEY sequence Value written to Step WDKEY Result 1 0x0A35C No Action 2 0x0A35C No Action 3 0x0E51A WDKEY is enabled for reset by next 0x0A35C 4 0x0E51A WDKEY is enabled for reset by next 0x0A35C 5 0x0E51A WDKEY is enabled for reset by next 0x0A35C 6 0x0A35C Watchdog is reset 7 0x0A35C No Action 8 0x0E51A WDKEY is enabled for reset by next 0x0A35C 9 0x0A35C Watchdog is reset 10 0x0E51A WDKEY is enabled for reset by next 0x0A35C 11 0x02345 System reset; incorrect value written to WDKEY"]
pub type WdkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1's. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation. Table 2. Example of a WDKEY sequence Value written to Step WDKEY Result 1 0x0A35C No Action 2 0x0A35C No Action 3 0x0E51A WDKEY is enabled for reset by next 0x0A35C 4 0x0E51A WDKEY is enabled for reset by next 0x0A35C 5 0x0E51A WDKEY is enabled for reset by next 0x0A35C 6 0x0A35C Watchdog is reset 7 0x0A35C No Action 8 0x0E51A WDKEY is enabled for reset by next 0x0A35C 9 0x0A35C Watchdog is reset 10 0x0E51A WDKEY is enabled for reset by next 0x0A35C 11 0x02345 System reset; incorrect value written to WDKEY"]
    #[inline(always)]
    pub fn wdkey(&self) -> WdkeyR {
        WdkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
User and privilege mode reads are indeterminate. Privilege mode (write): A write of 0xE51A followed by 0xA35C in two separate write operations defines the Key Sequence and discharges the watchdog capacitor. This also causes the upper 12 bits of the DWD down counter to be reloaded with the contents of the DWD preload register and the lower 13 bits to become all 1's. Writing any other value causes a digital watchdog reset, as shown in Table 1-3. Note: Register write access time precaution The user has to take into account that the write to the register takes 3 VCLK cycle. This needs to be considered for the AWD/DWD expiration calculation. Table 2. Example of a WDKEY sequence Value written to Step WDKEY Result 1 0x0A35C No Action 2 0x0A35C No Action 3 0x0E51A WDKEY is enabled for reset by next 0x0A35C 4 0x0E51A WDKEY is enabled for reset by next 0x0A35C 5 0x0E51A WDKEY is enabled for reset by next 0x0A35C 6 0x0A35C Watchdog is reset 7 0x0A35C No Action 8 0x0E51A WDKEY is enabled for reset by next 0x0A35C 9 0x0A35C Watchdog is reset 10 0x0E51A WDKEY is enabled for reset by next 0x0A35C 11 0x02345 System reset; incorrect value written to WDKEY"]
    #[inline(always)]
    #[must_use]
    pub fn wdkey(&mut self) -> WdkeyW<CfgWdkeySpec> {
        WdkeyW::new(self, 0)
    }
}
#[doc = "CFG_WDKEY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wdkey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wdkey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgWdkeySpec;
impl crate::RegisterSpec for CfgWdkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_wdkey::R`](R) reader structure"]
impl crate::Readable for CfgWdkeySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_wdkey::W`](W) writer structure"]
impl crate::Writable for CfgWdkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_WDKEY to value 0x0004_1820"]
impl crate::Resettable for CfgWdkeySpec {
    const RESET_VALUE: u32 = 0x0004_1820;
}
