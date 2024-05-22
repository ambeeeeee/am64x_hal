#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCC` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsTsccSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCC` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsTsccSpec>;
#[doc = "Field `TSS` reader - 1:0\\]
Timestamp Select"]
pub type TssR = crate::FieldReader;
#[doc = "Field `TSS` writer - 1:0\\]
Timestamp Select"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCP` reader - 19:16\\]
Timestamp Counter Prescaler"]
pub type TcpR = crate::FieldReader;
#[doc = "Field `TCP` writer - 19:16\\]
Timestamp Counter Prescaler"]
pub type TcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TcpR {
        TcpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Timestamp Select"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TssW<McanWrap_McanCfgVbp_McanRegsTsccSpec> {
        TssW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Timestamp Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TcpW<McanWrap_McanCfgVbp_McanRegsTsccSpec> {
        TcpW::new(self, 16)
    }
}
#[doc = "Timestamp counter prescaler setting, selection of internal/external timestamp vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsTsccSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsTsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsTsccSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_tscc::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsTsccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_TSCC to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsTsccSpec {
    const RESET_VALUE: u32 = 0;
}
