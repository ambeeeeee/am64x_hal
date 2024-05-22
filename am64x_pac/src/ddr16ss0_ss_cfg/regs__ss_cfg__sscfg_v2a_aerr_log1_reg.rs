#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_AERR_LOG1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aAerrLog1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_AERR_LOG1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aAerrLog1RegSpec>;
#[doc = "Field `AERR_ROUTE_ID` reader - 11:0\\]
RouteID of the VBUSM.C write command"]
pub type AerrRouteIdR = crate::FieldReader<u16>;
#[doc = "Field `AERR_ROUTE_ID` writer - 11:0\\]
RouteID of the VBUSM.C write command"]
pub type AerrRouteIdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AERR_ADDR_LSB` reader - 31:16\\]
Address\\[15:0\\]
of the VBUSM.C command"]
pub type AerrAddrLsbR = crate::FieldReader<u16>;
#[doc = "Field `AERR_ADDR_LSB` writer - 31:16\\]
Address\\[15:0\\]
of the VBUSM.C command"]
pub type AerrAddrLsbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
RouteID of the VBUSM.C write command"]
    #[inline(always)]
    pub fn aerr_route_id(&self) -> AerrRouteIdR {
        AerrRouteIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Address\\[15:0\\]
of the VBUSM.C command"]
    #[inline(always)]
    pub fn aerr_addr_lsb(&self) -> AerrAddrLsbR {
        AerrAddrLsbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
RouteID of the VBUSM.C write command"]
    #[inline(always)]
    #[must_use]
    pub fn aerr_route_id(&mut self) -> AerrRouteIdW<Regs_SsCfg_SscfgV2aAerrLog1RegSpec> {
        AerrRouteIdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Address\\[15:0\\]
of the VBUSM.C command"]
    #[inline(always)]
    #[must_use]
    pub fn aerr_addr_lsb(&mut self) -> AerrAddrLsbW<Regs_SsCfg_SscfgV2aAerrLog1RegSpec> {
        AerrAddrLsbW::new(self, 16)
    }
}
#[doc = "The Address Error Log 1 register displays the RouteID and lsb of the address for the first VBUSM.C command that was outside the programmed addressing range. Writing a 0x1 will clear all fields. Writing any other value has no effect. The Address Error Log 2 register will also be cleared upon writing this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_aerr_log1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_aerr_log1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aAerrLog1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aAerrLog1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_aerr_log1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aAerrLog1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_aerr_log1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aAerrLog1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_AERR_LOG1_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aAerrLog1RegSpec {
    const RESET_VALUE: u32 = 0;
}
