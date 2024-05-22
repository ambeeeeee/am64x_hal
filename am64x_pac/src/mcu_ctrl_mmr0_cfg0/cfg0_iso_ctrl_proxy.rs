#[doc = "Register `CFG0_ISO_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0IsoCtrlProxySpec>;
#[doc = "Register `CFG0_ISO_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0IsoCtrlProxySpec>;
#[doc = "Field `ISO_CTRL_MCU_RST_ISO_EN_PROXY` reader - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
pub type IsoCtrlMcuRstIsoEnProxyR = crate::BitReader;
#[doc = "Field `ISO_CTRL_MCU_RST_ISO_EN_PROXY` writer - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
pub type IsoCtrlMcuRstIsoEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_CTRL_MCU_DBG_ISO_EN_PROXY` reader - 1:1\\]
Isolates the MCU domain from Debug"]
pub type IsoCtrlMcuDbgIsoEnProxyR = crate::BitReader;
#[doc = "Field `ISO_CTRL_MCU_DBG_ISO_EN_PROXY` writer - 1:1\\]
Isolates the MCU domain from Debug"]
pub type IsoCtrlMcuDbgIsoEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
    #[inline(always)]
    pub fn iso_ctrl_mcu_rst_iso_en_proxy(&self) -> IsoCtrlMcuRstIsoEnProxyR {
        IsoCtrlMcuRstIsoEnProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Isolates the MCU domain from Debug"]
    #[inline(always)]
    pub fn iso_ctrl_mcu_dbg_iso_en_proxy(&self) -> IsoCtrlMcuDbgIsoEnProxyR {
        IsoCtrlMcuDbgIsoEnProxyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Isolates the MCU domain from Warm Reset initiated by Main"]
    #[inline(always)]
    #[must_use]
    pub fn iso_ctrl_mcu_rst_iso_en_proxy(
        &mut self,
    ) -> IsoCtrlMcuRstIsoEnProxyW<Cfg0IsoCtrlProxySpec> {
        IsoCtrlMcuRstIsoEnProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Isolates the MCU domain from Debug"]
    #[inline(always)]
    #[must_use]
    pub fn iso_ctrl_mcu_dbg_iso_en_proxy(
        &mut self,
    ) -> IsoCtrlMcuDbgIsoEnProxyW<Cfg0IsoCtrlProxySpec> {
        IsoCtrlMcuDbgIsoEnProxyW::new(self, 1)
    }
}
#[doc = "CFG0_ISO_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_iso_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_iso_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IsoCtrlProxySpec;
impl crate::RegisterSpec for Cfg0IsoCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_iso_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0IsoCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_iso_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0IsoCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ISO_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0IsoCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
