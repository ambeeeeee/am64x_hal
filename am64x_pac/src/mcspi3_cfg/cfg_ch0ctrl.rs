#[doc = "Register `CFG_CH0CTRL` reader"]
pub type R = crate::R<CfgCh0ctrlSpec>;
#[doc = "Register `CFG_CH0CTRL` writer"]
pub type W = crate::W<CfgCh0ctrlSpec>;
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
    pub fn en(&mut self) -> EnW<CfgCh0ctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock ratio extension: This register is used to concatenate with MCSPI_CHCONF\\[CLKD\\]
register for clock ratio only when granularity is one clock cycle \\[MCSPI_CHCONF\\[CLKG\\]
set to 1\\]
Then the max value reached is 4096 clock divider ratio"]
    #[inline(always)]
    #[must_use]
    pub fn extclk(&mut self) -> ExtclkW<CfgCh0ctrlSpec> {
        ExtclkW::new(self, 8)
    }
}
#[doc = "This register is dedicated to enable the channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch0ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch0ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCh0ctrlSpec;
impl crate::RegisterSpec for CfgCh0ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_ch0ctrl::R`](R) reader structure"]
impl crate::Readable for CfgCh0ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_ch0ctrl::W`](W) writer structure"]
impl crate::Writable for CfgCh0ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CH0CTRL to value 0"]
impl crate::Resettable for CfgCh0ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
