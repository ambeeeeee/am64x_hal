#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_AERR_LOG2_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aAerrLog2RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_AERR_LOG2_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aAerrLog2RegSpec>;
#[doc = "Field `AERR_ADDR_MSB` reader - 31:0\\]
Address\\[34:16\\]
of the VBUSM.C command"]
pub type AerrAddrMsbR = crate::FieldReader<u32>;
#[doc = "Field `AERR_ADDR_MSB` writer - 31:0\\]
Address\\[34:16\\]
of the VBUSM.C command"]
pub type AerrAddrMsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Address\\[34:16\\]
of the VBUSM.C command"]
    #[inline(always)]
    pub fn aerr_addr_msb(&self) -> AerrAddrMsbR {
        AerrAddrMsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Address\\[34:16\\]
of the VBUSM.C command"]
    #[inline(always)]
    #[must_use]
    pub fn aerr_addr_msb(&mut self) -> AerrAddrMsbW<Regs_SsCfg_SscfgV2aAerrLog2RegSpec> {
        AerrAddrMsbW::new(self, 0)
    }
}
#[doc = "The Address Error Log 2 registers displays the msb of the address for the first VBUSM.C command that was outside the programmed addressing range. This register will be cleared upon writing the Address Error Log 1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_aerr_log2_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_aerr_log2_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aAerrLog2RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aAerrLog2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_aerr_log2_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aAerrLog2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_aerr_log2_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aAerrLog2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_AERR_LOG2_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aAerrLog2RegSpec {
    const RESET_VALUE: u32 = 0;
}
