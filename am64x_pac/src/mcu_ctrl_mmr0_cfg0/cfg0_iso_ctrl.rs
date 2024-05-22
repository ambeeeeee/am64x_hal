#[doc = "Register `CFG0_ISO_CTRL` reader"]
pub type R = crate::R<Cfg0IsoCtrlSpec>;
#[doc = "Register `CFG0_ISO_CTRL` writer"]
pub type W = crate::W<Cfg0IsoCtrlSpec>;
#[doc = "Field `ISO_CTRL_MCU_RST_ISO_EN` reader - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
pub type IsoCtrlMcuRstIsoEnR = crate::BitReader;
#[doc = "Field `ISO_CTRL_MCU_RST_ISO_EN` writer - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
pub type IsoCtrlMcuRstIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_CTRL_MCU_DBG_ISO_EN` reader - 1:1\\]
Isolates the MCU domain from Debug"]
pub type IsoCtrlMcuDbgIsoEnR = crate::BitReader;
#[doc = "Field `ISO_CTRL_MCU_DBG_ISO_EN` writer - 1:1\\]
Isolates the MCU domain from Debug"]
pub type IsoCtrlMcuDbgIsoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
    #[inline(always)]
    pub fn iso_ctrl_mcu_rst_iso_en(&self) -> IsoCtrlMcuRstIsoEnR {
        IsoCtrlMcuRstIsoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Isolates the MCU domain from Debug"]
    #[inline(always)]
    pub fn iso_ctrl_mcu_dbg_iso_en(&self) -> IsoCtrlMcuDbgIsoEnR {
        IsoCtrlMcuDbgIsoEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
    #[inline(always)]
    #[must_use]
    pub fn iso_ctrl_mcu_rst_iso_en(&mut self) -> IsoCtrlMcuRstIsoEnW<Cfg0IsoCtrlSpec> {
        IsoCtrlMcuRstIsoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Isolates the MCU domain from Debug"]
    #[inline(always)]
    #[must_use]
    pub fn iso_ctrl_mcu_dbg_iso_en(&mut self) -> IsoCtrlMcuDbgIsoEnW<Cfg0IsoCtrlSpec> {
        IsoCtrlMcuDbgIsoEnW::new(self, 1)
    }
}
#[doc = "CFG0_ISO_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_iso_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_iso_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IsoCtrlSpec;
impl crate::RegisterSpec for Cfg0IsoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_iso_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0IsoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_iso_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0IsoCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ISO_CTRL to value 0"]
impl crate::Resettable for Cfg0IsoCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
