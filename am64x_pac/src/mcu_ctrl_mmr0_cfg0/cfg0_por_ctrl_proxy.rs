#[doc = "Register `CFG0_POR_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0PorCtrlProxySpec>;
#[doc = "Register `CFG0_POR_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0PorCtrlProxySpec>;
#[doc = "Field `POR_CTRL_MASK_HHV_PROXY` reader - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
pub type PorCtrlMaskHhvProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_MASK_HHV_PROXY` writer - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
pub type PorCtrlMaskHhvProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_TRIM_SEL_PROXY` reader - 7:7\\]
POR Trim Select"]
pub type PorCtrlTrimSelProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_TRIM_SEL_PROXY` writer - 7:7\\]
POR Trim Select"]
pub type PorCtrlTrimSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD0_PROXY` reader - 16:16\\]
PORHV override active"]
pub type PorCtrlOvrd0ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD0_PROXY` writer - 16:16\\]
PORHV override active"]
pub type PorCtrlOvrd0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD1_PROXY` reader - 17:17\\]
BGOK override active"]
pub type PorCtrlOvrd1ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD1_PROXY` writer - 17:17\\]
BGOK override active"]
pub type PorCtrlOvrd1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD2_PROXY` reader - 18:18\\]
POKHV override active"]
pub type PorCtrlOvrd2ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD2_PROXY` writer - 18:18\\]
POKHV override active"]
pub type PorCtrlOvrd2ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD3_PROXY` reader - 19:19\\]
POKLVA override active"]
pub type PorCtrlOvrd3ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD3_PROXY` writer - 19:19\\]
POKLVA override active"]
pub type PorCtrlOvrd3ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD4_PROXY` reader - 20:20\\]
POKLVB override active"]
pub type PorCtrlOvrd4ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD4_PROXY` writer - 20:20\\]
POKLVB override active"]
pub type PorCtrlOvrd4ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD5_PROXY` reader - 21:21\\]
Reserved override active"]
pub type PorCtrlOvrd5ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD5_PROXY` writer - 21:21\\]
Reserved override active"]
pub type PorCtrlOvrd5ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET0_PROXY` reader - 24:24\\]
PORHV override set"]
pub type PorCtrlOvrdSet0ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET0_PROXY` writer - 24:24\\]
PORHV override set"]
pub type PorCtrlOvrdSet0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET1_PROXY` reader - 25:25\\]
BGOK override set"]
pub type PorCtrlOvrdSet1ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET1_PROXY` writer - 25:25\\]
BGOK override set"]
pub type PorCtrlOvrdSet1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET2_PROXY` reader - 26:26\\]
POKHV override set"]
pub type PorCtrlOvrdSet2ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET2_PROXY` writer - 26:26\\]
POKHV override set"]
pub type PorCtrlOvrdSet2ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET3_PROXY` reader - 27:27\\]
POKLVA override set"]
pub type PorCtrlOvrdSet3ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET3_PROXY` writer - 27:27\\]
POKLVA override set"]
pub type PorCtrlOvrdSet3ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET4_PROXY` reader - 28:28\\]
POKLVB override set"]
pub type PorCtrlOvrdSet4ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET4_PROXY` writer - 28:28\\]
POKLVB override set"]
pub type PorCtrlOvrdSet4ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR_CTRL_OVRD_SET5_PROXY` reader - 29:29\\]
Reserved override set"]
pub type PorCtrlOvrdSet5ProxyR = crate::BitReader;
#[doc = "Field `POR_CTRL_OVRD_SET5_PROXY` writer - 29:29\\]
Reserved override set"]
pub type PorCtrlOvrdSet5ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
    #[inline(always)]
    pub fn por_ctrl_mask_hhv_proxy(&self) -> PorCtrlMaskHhvProxyR {
        PorCtrlMaskHhvProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
POR Trim Select"]
    #[inline(always)]
    pub fn por_ctrl_trim_sel_proxy(&self) -> PorCtrlTrimSelProxyR {
        PorCtrlTrimSelProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
PORHV override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd0_proxy(&self) -> PorCtrlOvrd0ProxyR {
        PorCtrlOvrd0ProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
BGOK override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd1_proxy(&self) -> PorCtrlOvrd1ProxyR {
        PorCtrlOvrd1ProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
POKHV override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd2_proxy(&self) -> PorCtrlOvrd2ProxyR {
        PorCtrlOvrd2ProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
POKLVA override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd3_proxy(&self) -> PorCtrlOvrd3ProxyR {
        PorCtrlOvrd3ProxyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
POKLVB override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd4_proxy(&self) -> PorCtrlOvrd4ProxyR {
        PorCtrlOvrd4ProxyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved override active"]
    #[inline(always)]
    pub fn por_ctrl_ovrd5_proxy(&self) -> PorCtrlOvrd5ProxyR {
        PorCtrlOvrd5ProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
PORHV override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set0_proxy(&self) -> PorCtrlOvrdSet0ProxyR {
        PorCtrlOvrdSet0ProxyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
BGOK override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set1_proxy(&self) -> PorCtrlOvrdSet1ProxyR {
        PorCtrlOvrdSet1ProxyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
POKHV override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set2_proxy(&self) -> PorCtrlOvrdSet2ProxyR {
        PorCtrlOvrdSet2ProxyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
POKLVA override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set3_proxy(&self) -> PorCtrlOvrdSet3ProxyR {
        PorCtrlOvrdSet3ProxyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
POKLVB override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set4_proxy(&self) -> PorCtrlOvrdSet4ProxyR {
        PorCtrlOvrdSet4ProxyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Reserved override set"]
    #[inline(always)]
    pub fn por_ctrl_ovrd_set5_proxy(&self) -> PorCtrlOvrdSet5ProxyR {
        PorCtrlOvrdSet5ProxyR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Mask HHV/SOC_PORz outputs when applying new trim values"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_mask_hhv_proxy(&mut self) -> PorCtrlMaskHhvProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlMaskHhvProxyW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
POR Trim Select"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_trim_sel_proxy(&mut self) -> PorCtrlTrimSelProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlTrimSelProxyW::new(self, 7)
    }
    #[doc = "Bit 16 - 16:16\\]
PORHV override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd0_proxy(&mut self) -> PorCtrlOvrd0ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrd0ProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
BGOK override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd1_proxy(&mut self) -> PorCtrlOvrd1ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrd1ProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
POKHV override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd2_proxy(&mut self) -> PorCtrlOvrd2ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrd2ProxyW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
POKLVA override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd3_proxy(&mut self) -> PorCtrlOvrd3ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrd3ProxyW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
POKLVB override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd4_proxy(&mut self) -> PorCtrlOvrd4ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrd4ProxyW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved override active"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd5_proxy(&mut self) -> PorCtrlOvrd5ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrd5ProxyW::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
PORHV override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set0_proxy(&mut self) -> PorCtrlOvrdSet0ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrdSet0ProxyW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
BGOK override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set1_proxy(&mut self) -> PorCtrlOvrdSet1ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrdSet1ProxyW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
POKHV override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set2_proxy(&mut self) -> PorCtrlOvrdSet2ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrdSet2ProxyW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
POKLVA override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set3_proxy(&mut self) -> PorCtrlOvrdSet3ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrdSet3ProxyW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
POKLVB override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set4_proxy(&mut self) -> PorCtrlOvrdSet4ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrdSet4ProxyW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Reserved override set"]
    #[inline(always)]
    #[must_use]
    pub fn por_ctrl_ovrd_set5_proxy(&mut self) -> PorCtrlOvrdSet5ProxyW<Cfg0PorCtrlProxySpec> {
        PorCtrlOvrdSet5ProxyW::new(self, 29)
    }
}
#[doc = "CFG0_POR_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_por_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_por_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PorCtrlProxySpec;
impl crate::RegisterSpec for Cfg0PorCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_por_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PorCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_por_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PorCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_POR_CTRL_PROXY to value 0x10"]
impl crate::Resettable for Cfg0PorCtrlProxySpec {
    const RESET_VALUE: u32 = 0x10;
}
