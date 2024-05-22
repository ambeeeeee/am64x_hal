#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fsas__fsas_otfa_cfg__fsas_otfa_regs_revid: Fsas_FsasOtfaCfg_FsasOtfaRegsRevid,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg: Fsas_FsasOtfaCfg_FsasOtfaRegsScfg,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_isr: Fsas_FsasOtfaCfg_FsasOtfaRegsIsr,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_is: Fsas_FsasOtfaCfg_FsasOtfaRegsIs,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_ies: Fsas_FsasOtfaCfg_FsasOtfaRegsIes,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_iec: Fsas_FsasOtfaCfg_FsasOtfaRegsIec,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg: Fsas_FsasOtfaCfg_FsasOtfaRegsCcfg,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus: Fsas_FsasOtfaCfg_FsasOtfaRegsCstatus,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0: Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0: Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst0,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0: Fsas_FsasOtfaCfg_FsasOtfaRegsRgst0,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0: Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi0,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye00,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye01,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye02,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye03,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye04,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye05,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye06,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye07,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep00,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep01,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep02,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep03,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep04,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep05,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep06,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep07,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya00,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya01,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya02,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya03,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap00,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap02,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap03,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv00,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv02,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv03,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1: Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1: Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst1,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1: Fsas_FsasOtfaCfg_FsasOtfaRegsRgst1,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1: Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi1,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye10,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye11,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye12,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye14,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye15,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye16,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye17,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep10,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep11,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep12,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep13,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep14,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep15,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep16,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep17,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya10,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya11,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya12,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya13,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap10,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap11,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap12,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap13,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv10,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv11,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv12,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv13,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2: Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg2,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2: Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst2,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2: Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2: Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi2,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye20,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye21,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye22,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye23,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye24,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye25,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye26,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye27,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep20,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep21,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep22,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep23,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep24,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep25,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep26,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep27,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya20,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya21,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya22,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya23,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap20,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap21,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap22,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap23,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv20,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv21,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv22,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3: Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3: Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst3,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3: Fsas_FsasOtfaCfg_FsasOtfaRegsRgst3,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3: Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi3,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye30,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye31,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye32,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye33,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye34,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye35,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye36,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye37,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep30,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep31,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep32,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep33,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep34,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep35,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep36,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep37,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya30,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya31,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya32,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya33,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap30,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap31,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap32,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33: Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap33,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv30,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv31,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv32,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33: Fsas_FsasOtfaCfg_FsasOtfaRegsRiv33,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0: Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1: Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo: Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfo,
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt: Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_revid"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_revid(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRevid {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_revid
    }
    #[doc = "0x04 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsScfg {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg
    }
    #[doc = "0x08 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_isr(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsIsr {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_isr
    }
    #[doc = "0x0c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_is"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_is(&self) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsIs {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_is
    }
    #[doc = "0x10 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ies"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_ies(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsIes {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_ies
    }
    #[doc = "0x14 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_iec"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_iec(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsIec {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_iec
    }
    #[doc = "0x18 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsCcfg {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg
    }
    #[doc = "0x1c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsCstatus {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus
    }
    #[doc = "0x20 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0
    }
    #[doc = "0x24 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst0"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst0 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0
    }
    #[doc = "0x28 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst0"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgst0 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0
    }
    #[doc = "0x2c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi0"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi0 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0
    }
    #[doc = "0x30 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye00"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye00 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00
    }
    #[doc = "0x34 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye01"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye01 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01
    }
    #[doc = "0x38 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye02"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye02 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02
    }
    #[doc = "0x3c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye03"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye03 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03
    }
    #[doc = "0x40 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye04"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye04 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04
    }
    #[doc = "0x44 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye05"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye05 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05
    }
    #[doc = "0x48 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye06"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye06 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06
    }
    #[doc = "0x4c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye07"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye07 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07
    }
    #[doc = "0x50 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep00"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep00 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00
    }
    #[doc = "0x54 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep01"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep01 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01
    }
    #[doc = "0x58 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep02"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep02 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02
    }
    #[doc = "0x5c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep03"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep03 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03
    }
    #[doc = "0x60 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep04"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep04 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04
    }
    #[doc = "0x64 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep05"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep05 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05
    }
    #[doc = "0x68 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep06"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep06 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06
    }
    #[doc = "0x6c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep07"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep07 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07
    }
    #[doc = "0x70 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya00"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya00 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00
    }
    #[doc = "0x74 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya01"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya01 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01
    }
    #[doc = "0x78 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya02"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya02 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02
    }
    #[doc = "0x7c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya03"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya03 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03
    }
    #[doc = "0x80 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap00"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap00 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00
    }
    #[doc = "0x84 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01
    }
    #[doc = "0x88 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap02"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap02 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02
    }
    #[doc = "0x8c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap03"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap03 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03
    }
    #[doc = "0x90 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv00"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv00 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00
    }
    #[doc = "0x94 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01
    }
    #[doc = "0x98 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv02"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv02 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02
    }
    #[doc = "0x9c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv03"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv03 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03
    }
    #[doc = "0xa0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1
    }
    #[doc = "0xa4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst1"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst1 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1
    }
    #[doc = "0xa8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst1"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgst1 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1
    }
    #[doc = "0xac - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi1"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi1 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1
    }
    #[doc = "0xb0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye10"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye10 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10
    }
    #[doc = "0xb4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye11"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye11 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11
    }
    #[doc = "0xb8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye12"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye12 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12
    }
    #[doc = "0xbc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13
    }
    #[doc = "0xc0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye14"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye14 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14
    }
    #[doc = "0xc4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye15"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye15 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15
    }
    #[doc = "0xc8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye16"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye16 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16
    }
    #[doc = "0xcc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye17"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye17 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17
    }
    #[doc = "0xd0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep10"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep10 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10
    }
    #[doc = "0xd4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep11"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep11 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11
    }
    #[doc = "0xd8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep12"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep12 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12
    }
    #[doc = "0xdc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep13"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep13 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13
    }
    #[doc = "0xe0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep14"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep14 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14
    }
    #[doc = "0xe4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep15"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep15 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15
    }
    #[doc = "0xe8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep16"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep16 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16
    }
    #[doc = "0xec - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep17"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep17 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17
    }
    #[doc = "0xf0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya10"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya10 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10
    }
    #[doc = "0xf4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya11"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya11 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11
    }
    #[doc = "0xf8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya12"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya12 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12
    }
    #[doc = "0xfc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya13"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya13 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13
    }
    #[doc = "0x100 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap10"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap10 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10
    }
    #[doc = "0x104 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap11"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap11 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11
    }
    #[doc = "0x108 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap12"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap12 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12
    }
    #[doc = "0x10c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap13"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap13 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13
    }
    #[doc = "0x110 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv10"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv10 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10
    }
    #[doc = "0x114 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv11"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv11 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11
    }
    #[doc = "0x118 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv12"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv12 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12
    }
    #[doc = "0x11c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv13"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv13 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13
    }
    #[doc = "0x120 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg2"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg2 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2
    }
    #[doc = "0x124 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst2"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst2 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2
    }
    #[doc = "0x128 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2
    }
    #[doc = "0x12c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi2"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi2 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2
    }
    #[doc = "0x130 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye20"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye20 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20
    }
    #[doc = "0x134 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye21"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye21 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21
    }
    #[doc = "0x138 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye22"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye22 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22
    }
    #[doc = "0x13c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye23"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye23 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23
    }
    #[doc = "0x140 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye24"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye24 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24
    }
    #[doc = "0x144 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye25"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye25 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25
    }
    #[doc = "0x148 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye26"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye26 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26
    }
    #[doc = "0x14c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye27"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye27 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27
    }
    #[doc = "0x150 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep20"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep20 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20
    }
    #[doc = "0x154 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep21"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep21 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21
    }
    #[doc = "0x158 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep22"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep22 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22
    }
    #[doc = "0x15c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep23"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep23 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23
    }
    #[doc = "0x160 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep24"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep24 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24
    }
    #[doc = "0x164 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep25"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep25 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25
    }
    #[doc = "0x168 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep26"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep26 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26
    }
    #[doc = "0x16c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep27"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep27 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27
    }
    #[doc = "0x170 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya20"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya20 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20
    }
    #[doc = "0x174 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya21"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya21 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21
    }
    #[doc = "0x178 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya22"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya22 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22
    }
    #[doc = "0x17c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya23"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya23 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23
    }
    #[doc = "0x180 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap20"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap20 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20
    }
    #[doc = "0x184 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap21"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap21 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21
    }
    #[doc = "0x188 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap22"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap22 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22
    }
    #[doc = "0x18c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap23"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap23 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23
    }
    #[doc = "0x190 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv20"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv20 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20
    }
    #[doc = "0x194 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv21"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv21 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21
    }
    #[doc = "0x198 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv22"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv22 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22
    }
    #[doc = "0x19c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23
    }
    #[doc = "0x1a0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3
    }
    #[doc = "0x1a4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst3"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst3 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3
    }
    #[doc = "0x1a8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst3"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgst3 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3
    }
    #[doc = "0x1ac - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi3"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi3 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3
    }
    #[doc = "0x1b0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye30"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye30 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30
    }
    #[doc = "0x1b4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye31"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye31 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31
    }
    #[doc = "0x1b8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye32"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye32 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32
    }
    #[doc = "0x1bc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye33"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye33 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33
    }
    #[doc = "0x1c0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye34"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye34 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34
    }
    #[doc = "0x1c4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye35"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye35 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35
    }
    #[doc = "0x1c8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye36"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye36 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36
    }
    #[doc = "0x1cc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye37"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye37 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37
    }
    #[doc = "0x1d0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep30"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep30 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30
    }
    #[doc = "0x1d4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep31"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep31 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31
    }
    #[doc = "0x1d8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep32"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep32 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32
    }
    #[doc = "0x1dc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep33"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep33 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33
    }
    #[doc = "0x1e0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep34"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep34 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34
    }
    #[doc = "0x1e4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep35"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep35 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35
    }
    #[doc = "0x1e8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep36"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep36 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36
    }
    #[doc = "0x1ec - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep37"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep37 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37
    }
    #[doc = "0x1f0 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya30"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya30 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30
    }
    #[doc = "0x1f4 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya31"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya31 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31
    }
    #[doc = "0x1f8 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya32"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya32 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32
    }
    #[doc = "0x1fc - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya33"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya33 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33
    }
    #[doc = "0x200 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap30"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap30 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30
    }
    #[doc = "0x204 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap31"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap31 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31
    }
    #[doc = "0x208 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap32"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap32 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32
    }
    #[doc = "0x20c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap33"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap33 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33
    }
    #[doc = "0x210 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv30"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv30 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30
    }
    #[doc = "0x214 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv31"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv31 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31
    }
    #[doc = "0x218 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv32"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv32 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32
    }
    #[doc = "0x21c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv33"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRiv33 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33
    }
    #[doc = "0x220 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0
    }
    #[doc = "0x224 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1 {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1
    }
    #[doc = "0x228 - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfo {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo
    }
    #[doc = "0x22c - FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt"]
    #[inline(always)]
    pub const fn fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt(
        &self,
    ) -> &Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcnt {
        &self.fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_revid (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_revid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_revid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_revid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_revid`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_revid")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRevid =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_revid::Fsas_FsasOtfaCfg_FsasOtfaRegsRevidSpec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_revid"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_revid;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsScfg =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg::Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_isr`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsIsr =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_isr::Fsas_FsasOtfaCfg_FsasOtfaRegsIsrSpec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_isr"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_isr;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_is (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_is\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_is`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_is")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsIs =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_is::Fsas_FsasOtfaCfg_FsasOtfaRegsIsSpec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_is"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_is;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ies (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ies\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ies::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ies::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_ies`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ies")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsIes =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_ies::Fsas_FsasOtfaCfg_FsasOtfaRegsIesSpec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ies"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_ies;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_iec (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_iec\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_iec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_iec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_iec`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_iec")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsIec =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_iec::Fsas_FsasOtfaCfg_FsasOtfaRegsIecSpec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_iec"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_iec;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsCcfg =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg::Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsCstatus = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus::Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0::Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg0Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg0"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg0;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst0 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst0")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst0 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0::Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst0Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst0"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst0;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst0 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst0")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgst0 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0::Fsas_FsasOtfaCfg_FsasOtfaRegsRgst0Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst0"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst0;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi0 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi0")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi0 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0::Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi0Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi0"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi0;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye00 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye00")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye00 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye00Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye00"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye00;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye01 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye01")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye01 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye01Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye01"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye01;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye02 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye02")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye02 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye02Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye02"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye02;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye03 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye03")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye03 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye03Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye03"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye03;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye04 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye04")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye04 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye04Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye04"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye04;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye05 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye05\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye05")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye05 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye05Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye05"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye05;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye06 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye06\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye06")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye06 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye06Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye06"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye06;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye07 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye07\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye07")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye07 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye07Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye07"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye07;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep00 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep00")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep00 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep00Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep00"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep00;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep01 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep01")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep01 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep01Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep01"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep01;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep02 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep02")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep02 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep02Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep02"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep02;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep03 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep03")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep03 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep03Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep03"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep03;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep04 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep04")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep04 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep04Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep04"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep04;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep05 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep05\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep05")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep05 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep05Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep05"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep05;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep06 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep06\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep06")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep06 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep06Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep06"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep06;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep07 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep07\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep07")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep07 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep07Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep07"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep07;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya00 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya00")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya00 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya00Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya00"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya00;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya01 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya01")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya01 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya01Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya01"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya01;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya02 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya02")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya02 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya02Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya02"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya02;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya03 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya03")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya03 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya03Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya03"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya03;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap00 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap00")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap00 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap00Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap00"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap00;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap02 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap02")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap02 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap02Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap02"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap02;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap03 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap03")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap03 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap03Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap03"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap03;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv00 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv00")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv00 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv00Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv00"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv00;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv02 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv02")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv02 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv02Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv02"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv02;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv03 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv03")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv03 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv03Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv03"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv03;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1::Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg1Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg1"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg1;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst1 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst1")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst1 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1::Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst1Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst1"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst1;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst1 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst1")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgst1 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1::Fsas_FsasOtfaCfg_FsasOtfaRegsRgst1Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst1"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst1;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi1 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi1")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi1 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1::Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi1Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi1"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi1;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye10 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye10")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye10 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye10Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye10"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye10;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye11 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye11")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye11 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye11Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye11"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye11;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye12 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye12")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye12 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye12Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye12"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye12;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye14 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye14")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye14 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye14Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye14"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye14;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye15 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye15")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye15 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye15Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye15"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye15;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye16 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye16")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye16 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye16Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye16"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye16;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye17 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye17")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye17 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye17Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye17"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye17;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep10 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep10")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep10 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep10Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep10"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep10;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep11 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep11")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep11 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep11Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep11"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep11;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep12 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep12")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep12 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep12Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep12"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep12;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep13 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep13")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep13 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep13Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep13"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep13;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep14 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep14")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep14 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep14Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep14"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep14;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep15 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep15")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep15 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep15Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep15"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep15;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep16 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep16")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep16 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep16Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep16"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep16;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep17 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep17")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep17 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep17Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep17"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep17;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya10 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya10")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya10 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya10Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya10"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya10;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya11 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya11")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya11 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya11Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya11"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya11;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya12 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya12")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya12 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya12Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya12"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya12;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya13 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya13")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya13 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya13Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya13"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya13;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap10 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap10")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap10 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap10Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap10"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap10;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap11 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap11")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap11 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap11Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap11"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap11;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap12 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap12")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap12 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap12Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap12"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap12;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap13 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap13")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap13 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap13Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap13"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap13;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv10 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv10")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv10 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv10Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv10"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv10;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv11 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv11")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv11 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv11Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv11"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv11;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv12 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv12")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv12 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv12Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv12"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv12;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv13 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv13")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv13 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv13Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv13"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv13;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg2 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg2")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg2 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2::Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg2Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg2"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg2;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst2 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst2")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst2 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2::Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst2Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst2"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst2;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2::Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi2 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi2")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi2 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2::Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi2Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi2"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi2;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye20 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye20")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye20 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye20Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye20"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye20;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye21 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye21")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye21 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye21Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye21"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye21;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye22 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye22")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye22 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye22Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye22"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye22;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye23 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye23")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye23 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye23Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye23"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye23;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye24 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye24")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye24 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye24Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye24"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye24;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye25 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye25")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye25 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye25Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye25"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye25;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye26 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye26")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye26 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye26Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye26"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye26;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye27 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye27")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye27 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye27Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye27"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye27;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep20 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep20")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep20 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep20Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep20"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep20;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep21 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep21")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep21 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep21Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep21"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep21;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep22 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep22")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep22 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep22Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep22"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep22;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep23 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep23")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep23 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep23Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep23"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep23;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep24 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep24")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep24 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep24Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep24"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep24;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep25 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep25")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep25 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep25Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep25"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep25;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep26 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep26")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep26 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep26Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep26"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep26;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep27 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep27")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep27 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep27Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep27"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep27;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya20 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya20")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya20 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya20Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya20"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya20;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya21 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya21")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya21 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya21Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya21"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya21;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya22 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya22")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya22 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya22Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya22"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya22;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya23 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya23")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya23 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya23Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya23"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya23;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap20 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap20")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap20 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap20Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap20"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap20;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap21 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap21")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap21 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap21Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap21"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap21;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap22 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap22")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap22 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap22Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap22"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap22;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap23 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap23")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap23 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap23Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap23"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap23;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv20 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv20")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv20 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv20Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv20"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv20;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv21 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv21")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv21 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv21Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv21"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv21;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv22 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv22")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv22 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv22Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv22"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv22;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3::Fsas_FsasOtfaCfg_FsasOtfaRegsRgcfg3Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgcfg3"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgcfg3;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst3 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst3")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst3 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3::Fsas_FsasOtfaCfg_FsasOtfaRegsRgmacst3Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgmacst3"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgmacst3;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst3 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst3")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgst3 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3::Fsas_FsasOtfaCfg_FsasOtfaRegsRgst3Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst3"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst3;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi3 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi3")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi3 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3::Fsas_FsasOtfaCfg_FsasOtfaRegsRgsi3Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgsi3"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rgsi3;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye30 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye30")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye30 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye30Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye30"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye30;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye31 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye31")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye31 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye31Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye31"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye31;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye32 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye32")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye32 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye32Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye32"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye32;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye33 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye33")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye33 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye33Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye33"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye33;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye34 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye34")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye34 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye34Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye34"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye34;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye35 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye35")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye35 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye35Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye35"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye35;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye36 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye36")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye36 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye36Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye36"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye36;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye37 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye37")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye37 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye37Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye37"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye37;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep30 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep30")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep30 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep30Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep30"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep30;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep31 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep31")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep31 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep31Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep31"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep31;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep32 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep32")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep32 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep32Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep32"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep32;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep33 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep33")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep33 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep33Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep33"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep33;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep34 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep34")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep34 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep34Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep34"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep34;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep35 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep35")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep35 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep35Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep35"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep35;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep36 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep36")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep36 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep36Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep36"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep36;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep37 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep37")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep37 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyep37Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyep37"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyep37;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya30 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya30")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya30 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya30Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya30"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya30;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya31 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya31")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya31 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya31Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya31"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya31;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya32 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya32")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya32 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya32Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya32"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya32;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya33 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya33")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya33 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeya33Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeya33"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeya33;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap30 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap30")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap30 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap30Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap30"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap30;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap31 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap31")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap31 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap31Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap31"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap31;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap32 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap32")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap32 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap32Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap32"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap32;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap33 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap33")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap33 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33::Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap33Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap33"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap33;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv30 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv30")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv30 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv30Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv30"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv30;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv31 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv31")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv31 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv31Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv31"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv31;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv32 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv32")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv32 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv32Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv32"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv32;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv33 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv33")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRiv33 =
    crate::Reg<fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33::Fsas_FsasOtfaCfg_FsasOtfaRegsRiv33Spec>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv33"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_riv33;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0::Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1 (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1 = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1::Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfo = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo::Fsas_FsasOtfaCfg_FsasOtfaRegsMaccacheinfoSpec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_maccacheinfo"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_maccacheinfo;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt (rw) register accessor: FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt`]
module"]
#[doc(alias = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt")]
pub type Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcnt = crate::Reg<
    fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt::Fsas_FsasOtfaCfg_FsasOtfaRegsRmwrmcntSpec,
>;
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rmwrmcnt"]
pub mod fsas__fsas_otfa_cfg__fsas_otfa_regs_rmwrmcnt;
