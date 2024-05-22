#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_argument1_lo` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_argument1_lo` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec>;
#[doc = "Field `CMD_ARG1` reader - 15:0\\]
The SD Command Argument is specified as bit23-8 of Command-Format."]
pub type CmdArg1R = crate::FieldReader<u16>;
#[doc = "Field `CMD_ARG1` writer - 15:0\\]
The SD Command Argument is specified as bit23-8 of Command-Format."]
pub type CmdArg1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The SD Command Argument is specified as bit23-8 of Command-Format."]
    #[inline(always)]
    pub fn cmd_arg1(&self) -> CmdArg1R {
        CmdArg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The SD Command Argument is specified as bit23-8 of Command-Format."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_arg1(&mut self) -> CmdArg1W<SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec> {
        CmdArg1W::new(self, 0)
    }
}
#[doc = "This register contains Lower bits of SD Command Argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_argument1_lo::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_argument1_lo to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgArgument1LoSpec {
    const RESET_VALUE: u16 = 0;
}
