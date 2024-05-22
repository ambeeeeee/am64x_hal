#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_data_port` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgDataPortSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_data_port` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgDataPortSpec>;
#[doc = "Field `BUF_RD_DATA` reader - 31:0\\]
The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
pub type BufRdDataR = crate::FieldReader<u32>;
#[doc = "Field `BUF_RD_DATA` writer - 31:0\\]
The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
pub type BufRdDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
    #[inline(always)]
    pub fn buf_rd_data(&self) -> BufRdDataR {
        BufRdDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The Host Controller Buffer can be accessed through this 32-bit Data Port Register."]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_data(&mut self) -> BufRdDataW<SdhcWrap_CtlCfg_CtlcfgDataPortSpec> {
        BufRdDataW::new(self, 0)
    }
}
#[doc = "This register is used to access internal buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_data_port::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_data_port::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgDataPortSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgDataPortSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_data_port::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgDataPortSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_data_port::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgDataPortSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_data_port to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgDataPortSpec {
    const RESET_VALUE: u32 = 0;
}
