#[doc = "Register `CFG_CH2CTRL` reader"]
pub type R = crate::R<CfgCh2ctrlSpec>;
#[doc = "Register `CFG_CH2CTRL` writer"]
pub type W = crate::W<CfgCh2ctrlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Channel Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Channel Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTCLK` reader - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio"]
pub type ExtclkR = crate::FieldReader;
#[doc = "Field `EXTCLK` writer - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio"]
pub type ExtclkW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio"]
    #[inline(always)]
    pub fn extclk(&self) -> ExtclkR {
        ExtclkR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CfgCh2ctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio"]
    #[inline(always)]
    #[must_use]
    pub fn extclk(&mut self) -> ExtclkW<CfgCh2ctrlSpec> {
        ExtclkW::new(self, 8)
    }
}
#[doc = "This register is dedicated to enable the channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch2ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch2ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCh2ctrlSpec;
impl crate::RegisterSpec for CfgCh2ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_ch2ctrl::R`](R) reader structure"]
impl crate::Readable for CfgCh2ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_ch2ctrl::W`](W) writer structure"]
impl crate::Writable for CfgCh2ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CH2CTRL to value 0"]
impl crate::Resettable for CfgCh2ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
