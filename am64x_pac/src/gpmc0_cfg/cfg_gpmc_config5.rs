#[doc = "Register `CFG_GPMC_CONFIG5` reader"]
pub type R = crate::R<CfgGpmcConfig5Spec>;
#[doc = "Register `CFG_GPMC_CONFIG5` writer"]
pub type W = crate::W<CfgGpmcConfig5Spec>;
#[doc = "Field `RDCYCLETIME` reader - 4:0\\]
Total read cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type RdcycletimeR = crate::FieldReader;
#[doc = "Field `RDCYCLETIME` writer - 4:0\\]
Total read cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type RdcycletimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRCYCLETIME` reader - 12:8\\]
Total write cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type WrcycletimeR = crate::FieldReader;
#[doc = "Field `WRCYCLETIME` writer - 12:8\\]
Total write cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type WrcycletimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RDACCESSTIME` reader - 20:16\\]
Delay between start cycle time and first data valid \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type RdaccesstimeR = crate::FieldReader;
#[doc = "Field `RDACCESSTIME` writer - 20:16\\]
Delay between start cycle time and first data valid \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type RdaccesstimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PAGEBURSTACCESSTIME` reader - 27:24\\]
Delay between successive words in a multiple access \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type PageburstaccesstimeR = crate::FieldReader;
#[doc = "Field `PAGEBURSTACCESSTIME` writer - 27:24\\]
Delay between successive words in a multiple access \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type PageburstaccesstimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Total read cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn rdcycletime(&self) -> RdcycletimeR {
        RdcycletimeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Total write cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn wrcycletime(&self) -> WrcycletimeR {
        WrcycletimeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Delay between start cycle time and first data valid \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn rdaccesstime(&self) -> RdaccesstimeR {
        RdaccesstimeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Delay between successive words in a multiple access \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn pageburstaccesstime(&self) -> PageburstaccesstimeR {
        PageburstaccesstimeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Total read cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rdcycletime(&mut self) -> RdcycletimeW<CfgGpmcConfig5Spec> {
        RdcycletimeW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Total write cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wrcycletime(&mut self) -> WrcycletimeW<CfgGpmcConfig5Spec> {
        WrcycletimeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Delay between start cycle time and first data valid \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rdaccesstime(&mut self) -> RdaccesstimeW<CfgGpmcConfig5Spec> {
        RdaccesstimeW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Delay between successive words in a multiple access \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn pageburstaccesstime(&mut self) -> PageburstaccesstimeW<CfgGpmcConfig5Spec> {
        PageburstaccesstimeW::new(self, 24)
    }
}
#[doc = "RdAccessTime and CycleTime timing parameters configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfig5Spec;
impl crate::RegisterSpec for CfgGpmcConfig5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config5::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfig5Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config5::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfig5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG5 to value 0x0115_1717"]
impl crate::Resettable for CfgGpmcConfig5Spec {
    const RESET_VALUE: u32 = 0x0115_1717;
}
