#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsToccSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsToccSpec>;
#[doc = "Field `ETOC` reader - 0:0\\]
Enable Timeout Counter"]
pub type EtocR = crate::BitReader;
#[doc = "Field `ETOC` writer - 0:0\\]
Enable Timeout Counter"]
pub type EtocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOS` reader - 2:1\\]
Timeout Select"]
pub type TosR = crate::FieldReader;
#[doc = "Field `TOS` writer - 2:1\\]
Timeout Select"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOP` reader - 31:16\\]
Timeout Period"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - 31:16\\]
Timeout Period"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> EtocR {
        EtocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Timeout Counter"]
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> EtocW<McanWrap_McanCfgVbp_McanRegsToccSpec> {
        EtocW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Timeout Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TosW<McanWrap_McanCfgVbp_McanRegsToccSpec> {
        TosW::new(self, 1)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Timeout Period"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TopW<McanWrap_McanCfgVbp_McanRegsToccSpec> {
        TopW::new(self, 16)
    }
}
#[doc = "Configuration of timeout period, selection of timeout counter operation mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsToccSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsToccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsToccSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tocc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsToccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TOCC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsToccSpec {
    const RESET_VALUE: u32 = 0;
}
