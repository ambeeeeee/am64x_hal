#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ILE` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsIleSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ILE` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsIleSpec>;
#[doc = "Field `EINT0` reader - 0:0\\]
Enable Interrupt Line 0"]
pub type Eint0R = crate::BitReader;
#[doc = "Field `EINT0` writer - 0:0\\]
Enable Interrupt Line 0"]
pub type Eint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT1` reader - 1:1\\]
Enable Interrupt Line 1"]
pub type Eint1R = crate::BitReader;
#[doc = "Field `EINT1` writer - 1:1\\]
Enable Interrupt Line 1"]
pub type Eint1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> Eint0R {
        Eint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> Eint1R {
        Eint1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Interrupt Line 0"]
    #[inline(always)]
    #[must_use]
    pub fn eint0(&mut self) -> Eint0W<McanWrap_McanCfgVbp_McanRegsIleSpec> {
        Eint0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Interrupt Line 1"]
    #[inline(always)]
    #[must_use]
    pub fn eint1(&mut self) -> Eint1W<McanWrap_McanCfgVbp_McanRegsIleSpec> {
        Eint1W::new(self, 1)
    }
}
#[doc = "Enable/disable interrupt lines m_can_int0 / m_can_int1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ile::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ile::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsIleSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsIleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ile::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsIleSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ile::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsIleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_ILE to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsIleSpec {
    const RESET_VALUE: u32 = 0;
}
