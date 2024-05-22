#[doc = "Register `CFG_GPMC_CONFIG2` reader"]
pub type R = crate::R<CfgGpmcConfig2Spec>;
#[doc = "Register `CFG_GPMC_CONFIG2` writer"]
pub type W = crate::W<CfgGpmcConfig2Spec>;
#[doc = "Field `CSONTIME` reader - 3:0\\]
CS# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type CsontimeR = crate::FieldReader;
#[doc = "Field `CSONTIME` writer - 3:0\\]
CS# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type CsontimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSEXTRADELAY` reader - 7:7\\]
CS# Add Extra Half GPMC.FCLK cycle"]
pub type CsextradelayR = crate::BitReader;
#[doc = "Field `CSEXTRADELAY` writer - 7:7\\]
CS# Add Extra Half GPMC.FCLK cycle"]
pub type CsextradelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRDOFFTIME` reader - 12:8\\]
CS# de-assertion time from start cycle time for read accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type CsrdofftimeR = crate::FieldReader;
#[doc = "Field `CSRDOFFTIME` writer - 12:8\\]
CS# de-assertion time from start cycle time for read accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type CsrdofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CSWROFFTIME` reader - 20:16\\]
CS# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type CswrofftimeR = crate::FieldReader;
#[doc = "Field `CSWROFFTIME` writer - 20:16\\]
CS# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type CswrofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
CS# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn csontime(&self) -> CsontimeR {
        CsontimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
CS# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    pub fn csextradelay(&self) -> CsextradelayR {
        CsextradelayR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
CS# de-assertion time from start cycle time for read accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn csrdofftime(&self) -> CsrdofftimeR {
        CsrdofftimeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
CS# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn cswrofftime(&self) -> CswrofftimeR {
        CswrofftimeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
CS# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn csontime(&mut self) -> CsontimeW<CfgGpmcConfig2Spec> {
        CsontimeW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
CS# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    #[must_use]
    pub fn csextradelay(&mut self) -> CsextradelayW<CfgGpmcConfig2Spec> {
        CsextradelayW::new(self, 7)
    }
    #[doc = "Bits 8:12 - 12:8\\]
CS# de-assertion time from start cycle time for read accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn csrdofftime(&mut self) -> CsrdofftimeW<CfgGpmcConfig2Spec> {
        CsrdofftimeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
CS# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cswrofftime(&mut self) -> CswrofftimeW<CfgGpmcConfig2Spec> {
        CswrofftimeW::new(self, 16)
    }
}
#[doc = "Chip-select signal timing parameter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfig2Spec;
impl crate::RegisterSpec for CfgGpmcConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config2::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config2::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfig2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG2 to value 0x0016_1601"]
impl crate::Resettable for CfgGpmcConfig2Spec {
    const RESET_VALUE: u32 = 0x0016_1601;
}
